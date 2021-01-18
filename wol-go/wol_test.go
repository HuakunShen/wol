package wol_go

import (
	"bytes"
	"testing"
)

func TestCreateMagicPacket(t *testing.T) {
	mac := "2d:ff:ac:7e:ca:6a"
	if _, err := CreateMagicPacket(mac); err == nil {
		t.Errorf("There should be an error for mac=%s", mac)
	}
	mac = "2cfda1715569"
	packet, err := CreateMagicPacket(mac)
	if err != nil {
		t.Errorf("There shouldn't be an error for mac=%s", mac)
	}
	if len(packet) != 102 {
		t.Errorf("Packet length must be 102, instead we got %d", len(packet))
	}
	if res := bytes.Compare(packet[:6], []byte{255, 255, 255, 255, 255, 255}); res != 0 {
		t.Errorf("Fisrt 6 bytes are wrong, should be [255, 255, 255, 255, 255, 255]")
	}
	firstMac := packet[6:12]
	for i := 1; i < 17; i++ {
		if res := bytes.Compare(firstMac, packet[i*6:(i+1)*6]); res != 0 {
			t.Errorf("the bytes after the first 6 bytes should be 16 repetition of the same mac address")
		}
	}
}
