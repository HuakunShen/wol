//! Wake-on-LAN magic packets.
//!
//! Two layers, kept apart on purpose:
//!
//! - [`MacAddress`] and [`magic_packet`] are pure bytes. No I/O, no panics, no
//!   dependencies.
//! - [`send_from`] sends one datagram, with the **source** address chosen by
//!   the caller.
//!
//! *Which* interfaces to send from, which broadcast addresses to aim at, how
//! many packets, and on which ports, are policy the caller owns. Enumerating
//! interfaces needs a platform crate, and this one has no dependencies.
//!
//! The MAC formats accepted here match `wol-go`'s: twelve bare hex digits, or
//! six pairs separated by `:`, `-`, or `.`.

use std::error::Error;
use std::fmt;
use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;

/// Six 0xFF bytes, then the target MAC sixteen times.
pub const MAGIC_PACKET_LEN: usize = 102;

/// A magic packet carrying a SecureOn password: [`MAGIC_PACKET_LEN`] plus the
/// six password bytes.
pub const MAGIC_PACKET_LEN_WITH_PASSWORD: usize = 108;

/// The two ports Wake-on-LAN is conventionally sent to. Neither is registered
/// for it — 9 is discard and 7 is echo — and hardware differs in which it
/// listens on, so a sender that wants to be heard sends to both.
pub const DISCARD_PORT: u16 = 9;
/// See [`DISCARD_PORT`].
pub const ECHO_PORT: u16 = 7;

/// A six-byte hardware address.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MacAddress([u8; 6]);

impl MacAddress {
    pub const fn from_bytes(bytes: [u8; 6]) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; 6] {
        &self.0
    }

    /// Whether every byte is zero. An interface that reports this has no usable
    /// address, and a caller should skip it rather than broadcast it.
    pub fn is_zero(&self) -> bool {
        self.0 == [0u8; 6]
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c, d, e, g] = self.0;
        write!(f, "{a:02x}:{b:02x}:{c:02x}:{d:02x}:{e:02x}:{g:02x}")
    }
}

/// Why a string is not a MAC address.
///
/// A distinct type rather than a string, because the previous version of this
/// crate reached `unwrap()` on non-hex input and **panicked**. A library that
/// panics on user input cannot be called from a daemon.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MacParseError {
    /// Not twelve hex digits, and not six separated pairs.
    Length,
    /// The separators are not all the same character, or are not one of
    /// `:`, `-`, `.`.
    Separator,
    /// A character that is not a hex digit.
    NotHex,
}

impl fmt::Display for MacParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reason = match self {
            Self::Length => "a MAC address is twelve hex digits, or six separated pairs",
            Self::Separator => "separators must all be the same one of ':', '-', '.'",
            Self::NotHex => "contains a character that is not a hex digit",
        };
        f.write_str(reason)
    }
}

impl Error for MacParseError {}

impl FromStr for MacAddress {
    type Err = MacParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let digits: String = match text.len() {
            12 => text.to_string(),
            17 => {
                // Six pairs and five separators. Every separator must be the
                // same character, so `aa:bb-cc.dd:ee:ff` is refused rather than
                // quietly accepted.
                let separator = text.as_bytes()[2];
                if !matches!(separator, b':' | b'-' | b'.') {
                    return Err(MacParseError::Separator);
                }
                let mut digits = String::with_capacity(12);
                for (index, byte) in text.bytes().enumerate() {
                    if index % 3 == 2 {
                        if byte != separator {
                            return Err(MacParseError::Separator);
                        }
                    } else {
                        digits.push(byte as char);
                    }
                }
                digits
            }
            _ => return Err(MacParseError::Length),
        };

        let mut bytes = [0u8; 6];
        for (slot, pair) in bytes.iter_mut().zip(digits.as_bytes().chunks(2)) {
            let pair = std::str::from_utf8(pair).map_err(|_| MacParseError::NotHex)?;
            *slot = u8::from_str_radix(pair, 16).map_err(|_| MacParseError::NotHex)?;
        }
        Ok(MacAddress(bytes))
    }
}

/// Six 0xFF bytes followed by the MAC repeated sixteen times.
pub fn magic_packet(mac: MacAddress) -> [u8; MAGIC_PACKET_LEN] {
    let mut packet = [0xFFu8; MAGIC_PACKET_LEN];
    for chunk in packet[6..].chunks_mut(6) {
        chunk.copy_from_slice(mac.as_bytes());
    }
    packet
}

/// [`magic_packet`] with a six-byte SecureOn password appended.
///
/// NICs configured to require a password ignore a packet without one. Nothing
/// here validates the password — a wrong one is indistinguishable from an
/// unreachable target, which is why a caller should surface it as a
/// configuration fact rather than as a send failure.
pub fn magic_packet_with_password(
    mac: MacAddress,
    password: [u8; 6],
) -> [u8; MAGIC_PACKET_LEN_WITH_PASSWORD] {
    let mut packet = [0xFFu8; MAGIC_PACKET_LEN_WITH_PASSWORD];
    packet[..MAGIC_PACKET_LEN].copy_from_slice(&magic_packet(mac));
    packet[MAGIC_PACKET_LEN..].copy_from_slice(&password);
    packet
}

