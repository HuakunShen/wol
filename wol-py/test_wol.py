import socket
import unittest

from wol import create_magic_packet, normalize_mac, wake_on_lan


class NormalizeMacTest(unittest.TestCase):
    def test_accepts_bare_and_separated(self):
        for mac in [
            "aabbccddeeff",
            "AABBCCDDEEFF",
            "aa:bb:cc:dd:ee:ff",
            "aa-bb-cc-dd-ee-ff",
            "aa.bb.cc.dd.ee.ff",
        ]:
            self.assertEqual(normalize_mac(mac).lower(), "aabbccddeeff")

    def test_rejects_bad_input(self):
        for mac in [
            "",
            "aabbccddeef",
            "aabbccddeezz",
            "zzzzzzzzzzzz",
            # Used to be accepted: the old code took whatever sat at index 2 as
            # the separator and stripped every occurrence of it.
            "aa1bb1cc1dd1ee1ff",
            "a-a-b-b-c-c-d-d-e-e-f-f",
            "aa:bb:cc:dd:ee:ff:",
            "aa\\bb\\cc\\dd\\ee\\ff",
        ]:
            with self.assertRaises(ValueError, msg=mac):
                normalize_mac(mac)


class CreateMagicPacketTest(unittest.TestCase):
    def test_layout(self):
        packet = create_magic_packet("aa:bb:cc:dd:ee:ff")
        self.assertEqual(len(packet), 102)
        self.assertEqual(packet[:6], b"\xff" * 6)
        self.assertEqual(packet[6:].hex(), "aabbccddeeff" * 16)


class WakeOnLanTest(unittest.TestCase):
    def test_sends_a_well_formed_packet(self):
        with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as server:
            server.bind(("127.0.0.1", 0))
            server.settimeout(2)
            port = server.getsockname()[1]

            wake_on_lan("aa:bb:cc:dd:ee:ff", "127.0.0.1", port)

            data, _ = server.recvfrom(128)
            self.assertEqual(data, create_magic_packet("aabbccddeeff"))

    def test_rejects_bad_port(self):
        for port in [0, 65536, -1, "9"]:
            with self.assertRaises(ValueError, msg=repr(port)):
                wake_on_lan("aabbccddeeff", "127.0.0.1", port)


if __name__ == "__main__":
    unittest.main()
