package process

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"

	"github.com/kamadorueda/tap-json/cli"
)

// IsBase return true if the provided value is a basic type
func IsBase(value *interface{}) bool {
	switch (*value).(type) {
	case bool:
		return true
	case float64:
		return true
	case string:
		return true
	default:
		return false
	}
}

func isArray(value *interface{}) bool {
	switch (*value).(type) {
	case []interface{}:
		return true
	default:
		return false
	}
}

func isMap(value *interface{}) bool {
	switch (*value).(type) {
	case map[string]interface{}:
		return true
	default:
		return false
	}
}

// Simplify data
func Simplify(value *interface{}) *interface{} {
	var newValue interface{}

	fmt.Println("Simplify:", *value)
	fmt.Println(fmt.Sprintf("    type: %T", *value))

	if IsBase(value) {
		newValue = *value
	} else if isMap(value) {
		var newValueTmp = make(map[string]interface{})
		for k, v := range (*value).(map[string]interface{}) {
			newValueTmp[k] = v
		}
		newValue = newValueTmp
	} else {
		panic(value)
	}

	fmt.Println("Simplified:", newValue)
	return &newValue
}

// Process take the input stream and outputs Singer records
func Process(config *cli.Config) bool {
	var success bool = true
	var inputData map[string]interface{}

	// Create a temporary directory where results will be stored
	// stagingArea, err := ioutil.TempDir("", "")
	// if err != nil {
	// 	panic(err)
	// }

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

		var record interface{}
		record = inputData["record"]

		// Do the magic denesting process
		Simplify(&record)

		fmt.Println(record)
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("[ERROR] While scanning os.Stdin:", err)
	}

	return success
}
