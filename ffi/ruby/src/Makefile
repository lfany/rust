##Makefile for go shared libtest

GOBIN=$(shell which go)
all:
	@pwd
	@env
	@cd .. &&cargo build --release
	$(GOBIN) build -x -v ffi.go
	ifeq ($(shell uname -s), Linux)
	@cp../target/release/libruby.so.
	@chmoda + x./src
	@./src
	else ifeq ($(shell uname -o), Msys)
	@cp ../target/release/ruby.dll .
	@./src.exe
	else
	@cp ../target/release/libruby.dylib .
	@chmod a+x ./src
	@./src
	endif

clean:
	rm-rfruby.dlllibruby.solibruby.dylibsrcsrc.exe
