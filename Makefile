LD=ld
RUSTC=rustc
NASM=nasm
QEMU=qemu-system-i386

all: floppy.img

.SUFFIXES: .o .rs .asm

.PHONY: clean run

main.o: *.rs
	$(RUSTC) -O --target i686-unknown-linux-gnu --crate-type lib --emit obj -o $@ -C relocation-model=static main.rs

lib.o: lib.asm
	$(NASM) -f elf32 -o $@ $<

floppy.img: loader.bin main.bin
	dd if=/dev/zero of=$@ bs=512 count=2 &>/dev/null
	cat $^ | dd if=/dev/stdin of=$@ conv=notrunc &>/dev/null

loader.bin: loader.asm
	$(NASM) -o $@ -f bin $<

main.bin: linker.ld main.o lib.o
	$(LD) -m elf_i386 -o $@ -T $<

run: floppy.img
	$(QEMU) -s -fda $<

debug: floppy.img
	$(QEMU) -s -S -fda $<

clean:
	rm -f *.bin *.o *.img
