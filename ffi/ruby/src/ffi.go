package main

/*
//#cgo CFLAGS: -I.
#cgo LDFLAGS: ${SRCDIR}/../target/release/libruby.so
//#include <stdio.h>
void process();
*/
import "C"

func main() {
	C.process()
}
