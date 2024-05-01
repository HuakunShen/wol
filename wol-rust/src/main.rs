use wol_rust::wakeonlan;

fn main() {
    wakeonlan("74:56:3c:30:d4:3b".to_string(), "10.6.6.250".to_string(), 9).unwrap();
}
