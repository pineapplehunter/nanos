
all: crt0.s ns16550a.s uart.c
	riscv64-elf-gcc crt0.s ns16550a.s uart.c -nostdlib -nodefaultlibs -Wl,-Tkernel.ld