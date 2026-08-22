use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

/// Normalize a MAC address to its six raw bytes.
///
/// Accepts either 12 hexadecimal characters (`"aabbccddeeff"`) or six hex pairs
/// separated by `.`, `:` or `-` (`"aa:bb:cc:dd:ee:ff"`).
pub fn parse_mac_address(mac_address: &str) -> Result<[u8; 6], Box<dyn std::error::Error>> {
    let bytes = mac_address.as_bytes();

    // Two accepted shapes, matching the JavaScript implementations exactly:
    // 12 bare hex characters, or six hex pairs each joined by '.', ':' or '-'.
    // Separators are only allowed in the positions where they belong, so
    // "::::aabbccddeeff" and "a-a-b-b-c-c-d-d-e-e-f-f" are rejected.
    let normalized: String = match bytes.len() {
        12 if bytes.iter().all(u8::is_ascii_hexdigit) => mac_address.to_string(),
        17
            if bytes.iter().enumerate().all(|(i, c)| {
                if i % 3 == 2 {
                    matches!(c, b'.' | b':' | b'-')
                } else {
                    c.is_ascii_hexdigit()
                }
            }) =>
        {
            bytes
                .iter()
                .enumerate()
                .filter(|(i, _)| i % 3 != 2)
                .map(|(_, c)| *c as char)
                .collect()
        }
        _ => {
            return Err(format!(
                "invalid MAC address {mac_address:?}: expected 12 hex characters, \
                 optionally separated by '.', ':' or '-'"
            )
            .into())
        }
    };

    // Both arms above guarantee 12 ASCII hex characters, so this slicing cannot
    // land inside a multi-byte sequence and the radix conversion cannot fail.
    let mut mac_bytes = [0u8; 6];
    for (i, byte) in mac_bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&normalized[i * 2..i * 2 + 2], 16)?;
    }
    Ok(mac_bytes)
}

/// Build a Wake-on-LAN magic packet: six 0xFF bytes followed by the MAC address
/// repeated 16 times, for a total of 102 bytes.
pub fn create_magic_packet(
    mac_address: String,
) -> Result<[u8; 102], Box<dyn std::error::Error>> {
    let mac_bytes = parse_mac_address(&mac_address)?;

    let mut magic_packet = [0u8; 102];
    magic_packet[..6].copy_from_slice(&[0xFF; 6]);
    for i in 0..16 {
        let start = 6 + i * 6;
        magic_packet[start..start + 6].copy_from_slice(&mac_bytes);
    }
    Ok(magic_packet)
}

/// Send a Wake-on-LAN magic packet to `ip:port`.
pub fn wakeonlan(
    mac_address: String,
    ip: String,
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let magic_packet = create_magic_packet(mac_address)?;

    if port == 0 {
        return Err("invalid port 0: expected an integer between 1 and 65535".into());
    }

    // Resolve rather than parse, so host names work as well as literal IPs.
    // Literal IPs are still parsed directly by std, without a DNS lookup.
    // The socket below is IPv4, so an IPv6 result would fail with a bare EINVAL.
    let addr: SocketAddr = (ip.as_str(), port)
        .to_socket_addrs()?
        .find(SocketAddr::is_ipv4)
        .ok_or_else(|| format!("could not resolve {ip:?} to an IPv4 address"))?;

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    socket.send_to(&magic_packet, addr)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_magic_packet() {
        let magic_packet = create_magic_packet("aa:bb:cc:dd:ee:ff".to_string()).unwrap();
        assert_eq!(magic_packet.len(), 102);
        // first 6 bytes should be 0xFF
        assert_eq!(magic_packet[..6], [0xFF; 6]);
        // next 96 bytes should be mac_address repeated 16 times
        assert_eq!(
            magic_packet[6..102],
            [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF].repeat(16)
        );
    }

    #[test]
    fn accepts_every_separator() {
        let expected = create_magic_packet("aabbccddeeff".to_string()).unwrap();
        for mac in ["aa:bb:cc:dd:ee:ff", "aa-bb-cc-dd-ee-ff", "aa.bb.cc.dd.ee.ff"] {
            assert_eq!(create_magic_packet(mac.to_string()).unwrap(), expected);
        }
    }

    #[test]
    fn verify_magic_packet_invalid() {
        let res = create_magic_packet("aa:bb:cc:dd:ee:f".to_string());
        assert!(res.is_err());
    }

    #[test]
    fn non_hex_returns_error_instead_of_panicking() {
        assert!(create_magic_packet("gg:bb:cc:dd:ee:ff".to_string()).is_err());
        assert!(create_magic_packet("zzzzzzzzzzzz".to_string()).is_err());
    }

    #[test]
    fn rejects_separators_in_the_wrong_places() {
        for mac in [
            "aabb.ccdd.eeff",
            "aa:bb:cc:dd:ee:ff:",
            "::::aabbccddeeff",
            "-.:aabbccddeeff:.-",
            "a-a-b-b-c-c-d-d-e-e-f-f",
            "aa\\bb\\cc\\dd\\ee\\ff",
        ] {
            assert!(
                create_magic_packet(mac.to_string()).is_err(),
                "expected {mac:?} to be rejected"
            );
        }
    }

    #[test]
    fn rejects_port_zero() {
        assert!(wakeonlan("aabbccddeeff".to_string(), "127.0.0.1".to_string(), 0).is_err());
    }

    #[test]
    fn non_ascii_returns_error_instead_of_panicking() {
        // 12 bytes of UTF-8, but not 12 characters: chunking by 2 bytes used to
        // split a multi-byte sequence and panic.
        assert!(create_magic_packet("αβγδεζ".to_string()).is_err());
    }
}
