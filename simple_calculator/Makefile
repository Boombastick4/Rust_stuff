all: 
	cargo build
	rustc src/main.rs -o calculator.o
clean: 
	rm ./*.o
	rm Cargo.lock
	rm -rf target/

default: 
	clean 
	all
