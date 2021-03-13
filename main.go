package main

import (
	"flag"
	"fmt"
	"strconv"

	wol "github.com/HuakunShen/wol/wol-go"
)

func main() {
	port := flag.Int("port", 9, "Port Number, Default: 9")
	ip := flag.String("ip", "255.255.255.255", "ip, Default: 255.255.255.255")
	mac := flag.String("mac", "", "mac address of target machine (required)")
	flag.Parse()
	if flag.NFlag() == 0 {
		fmt.Println("No argument received, at least give me a mac address")
		flag.PrintDefaults()
	} else {
		fmt.Println("IP:\t\t", *ip)
		fmt.Println("Port:\t\t", *port)
		fmt.Println("Mac Address:\t", *mac)
		err := wol.WakeOnLan(*mac, *ip, strconv.Itoa(*port))
		if err != nil {
			fmt.Println("Error:", err)
		} else {
			fmt.Println("Success")
		}
	}

}
