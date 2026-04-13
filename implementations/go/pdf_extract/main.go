package main

import (
	fmt "fmt"
	"os"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: go run main.go <pdf_file>")
		return
	}
}