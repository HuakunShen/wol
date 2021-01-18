package wol_go

import (
	"encoding/hex"
	"fmt"
	"net"
	"regexp"
)

func CreateMagicPacket(mac string) []byte {
	mac_bytes, err := hex.DecodeString(mac)
	if err != nil {
		panic(err)
	}
	packet := make([]byte, 0, 102)
	packet = append(packet, []byte{255, 255, 255, 255, 255, 255}...)
	for i := 0; i < 16; i++ {
		packet = append(packet, mac_bytes...)
	}
	return packet
}

func WakeOnLan(mac, ip, port string) error {
	if len(mac) == 12 {
		// regular case
	} else if len(mac) == 17 {
		matched, err := regexp.MatchString("^([0-9A-Fa-f]{2}[\\.:-]){5}([0-9A-Fa-f]{2})$", mac)
		if err != nil {
			fmt.Println("Error matching mac address:", mac)
		} else if matched {
			newMac := ""
			for _, letter := range mac {
				if matched, err := regexp.MatchString("^[0-9A-Fa-f]$", string(letter)); matched == true && err == nil {
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
	magicPacket := CreateMagicPacket(mac)
	conn, err := net.Dial("udp", ip+":"+port)
	if err != nil {
		return fmt.Errorf("Error: connection failed: ", err)
	} else {
		fmt.Println("Connection Successful")
	}

	if n, err := conn.Write(magicPacket); err == nil && n != 102 {
		return fmt.Errorf("Error: magic packet sent was %d bytes (expected 102 bytes sent)", n)
	} else {
		fmt.Println("Magic packet sent successfully")
	}
	if err = conn.Close(); err != nil {
		return fmt.Errorf("Error: Close Connection Failed:", err)
	} else {
		fmt.Println("Connection Closed")
	}
	return nil
}
