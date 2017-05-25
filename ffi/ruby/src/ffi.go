package main

/*
//#cgo CFLAGS: -I.
#cgo LDFLAGS: ${SRCDIR}/../target/release/ruby.dll
//#include <stdio.h>
void process();
*/
import "C"

func main() {
	C.process()
}