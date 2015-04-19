use asm;

// PCI, from OSDev wiki
pub fn check_vendor(bus: u8, slot: u8) -> u16 {
    let vendor:u16 = config_read_word(bus,slot,0,0);
    return vendor;
}

pub fn config_read_word(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    let address = ((bus as u32) << 16) | ((slot as u32) << 11) |
              ((func as u32) << 8) | ((offset as u32) & 0xfc) | (0x80000000 as u32);
    asm::out32(0xCF8, address);
    let tmp = ((asm::in32(0xCFC) >> ((offset & 2) * 8)) & 0xffff) as u16;
    return tmp;
}
