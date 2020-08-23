package process

import (
	"bufio"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"

	"github.com/kamadorueda/tap-json/cli"
)

// Linearize take an arbitrary datastructure and write results to staging area
func Linearize(stagingArea string, relation string, value interface{}) {
	fmt.Println("Linearize:", stagingArea, relation, value)
}

// Process take the input stream and outputs Singer records
func Process(config *cli.Config) bool {
	var success bool = true
	var inputData map[string]interface{}

	// Create a temporary directory where results will be stored
	stagingArea, err := ioutil.TempDir("", "")
	if err != nil {
		panic(err)
	}

	// Scan stdin for JSON lines
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		inputLine := scanner.Bytes()
		fmt.Println("inputLine:", string(inputLine))

		// Turn inputLine into inputData by loading it as a JSON
		err := json.Unmarshal(inputLine, &inputData)
		if err != nil {
			fmt.Println("[ERROR] While json.Unmarshal:", err)
			continue
		}

		// Do the magic denesting process
		Linearize(stagingArea, inputData["stream"].(string), inputData["record"])
	}
	if err := scanner.Err(); err != nil {
		fmt.Println("[ERROR] While scanning os.Stdin:", err)
	}

	return success
}
