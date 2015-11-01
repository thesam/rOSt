pub fn out8(port:u16, value:u8) {
    unsafe {
        asm!(
        "out $0,$1"
        :
        : "{ax}"(value), "{dx}"(port)
        )
    }
}

pub fn out32(port:u16, value:u32) {
    unsafe {
        asm!(
        "out $0,$1"
        :
        : "{eax}"(value), "{dx}"(port)
        )
    }
}

pub fn in8(port:u16) -> u8 {
    let mut value:u8;
    unsafe {
        //TODO: Figure out the input/output constraints, is this correct?
        asm!(
        "inb $1,$0"
        : "={al}"(value)
        : "{dx}"(port)
        )
    }
    return value;
}

pub fn in32(port:u16) -> u32 {
    let mut value:u32;
    unsafe {
        //TODO: Figure out the input/output constraints, is this correct?
        asm!(
        "in $1,$0"
        : "={eax}"(value)
        : "{dx}"(port)
        )
    }
    return value;
}

pub fn nop() {
    unsafe {
        asm!("nop")
    }
}
