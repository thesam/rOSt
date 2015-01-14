section .text
global memset
global asm_int_49
global asm_int_handler
global asm_load_handler

extern int_handler

memset:
    push ebp
    mov ebp,esp

    push ebx
    push ecx
    push edx

    mov eax,[ebp+8] ; void * ptr
    mov cl,[ebp+12] ; char value
    mov edx,[ebp+16] ; int num

    add edx,eax

memset_loop:
    mov [eax],cl
    inc eax
    cmp eax,edx
    jne memset_loop

    mov eax,[ebp+8]

    pop edx
    pop ecx
    pop ebx

    mov esp, ebp
    pop ebp
    ret

asm_int_49:
    int 49
    ret

asm_int_handler:
    pushad
    call int_handler
    popad
    iret

asm_load_handler:
    push ebp
    mov ebp,esp
    pushad

    mov eax,[ebp+8] ; handler
    mov ebx,[ebp+12] ; idt
    mov ecx, idtr
    add ecx,2
    mov [ecx],ebx
    lidt [idtr]
    mov [ebx+49*8],ax
    mov word [ebx+49*8+2],0x08 ; Offset of code segment in GT
    mov word [ebx+49*8+4],0x8E00
    shr eax,16
    mov [ebx+49*8+6],ax

    popad
    mov esp, ebp
    pop ebp

    ret

idtr:
    dw (50*8)-1
    dd 0
