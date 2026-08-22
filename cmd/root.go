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
)

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:   "wol <mac>",
	Short: "WakeOnLan CLI",
	Long:  ``,
	Args:  cobra.ExactArgs(1),
	// Usage is only helpful for argument errors, not for a failed send.
	SilenceUsage: true,
	// Execute prints the error itself.
	SilenceErrors: true,
	RunE: func(cmd *cobra.Command, args []string) error {
		return wol.WakeOnLan(args[0], ip, strconv.Itoa(port))
	},
}

// Execute adds all child commands to the root command and sets flags appropriately.
// This is called by main.main(). It only needs to happen once to the rootCmd.
func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, "wol:", err)
		os.Exit(1)
	}
}

func init() {
	// Here you will define your flags and configuration settings.
	// Cobra supports persistent flags, which, if defined here,
	// will be global for your application.
	rootCmd.PersistentFlags().IntVarP(&port, "port", "p", 9, "Port Number, Default: 9")
	rootCmd.PersistentFlags().StringVarP(&ip, "ip", "i", "255.255.255.255", "IP, Default: 255.255.255.255")
}
