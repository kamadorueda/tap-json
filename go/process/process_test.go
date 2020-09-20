package process_test

import (
	"testing"

	"github.com/kamadorueda/tap-json/process"
)

func TestIsBase(t *testing.T) {
	testCases := []struct {
		value  interface{}
		result bool
	}{
		{value: true, result: true},
		{value: 12.3, result: true},
		{value: "tx", result: true},
		{value: make([]int, 0), result: false},
	}

	for _, testCase := range testCases {
		got := process.IsBase(&testCase.value)
		if got != testCase.result {
			t.Errorf("isBase(%v) = %v; want %v", testCase.value, got, testCase.result)
		}
	}
}
