package main

/*
#cgo LDFLAGS: ./../target/debug/libsay_hello.a -ldl
#include "./lib/say_hello.h"
*/
import "C"

import "fmt"
import "time"

func main() {
	start := time.Now()
    fmt.Println("Hello from GoLang!")
	duration := time.Since(start)
	fmt.Println(duration)
	start2 := time.Now()
	C.hello(C.CString("from Rust!"))
	duration2 := time.Since(start2)
	fmt.Println(duration2)
}