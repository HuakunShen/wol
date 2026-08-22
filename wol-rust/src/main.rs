use std::process::ExitCode;

use wol_rust::wakeonlan;

const USAGE: &str = "usage: wol <mac> [broadcast-ip] [port]

  mac            MAC address, e.g. aa:bb:cc:dd:ee:ff or aabbccddeeff
  broadcast-ip   broadcast address to send to (default: 255.255.255.255)
  port           UDP port (default: 9)";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|a| a == "-h" || a == "--help") {
        println!("{USAGE}");
        return ExitCode::SUCCESS;
    }

    if args.is_empty() || args.len() > 3 {
        eprintln!("{USAGE}");
        return ExitCode::FAILURE;
    }

    let mac = args[0].clone();
    let ip = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "255.255.255.255".to_string());

    let port = match args.get(2) {
        Some(raw) => match raw.parse::<u16>() {
            Ok(port) if port != 0 => port,
            _ => {
                eprintln!("wol: invalid port {raw:?}: expected an integer between 1 and 65535");
                return ExitCode::FAILURE;
            }
        },
        None => 9,
    };

    match wakeonlan(mac, ip, port) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("wol: {err}");
            ExitCode::FAILURE
        }
    }
}
