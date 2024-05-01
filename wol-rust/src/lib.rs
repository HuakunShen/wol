use std::net::{UdpSocket, SocketAddr};

pub fn create_magic_packet(mac_address: String) -> Result<[u8; 102], Box<dyn std::error::Error>> {
    // checkif mac_address contains colon
    let mac_address = match mac_address.contains(':') {
        true => mac_address.replace(":", ""),
        false => mac_address,
    };
    if mac_address.len() != 12 {
        return Err("Invalid MAC address".into());
    }

    let mut magic_packet = [0u8; 102];
    magic_packet[..6].copy_from_slice(&[0xFF; 6]);
    // convert mac_address to bytes
    let mac_bytes = mac_address
        .as_bytes()
        .chunks(2)
        .map(|chunk| {
            u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap()
            // hex str to u8
        })
        .collect::<Vec<u8>>();
    magic_packet[6..102].copy_from_slice(&mac_bytes.repeat(16));
    Ok(magic_packet)
}

pub fn wakeonlan(mac_address: String, ip: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let magic_packet = create_magic_packet(mac_address)?;

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;

    let addr: SocketAddr = format!("{}:{}", ip, port).parse()?;
    socket.send_to(&magic_packet, addr)?;
    drop(socket);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_magic_packet() {
        let mac_address = "aa:bb:cc:dd:ee:ff".to_string();
        let magic_packet = create_magic_packet(mac_address).unwrap();
        assert_eq!(magic_packet.len(), 102);
        // first 6 bytes should be 0xFF
        assert_eq!(magic_packet[..6], [0xFF; 6]);
        // next 96 bytes should be mac_address repeated 16 times
        assert_eq!(magic_packet[6..102], [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF].repeat(16));
    }

    #[test]
    fn verify_magic_packet_invalid() {
        let mac_address = "aa:bb:cc:dd:ee:f".to_string();
        let res = create_magic_packet(mac_address);
        assert!(res.is_err());
    }
}
