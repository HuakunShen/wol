import argparse
import re
import socket
import sys

# 12 bare hexadecimal characters.
MAC_BARE = re.compile(r"^[0-9A-Fa-f]{12}$")
# Six hexadecimal pairs joined by ".", ":" or "-".
MAC_SEPARATED = re.compile(r"^([0-9A-Fa-f]{2}[.:-]){5}[0-9A-Fa-f]{2}$")


def normalize_mac(macaddress: str) -> str:
    """Reduce a MAC address to 12 bare hexadecimal characters.

    Accepts either 12 hex characters or six hex pairs separated by ".", ":" or
    "-". Separators are only allowed where they belong, so "aa1bb1cc1dd1ee1ff"
    is rejected rather than silently reinterpreted as "aabbccddeeff".
    """
    if MAC_BARE.match(macaddress):
        return macaddress
    if MAC_SEPARATED.match(macaddress):
        return re.sub(r"[.:-]", "", macaddress)
    raise ValueError(
        "Invalid MAC address {!r}: expected 12 hex characters, "
        "optionally separated by '.', ':' or '-'".format(macaddress)
    )


def create_magic_packet(macaddress: str) -> bytes:
    """Build the 102-byte magic packet: six 0xFF bytes then the MAC 16 times."""
    payload = bytes.fromhex("FF" * 6 + normalize_mac(macaddress) * 16)
    if len(payload) != 102:
        raise ValueError(
            "Packet Byte Length Must be 102, instead, got {}".format(len(payload))
        )
    return payload


def wake_on_lan(macaddress: str, ip: str = "255.255.255.255", port: int = 9) -> None:
    """Send a Wake-on-LAN magic packet to ip:port."""
    if not isinstance(port, int) or not 1 <= port <= 65535:
        raise ValueError(
            "Invalid port {!r}: expected an integer between 1 and 65535".format(port)
        )
    packet = create_magic_packet(macaddress)
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as sock:
        sock.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)
        sock.sendto(packet, (ip, port))


def main() -> int:
    parser = argparse.ArgumentParser("Wake On Lan Parser")
    parser.add_argument("-p", "--port", default=9, type=int, help="port")
    parser.add_argument(
        "-i",
        "--ip",
        default="255.255.255.255",
        help="IP address, default: 255.255.255.255 broadcast",
    )
    parser.add_argument("mac", help="mac address")
    args = parser.parse_args()

    try:
        wake_on_lan(args.mac, args.ip, args.port)
    except (ValueError, OSError) as e:
        print("wol: {}".format(e), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
