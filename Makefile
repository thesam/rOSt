LD=ld
RUSTC=rustc
NASM=nasm
QEMU=qemu-system-i386 -net nic,model=rtl8139

all: floppy.img

.SUFFIXES: .o .rs .asm

.PHONY: clean run

main.o: main.rs kernel/*.rs apps/*.rs libcore.rlib
	$(RUSTC) -L . -O --target i686-unknown-linux-gnu --crate-type lib --emit obj -o $@ -C relocation-model=static -Ctarget-cpu=generic main.rs

lib.o: kernel/lib.asm
	$(NASM) -f elf32 -o $@ $<

libcore.rlib: libcore/*
	$(RUSTC) -O --target i686-unknown-linux-gnu -o $@ -C relocation-model=static -C no-stack-check -Ctarget-cpu=generic libcore/lib.rs

floppy.img: loader.bin main.bin
	dd if=/dev/zero of=$@ bs=512 count=2 &>/dev/null
	cat $^ | dd if=/dev/stdin of=$@ conv=notrunc &>/dev/null

loader.bin: loader.asm
	$(NASM) -o $@ -f bin $<

main.bin: linker.ld main.o lib.o libcore.rlib
	$(LD) -m elf_i386 -o $@ -T $<

run: floppy.img
	$(QEMU) -s -fda $<

debug: floppy.img
	$(QEMU) -s -S -fda $<

clean:
	rm -f *.bin *.o *.img *.rlib
