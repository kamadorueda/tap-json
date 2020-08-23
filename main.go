package main

import (
	"fmt"

	"github.com/kamadorueda/tap-json/cli"
	"github.com/kamadorueda/tap-json/process"
)

func main() {
	cli.Parse()

	fmt.Println(process.Version)
}