/// Send one packet from an explicit local address to an explicit target.
///
/// **The bind address is the argument that matters.** Binding `0.0.0.0` lets
/// the kernel pick the outgoing interface, and on a machine with more than one
/// — a laptop on Wi-Fi with a dock plugged in, any host with a VPN or a
/// container bridge — the one it picks is routinely not the one the target is
/// on. The packet is then sent successfully and arrives nowhere. Pass the
/// address of the interface whose broadcast domain the target sits in.
pub fn send_from(packet: &[u8], bind: SocketAddr, target: SocketAddr) -> io::Result<()> {
    let socket = UdpSocket::bind(bind)?;
    socket.set_broadcast(true)?;
    let sent = socket.send_to(packet, target)?;
    if sent != packet.len() {
        return Err(io::Error::new(
            io::ErrorKind::WriteZero,
            format!("sent {sent} of {} bytes", packet.len()),
        ));
    }
    Ok(())
}

/// Build a magic packet for `mac_address`.
///
/// Kept for callers written against the first version of this crate. It now
/// returns an error where it used to panic. Prefer [`magic_packet`].
pub fn create_magic_packet(
    mac_address: String,
) -> Result<[u8; MAGIC_PACKET_LEN], Box<dyn Error>> {
    Ok(magic_packet(mac_address.parse::<MacAddress>()?))
}

/// Send one magic packet, letting the OS choose the outgoing interface.
///
/// Kept for callers written against the first version of this crate. On a
/// multi-homed host this can send from the wrong interface and reach nothing —
/// see [`send_from`], which is what a caller that cares should use.
pub fn wakeonlan(mac_address: String, ip: String, port: u16) -> Result<(), Box<dyn Error>> {
    let packet = magic_packet(mac_address.parse::<MacAddress>()?);
    let target: SocketAddr = format!("{ip}:{port}").parse()?;
    let bind: SocketAddr = match target {
        SocketAddr::V4(_) => "0.0.0.0:0".parse()?,
        SocketAddr::V6(_) => "[::]:0".parse()?,
    };
    send_from(&packet, bind, target)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const AABB: MacAddress = MacAddress::from_bytes([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);

    #[test]
    fn verify_magic_packet() {
        let packet = create_magic_packet("aa:bb:cc:dd:ee:ff".to_string()).unwrap();
        assert_eq!(packet.len(), 102);
        assert_eq!(packet[..6], [0xFF; 6]);
        assert_eq!(packet[6..102], [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF].repeat(16));
    }

    #[test]
    fn verify_magic_packet_invalid() {
        assert!(create_magic_packet("aa:bb:cc:dd:ee:f".to_string()).is_err());
    }

    #[test]
    fn every_separator_wol_go_accepts_parses_to_the_same_address() {
        for text in ["aabbccddeeff", "aa:bb:cc:dd:ee:ff", "aa-bb-cc-dd-ee-ff", "aa.bb.cc.dd.ee.ff"] {
            assert_eq!(text.parse::<MacAddress>().unwrap(), AABB, "{text}");
        }
        assert_eq!("AA:BB:CC:DD:EE:FF".parse::<MacAddress>().unwrap(), AABB);
    }

    #[test]
    fn mixed_separators_are_refused_rather_than_stripped() {
        assert_eq!(
            "aa:bb-cc.dd:ee:ff".parse::<MacAddress>(),
            Err(MacParseError::Separator)
        );
    }

    /// The defect this crate shipped with: `from_str_radix(..).unwrap()` on a
    /// non-hex pair aborted the process. Nothing calling a library to build a
    /// packet should be able to be killed by the string it was handed.
    #[test]
    fn non_hex_input_is_an_error_and_not_a_panic() {
        for text in ["zz:bb:cc:dd:ee:ff", "gggggggggggg", "aa:bb:cc:dd:ee:gg"] {
            assert_eq!(text.parse::<MacAddress>(), Err(MacParseError::NotHex), "{text}");
        }
    }

    #[test]
    fn wrong_length_is_an_error() {
        for text in ["", "aabbccddeef", "aabbccddeeff00"] {
            assert_eq!(text.parse::<MacAddress>(), Err(MacParseError::Length), "{text}");
        }
    }

    #[test]
    fn a_password_packet_is_the_plain_packet_plus_six_bytes() {
        let password = [1, 2, 3, 4, 5, 6];
        let packet = magic_packet_with_password(AABB, password);
        assert_eq!(packet.len(), 108);
        assert_eq!(packet[..102], magic_packet(AABB));
        assert_eq!(packet[102..], password);
    }

    #[test]
    fn display_round_trips() {
        assert_eq!(AABB.to_string(), "aa:bb:cc:dd:ee:ff");
        assert_eq!(AABB.to_string().parse::<MacAddress>().unwrap(), AABB);
    }

    #[test]
    fn an_all_zero_address_says_so() {
        assert!(MacAddress::from_bytes([0; 6]).is_zero());
        assert!(!AABB.is_zero());
    }
}
