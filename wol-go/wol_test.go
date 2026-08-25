package wol_go

import (
	"bytes"
	"net"
	"strings"
	"testing"
	"time"
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

func TestNormalizeMac(t *testing.T) {
	for _, mac := range []string{
		"aabbccddeeff",
		"AABBCCDDEEFF",
		"aa:bb:cc:dd:ee:ff",
		"aa-bb-cc-dd-ee-ff",
		"aa.bb.cc.dd.ee.ff",
	} {
		got, err := NormalizeMac(mac)
		if err != nil {
			t.Errorf("NormalizeMac(%q) returned an error: %v", mac, err)
		}
		if want := "aabbccddeeff"; !strings.EqualFold(got, want) {
			t.Errorf("NormalizeMac(%q) = %q, want %q", mac, got, want)
		}
	}

	for _, mac := range []string{
		"",
		"aabbccddeef",
		"aabbccddeezz",
		"zzzzzzzzzzzz",
		// Used to be accepted: the old code took whatever sat at index 2 as the
		// separator and stripped every occurrence of it.
		"aa1bb1cc1dd1ee1ff",
		"a-a-b-b-c-c-d-d-e-e-f-f",
		"aa:bb:cc:dd:ee:ff:",
		`aa\bb\cc\dd\ee\ff`,
	} {
		if _, err := NormalizeMac(mac); err == nil {
			t.Errorf("NormalizeMac(%q) should have returned an error", mac)
		}
	}
}

func TestWakeOnLanRejectsBadPort(t *testing.T) {
	for _, port := range []string{"", "abc", "9abc", "0", "65536", "-1"} {
		if err := WakeOnLan("aabbccddeeff", "127.0.0.1", port); err == nil {
			t.Errorf("WakeOnLan with port %q should have returned an error", port)
		}
	}
}

func TestWakeOnLanSendsAWellFormedPacket(t *testing.T) {
	conn, err := net.ListenPacket("udp4", "127.0.0.1:0")
	if err != nil {
		t.Fatalf("could not listen: %v", err)
	}
	defer conn.Close()

	_, port, err := net.SplitHostPort(conn.LocalAddr().String())
	if err != nil {
		t.Fatalf("could not split address: %v", err)
	}

	if err := WakeOnLan("aa:bb:cc:dd:ee:ff", "127.0.0.1", port); err != nil {
		t.Fatalf("WakeOnLan returned an error: %v", err)
	}

	buf := make([]byte, 128)
	if err := conn.SetReadDeadline(time.Now().Add(2 * time.Second)); err != nil {
		t.Fatalf("could not set deadline: %v", err)
	}
	n, _, err := conn.ReadFrom(buf)
	if err != nil {
		t.Fatalf("no packet received: %v", err)
	}

	want, err := CreateMagicPacket("aabbccddeeff")
	if err != nil {
		t.Fatalf("could not build the expected packet: %v", err)
	}
	if !bytes.Equal(buf[:n], want) {
		t.Errorf("received packet does not match the expected magic packet")
	}
}
