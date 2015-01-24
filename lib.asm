section .text

extern int_handler

global memset
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

global asm_int_handler
asm_int_handler:
    pushad
    call int_handler
    popad
    iret

global __fixunssfdi
__fixunssfdi:
global __fixunsdfdi
__fixunsdfdi:
global __powisf2
__powisf2:
global __powidf2
__powidf2:
global __mulodi4
__mulodi4:
global __moddi3
__moddi3:
global __umoddi3
__umoddi3:
global __divdi3
__divdi3:
global __udivdi3
__udivdi3:
global trunc
trunc:
global truncf
truncf:
global fmod
fmod:
global fmodf
fmodf:
global pow
pow:
global powf
powf:
global floor
floor:
global floorf
floorf:
global log10
log10:
global log10f
log10f:
global memcpy
memcpy:
global memcmp
memcmp:
global log
log:
global log2
log2:
global exp
exp:
global exp2
exp2:
global exp2f
exp2f:
global fma
fma:
global fmaf
fmaf:
global round
round:
global roundf
roundf:
global ceil
ceil:
global ceilf
ceilf:
global log2f
log2f:
global logf
logf:
global expf
expf:
    jmp $
