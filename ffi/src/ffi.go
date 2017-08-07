package main

/*
#cgo LDFLAGS: -L${SRCDIR} -lruby_process
//#cgo LDFLAGS: ${SRCDIR}/../target/release/libruby.so
//#include <stdio.h>
void process();
*/
import "C"

func main() {
	C.process()
}
