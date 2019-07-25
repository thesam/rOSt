ENTRY(main)
OUTPUT_FORMAT(binary)

MEMORY {
    ram : org = 0x7e00, l = 350K
}

SECTIONS {
    . = 0x7e00;

    .main : {
        main.o(.text.main)
	main.o(*)
    } >ram

    .lib : {
        lib.o(*)
        libcore.rlib(*)
    } >ram

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.rel.eh_frame)
    }
}
