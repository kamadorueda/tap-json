package main

import (
	"fmt"

	"github.com/kamadorueda/tap-json/cli"
	"github.com/kamadorueda/tap-json/process"
)

func main() {
	var config cli.Config = cli.Parse()
	var success bool = process.Process(&config)

	fmt.Println("[INFO] Success:", success)
}
