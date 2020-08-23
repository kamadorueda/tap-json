package cli

import (
	"flag"
	"fmt"
)

// Parse CLI arguments from the user and return a struct
func Parse() {
	fs := flag.NewFlagSet("CLI", flag.ExitOnError)

	fmt.Println(fs)
}
