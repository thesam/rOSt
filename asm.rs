pub fn outb(port:u16, value:u8) {
    unsafe {
        asm!(
        "out $0,$1"
        :
        : "{ax}"(value), "{dx}"(port)
        )
    }
}

pub fn inb(port:u16) -> u8 {
    let mut value:u8 = 0;
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
