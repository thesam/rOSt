use core::marker::Copy;

use asm;

#[allow(dead_code)]
struct IDTEntry {
    offset_lo: u16,
    selector: u16,
    zero: u8,
    type_attr: u8,
    offset_hi: u16       
}

impl Copy for IDTEntry {}

struct IDT {
    entries: [IDTEntry; 256]
}

#[allow(dead_code)]
// Must be packed for lidt to read it correctly.
#[repr(packed)]
struct IDTR {
    length: u16,
    base: u32
}

static mut idt : IDT = IDT {entries: [IDTEntry {offset_lo: 0, selector: 0, zero: 0, type_attr: 0, offset_hi: 0};256]};

static mut keyboard_handler:fn() = do_nothing;

fn do_nothing() {
}

pub fn register_handler(handler:fn()) {
    unsafe {
        keyboard_handler = handler;
    }

    init_pic();
    enable_keyboard_interrupt();
    init_interrupt_handlers();
}

fn lidt() {
    unsafe {
        let idt_addr:*mut IDT = &mut idt;
        let mut idtr = IDTR {length: 256*8-1, base: idt_addr as u32};
        let idtr_addr:*mut IDTR = &mut idtr;
        asm!("lidt ($0)"::"{eax}"(idtr_addr as u32));
        //asm!("hlt");
    }
}

extern {
    fn asm_int_handler();
}

fn init_interrupt_handlers() {
    unsafe {
        let fnptr:unsafe extern fn() = asm_int_handler;
        let fnptr_addr = fnptr as u32;
        //TODO: 0x21 = Remapped interrupt 1 = Keyboard interrupt
        idt.entries[0x21] = IDTEntry {
            offset_lo: fnptr_addr as u16,
            selector: 0x08,
            zero: 0,
            type_attr: 0x8e,
            offset_hi: (fnptr_addr >> 16) as u16
        };
        lidt();
        asm!("sti");
    }
}

fn init_pic() {
	asm::outb(0x20 , 0x11);
	asm::outb(0xA0 , 0x11);

    // Remap interrupts
	asm::outb(0x21 , 0x20);
	asm::outb(0xA1 , 0x28);

	asm::outb(0x21 , 0x00);  
	asm::outb(0xA1 , 0x00);  

	asm::outb(0x21 , 0x01);
	asm::outb(0xA1 , 0x01);

	// Disable all interrupts
	asm::outb(0x21 , 0xff);
	asm::outb(0xA1 , 0xff);
}

fn enable_keyboard_interrupt() {
    asm::outb(0x21 , 0b11111101);
}

#[no_mangle]
#[no_stack_check]
pub extern fn int_handler() {
    unsafe {
        keyboard_handler();
    }
    // Ack interrupt to be able to get the next one
    asm::outb(0x20,0x20);
}

