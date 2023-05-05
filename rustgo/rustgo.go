package main

/*
#cgo LDFLAGS: ./../target/debug/libsay_hello.a -ldl
#include "./lib/say_hello.h"
*/
import "C"

import "fmt"

func main() {
    fmt.Println("Hello from GoLang!")
	C.hello(C.CString("from Rust!"))
}