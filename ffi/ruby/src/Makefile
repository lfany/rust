##Makefile for go shared libtest

GOBIN=$(shell which go)
all:
	@pwd
	@env
	@cd .. &&cargo build --release
	$(GOBIN) build -x -v ffi.go
	ifeq($(shelluname-s),Linux)
	@cp../target/release/libruby.so.
	@chmoda+x./src
	@./src
	elseifeq($(shelluname-o),'Msys')
	@cp../target/release/ruby.dll.
	@./src.exe
	else
	@cp../target/release/libruby.dylib.
	@chmoda+x./src
	@./src

clean:
	rm-rfruby.dlllibruby.solibruby.dylibsrcsrc.exe
