package wol_go

import (
	"encoding/hex"
	"fmt"
	"net"
	"regexp"
	"strconv"
	"strings"
	"syscall"
)

// macBare matches a MAC address written as 12 bare hexadecimal characters.
var macBare = regexp.MustCompile(`^[0-9A-Fa-f]{12}$`)

// macSeparated matches six hexadecimal pairs joined by ".", ":" or "-".
var macSeparated = regexp.MustCompile(`^([0-9A-Fa-f]{2}[.:-]){5}[0-9A-Fa-f]{2}$`)

// NormalizeMac reduces a MAC address to 12 bare hexadecimal characters.
//
// It accepts either 12 hex characters ("aabbccddeeff") or six hex pairs
// separated by ".", ":" or "-" ("aa:bb:cc:dd:ee:ff"), and rejects everything
// else. In particular separators are only allowed in the positions where they
// belong, so "aa1bb1cc1dd1ee1ff" is not silently reinterpreted.
func NormalizeMac(mac string) (string, error) {
	switch {
	case macBare.MatchString(mac):
		return mac, nil
	case macSeparated.MatchString(mac):
		normalized := make([]byte, 0, 12)
		for i := 0; i < len(mac); i++ {
			if i%3 != 2 {
				normalized = append(normalized, mac[i])
			}
		}
		return string(normalized), nil
	default:
		return "", fmt.Errorf(
			"invalid MAC address %q: expected 12 hex characters, optionally separated by '.', ':' or '-'",
			mac,
		)
	}
}

// CreateMagicPacket builds the 102-byte magic packet for a MAC address given as
// 12 bare hexadecimal characters: six 0xFF bytes followed by the address
// repeated 16 times.
func CreateMagicPacket(mac string) ([]byte, error) {
	if len(mac) != 12 {
		return nil, fmt.Errorf("mac has wrong length")
	}
	macRepeat := strings.Repeat(mac, 16) // 16 * 6 = 96 bytes
	macBytes, err := hex.DecodeString(macRepeat)
	if err != nil {
		return nil, err
	}
	packet := make([]byte, 0, 102) // 6+16*6=102 (mac address is 6 bytes too)
	packet = append(packet, []byte(strings.Repeat("\xFF", 6))...)
	packet = append(packet, macBytes...)
	return packet, nil
}

// broadcastDialer dials with SO_BROADCAST enabled. Without it, sending to a
// broadcast address such as 255.255.255.255 is refused with EACCES on Linux.
var broadcastDialer = net.Dialer{
	Control: func(network, address string, c syscall.RawConn) error {
		var sockErr error
		if err := c.Control(func(fd uintptr) {
			sockErr = syscall.SetsockoptInt(int(fd), syscall.SOL_SOCKET, syscall.SO_BROADCAST, 1)
		}); err != nil {
			return err
		}
		return sockErr
	},
}

// WakeOnLan sends a Wake-on-LAN magic packet for mac to ip:port.
func WakeOnLan(mac, ip, port string) error {
	normalized, err := NormalizeMac(mac)
	if err != nil {
		return err
	}

	portNum, err := strconv.Atoi(port)
	if err != nil || portNum < 1 || portNum > 65535 {
		return fmt.Errorf("invalid port %q: expected an integer between 1 and 65535", port)
	}

	magicPacket, err := CreateMagicPacket(normalized)
	if err != nil {
		return err
	}

	conn, err := broadcastDialer.Dial("udp4", net.JoinHostPort(ip, port))
	if err != nil {
		return err
	}
	defer conn.Close()

	n, err := conn.Write(magicPacket)
	if err != nil {
		return err
	}
	if n != len(magicPacket) {
		return fmt.Errorf("magic packet sent was %d bytes (expected %d bytes sent)", n, len(magicPacket))
	}
	return nil
}
