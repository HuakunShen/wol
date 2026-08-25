use std::process::ExitCode;

use wol_rust::{wakeonlan, DISCARD_PORT};

const USAGE: &str = "usage: wol-rust <mac> [broadcast-ip] [port]

  mac            MAC address, e.g. aa:bb:cc:dd:ee:ff or aabbccddeeff
  broadcast-ip   broadcast address or host name to send to
                 (default: 255.255.255.255)
  port           UDP port between 1 and 65535 (default: 9)

  wol-rust aa:bb:cc:dd:ee:ff 255.255.255.255 9";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        println!("{USAGE}");
        return ExitCode::SUCCESS;
    }

    // An extra argument is a mistake worth naming: silently ignoring it is how
    // a caller believes it passed a port it did not.
    if args.is_empty() || args.len() > 3 {
        eprintln!("{USAGE}");
        return ExitCode::FAILURE;
    }

    let mac = args[0].clone();
    let broadcast = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "255.255.255.255".to_string());

    let port = match args.get(2) {
        Some(text) => match text.parse::<u16>() {
            Ok(port) if port != 0 => port,
            _ => {
                eprintln!("wol: invalid port {text:?}: expected an integer between 1 and 65535");
                return ExitCode::FAILURE;
            }
        },
        None => DISCARD_PORT,
    };

    match wakeonlan(mac, broadcast, port) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("wol: {error}");
            ExitCode::FAILURE
        }
    }
}
