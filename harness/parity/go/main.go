package main

import (
	"errors"
	"fmt"
	"os"

	memless "github.com/nascent-tech/memless/bindings/go"
)

const pathIndex = 1

func main() {
	fmt.Println(outcome(argPath()))
}

func argPath() string {
	if len(os.Args) <= pathIndex {
		return ""
	}
	return os.Args[pathIndex]
}

func outcome(path string) string {
	instance, err := memless.Load(path)
	if err == nil {
		instance.Release()
		return "accepted"
	}
	var refusal *memless.RefusalError
	if errors.As(err, &refusal) {
		return "refused: " + refusal.Message
	}
	return "fault: " + err.Error()
}
