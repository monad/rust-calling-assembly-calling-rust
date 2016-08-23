all:
	mkdir -p libs
	nasm -f elf64 main.s -o libs/main.o
	ar rcs libs/libmain.a libs/main.o
	cargo run
