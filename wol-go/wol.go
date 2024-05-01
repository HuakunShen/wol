package wol_go

import (
	"encoding/hex"
	"fmt"
	"net"
	"regexp"
	"strings"
)

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

func WakeOnLan(mac, ip, port string) error {
	if len(mac) == 12 {
		// regular case
	} else if len(mac) == 17 {
		matched, err := regexp.MatchString("^([0-9A-Fa-f]{2}[\\.:-]){5}([0-9A-Fa-f]{2})$", mac)
		if err != nil {
			return err
		} else if matched {
			newMac := ""
			macRegex, err := regexp.Compile("^[0-9A-Fa-f]$")
			if err != nil {
				return err
			}
			for _, letter := range mac {
				if matched := macRegex.MatchString(string(letter)); matched {
					newMac += string(letter)
				}
			}
			mac = newMac
		} else {
			return fmt.Errorf("mac address invalid format")
		}
	} else {
		return fmt.Errorf("mac address wrong length")
	}
	magicPacket, err := CreateMagicPacket(mac)
	if err != nil {
		return err
	}
	conn, err := net.Dial("udp", ip+":"+port)
	if err != nil {
		return err
	}
	if n, err := conn.Write(magicPacket); err != nil && n != 102 {
		return fmt.Errorf("magic packet sent was %d bytes (expected 102 bytes sent)", n)
	}
	if err = conn.Close(); err != nil {
		return err
	}
	return nil
}
