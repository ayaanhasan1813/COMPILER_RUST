rcompiler = ./target/debug/rcompiler

build:
	cargo build

test: build
	@$(rcompiler) test/test.c > tmp-test1.s
	@gcc -c -o tmp-test2.o test/gcc.c
	@gcc -static -o tmp-test1 tmp-test1.s tmp-test2.o
	@./tmp-test1
	@$(rcompiler) ./test/token.c > tmp-test2.s
	@gcc -static -o tmp-test2 tmp-test2.s
	@./tmp-test2

clean:
	rm -f *~ tmp*

fib:
	@$(rcompiler) examples/fib.c > tmp-fib.s
	@gcc -static -o tmp-fib tmp-fib.s 
	@./tmp-fib

prime:
	@$(rcompiler) examples/prime.c > tmp-prime.s
	@gcc -static -o tmp-prime tmp-prime.s
	@./tmp-prime

.PHONY: test clean
