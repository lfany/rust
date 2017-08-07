##Makefile for go shared libtest

GOBIN=$(shell which go)
all:
	@pwd
	@env
	@cd .. &&cargo build --release
ifeq ($(shell uname -s),Linux)
	@cp ../target/release/libruby_process.so .
	$(GOBIN) build -x -v ffi.go
	@chmod a+x ./ffi
	@./ffi
else ifeq ($(shell uname -o),Msys)
	@cp ../target/release/ruby_process.dll .
	$(GOBIN) build -x -v ffi.go
	@./ffi.exe
else
	@cp ../target/release/libruby_process.dylib .
	$(GOBIN) build -x -v ffi.go
	@chmod a+x ./src
	@./ffi
endif

clean:
	rm -rf ruby_process.dll libruby_process.so libruby_process.dylib ffi ffi.exe
