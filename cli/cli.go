package cli

import (
	"flag"
	"strings"
)

// Config structure
type Config struct {
	datesFormats []string
}

// Parse CLI arguments from the user and return a struct
func Parse() Config {
	var dates = flag.String(
		"dates",
		"",
		"Comma separated string of date formats",
	)

	flag.Parse()

	var datesFormats = strings.Split(*dates, ",")
	datesFormats = append(datesFormats, "%Y-%m-%dT%H:%M:%S%z")
	datesFormats = append(datesFormats, "%Y-%m-%dT%H:%M:%S.%f%z")

	return Config{
		datesFormats: datesFormats,
	}
}
