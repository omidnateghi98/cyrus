package main

import (
    "fmt"
    "runtime"
)

func main() {
    fmt.Println("Hello from Cyrus Go environment!")
    fmt.Printf("Go version: %s\n", runtime.Version())
}
