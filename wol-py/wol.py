import socket
import argparse
import time


def create_magic_packet(macaddress: str):
    if len(macaddress) == 17:
        sep = macaddress[2]
        macaddress = macaddress.replace(sep, "")
    elif len(macaddress) != 12:
        raise ValueError("Incorrect MAC address format")
    payload = "F" * 12 + macaddress * 16
    payload = bytes.fromhex(payload)
    return payload


if __name__ == "__main__":
    parser = argparse.ArgumentParser("Wake On Lan Parser")
    parser.add_argument('-p', '--port', default=9, type=int, help="port")
    parser.add_argument(
        '-i', '--ip', default='255.255.255.255', help="IP address, default: 255.255.255.255 broadcast")
    parser.add_argument('mac', help="mac address")
    args = parser.parse_args()

    for key in args.__dict__:
        print("{}: {}".format(key, args.__dict__[key]))
    try:
        with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as sock:
            packet = create_magic_packet(args.mac)
            if len(packet) != 102:
                raise ValueError(
                    "Packet Byte Length Must be 102, instead, got {}".format(len(packet)))
            sock.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)
            sock.sendto(packet, (args.ip, args.port))
    except socket.gaierror as e:
        print(e)
        print("Conection failed")
    except ValueError as e:
        print(e)
    print("done")
