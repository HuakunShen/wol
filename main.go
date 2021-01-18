package main

import (
	"flag"
	"fmt"
	wol "github.com/HuakunShen/wol/wol-go"
	"strconv"
)

func main() {
	port := flag.Int("port", 9, "Port Number, Default: 9")
	ip := flag.String("ip", "255.255.255.255", "ip, Default: 255.255.255.255")
	mac := flag.String("mac", "", "ip, Default: 255.255.255.255")
	flag.Parse()
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
