use16

org 0x7c00

boot:
    ; initialize segment registers
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    ; load rust code into 0x7e00 so we can jump to it later
    mov bx, 0x7e00  ; read buffer
read_loop:
    mov ah, 2       ; read
    mov al, 1      ; 1 sector at a time (512B)
    mov ch, [cylinder]       ; cylinder & 0xff
    ;  Sector 1 is the first sector and contains this bootloader code (512 bytes). The Rust code starts at sector 2.
    mov cl, [sector]       ; sector | ((cylinder >> 2) & 0xc0)
    mov dh, [head]       ; head
    int 0x13
    jc error
    add bx, 0x200 ; 512B read
    jnc inc_sector
    mov ax, es
    add ax, 0x1000
    mov es, ax ; bx overflow, compensate with ES (0x1000 => 0x10000)
inc_sector:
    inc byte [sector]
    cmp byte [sector], 19 ; 18+1
    jne check_sector_count
    mov byte [sector], 1
    inc byte [head] ; increase head
    cmp byte [head], 2 ; 1+1
    jne check_sector_count
    mov byte [head], 0
    inc byte [cylinder]
check_sector_count:    
    dec byte [sector_count]
    cmp byte [sector_count], 0
    jne read_loop

    ; load protected mode GDT and a null IDT (we don't need interrupts)
    cli
    lgdt [gdtr]
    lidt [idtr]
    ; initialize stack
    mov sp, 0x7bfe
    ; set protected mode bit of cr0
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    ; far jump to load CS with 32 bit segment
    jmp 0x08:protected_mode

error:
    mov si, .msg
.loop:
    lodsb
    or al, al
    jz .done
    mov ah, 0x0e
    int 0x10
    jmp .loop
.done:
    jmp $
    .msg db "rOSt: could not read disk", 0

cylinder:
    db 0
head:
    db 0 
sector:
    ; Start at sector 2, since bootloader is already loaded from sector 1
    db 2
sector_count:
    ; Want to read 399 sectors after bootloader => 200KB including bootloader
    db 399

protected_mode:
    use32
    ; load all the other segments with 32 bit data segments
    mov eax, 0x10
    mov ds, eax
    mov es, eax
    mov fs, eax
    mov gs, eax
    mov ss, eax
    ; set up stack
    mov esp, 0x7bfc
    ; jump into rust
    call 0x7e00
    jmp $

gdtr:
    dw (gdt_end - gdt) + 1  ; size
    dd gdt                  ; offset

idtr:
    dw 0
    dd 0

gdt:
    ; null entry
    dq 0
    ; code entry
    dw 0xffff       ; limit 0:15
    dw 0x0000       ; base 0:15
    db 0x00         ; base 16:23
    db 0b10011010   ; access byte - code
    db 0x4f         ; flags/(limit 16:19). flag is set to 32 bit protected mode
    db 0x00         ; base 24:31
    ; data entry
    dw 0xffff       ; limit 0:15
    dw 0x0000       ; base 0:15
    db 0x00         ; base 16:23
    db 0b10010010   ; access byte - data
    db 0x4f         ; flags/(limit 16:19). flag is set to 32 bit protected mode
    db 0x00         ; base 24:31
gdt_end:

times 510-($-$$) db 0
db 0x55
db 0xaa
