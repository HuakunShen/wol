use std::process::ExitCode;

use wol_rust::{DISCARD_PORT, wakeonlan};

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let (Some(mac), broadcast, port) = (args.next(), args.next(), args.next()) else {
        eprintln!("usage: wol-rust <mac> [broadcast-ip] [port]");
        eprintln!("       wol-rust aa:bb:cc:dd:ee:ff 255.255.255.255 9");
        return ExitCode::FAILURE;
    };
    let broadcast = broadcast.unwrap_or_else(|| "255.255.255.255".to_string());
    let port = match port {
        Some(text) => match text.parse() {
            Ok(port) => port,
            Err(error) => {
                eprintln!("bad port {text:?}: {error}");
                return ExitCode::FAILURE;
            }
        },
        None => DISCARD_PORT,
    };

    match wakeonlan(mac, broadcast, port) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
