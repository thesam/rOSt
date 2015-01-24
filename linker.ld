ENTRY(main)
OUTPUT_FORMAT(binary)

MEMORY {
    ram : org = 0x7e00, l = 12K
}

SECTIONS {
    . = 0x7e00;

    .main : {
        main.o(.text.main)
    } >ram

    .lib : {
        lib.o(*)
    } >ram

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.rel.eh_frame)
    }
}
