/*
Copyright © 2025 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"
	"os"
	"strconv"

	wol "github.com/HuakunShen/wol/wol-go"
	"github.com/spf13/cobra"
)

var (
	port int
	ip   string
	mac  string
)

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:   "wol",
	Short: "WakeOnLan CLI",
	Long:  ``,
	Run: func(cmd *cobra.Command, args []string) {
		if mac == "" {
			fmt.Println("No argument received, at least give me a mac address")
			cmd.Help()
		} else {
			fmt.Println("IP:\t\t", ip)
			fmt.Println("Port:\t\t", port)
			fmt.Println("Mac Address:\t", mac)
			err := wol.WakeOnLan(mac, ip, strconv.Itoa(port))
			if err != nil {
				fmt.Println("Error:", err)
			} else {
				fmt.Println("Success")
			}
		}
	},
}

// Execute adds all child commands to the root command and sets flags appropriately.
// This is called by main.main(). It only needs to happen once to the rootCmd.
func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

func init() {
	// Here you will define your flags and configuration settings.
	// Cobra supports persistent flags, which, if defined here,
	// will be global for your application.
	rootCmd.PersistentFlags().IntVarP(&port, "port", "p", 9, "Port Number, Default: 9")
	rootCmd.PersistentFlags().StringVarP(&ip, "ip", "i", "255.255.255.255", "IP, Default: 255.255.255.255")
	rootCmd.Args = cobra.ExactArgs(1)
	rootCmd.PersistentPreRun = func(cmd *cobra.Command, args []string) {
		mac = args[0]
	}
}
