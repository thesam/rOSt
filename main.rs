#![no_std]

#![feature(asm)]
#![feature(lang_items)]
#![feature(box_syntax)]

#[allow(unstable)]
extern crate core;

use console::Color;

// Must be re-exported to be called from assembly
pub use interrupt::int_handler;

mod console;
mod asm;
mod interrupt;

use core::ptr::Unique;

#[lang="owned_box"]
pub struct Box<T>(Unique<T>);

#[no_mangle]
#[no_stack_check]
pub fn main() {
    let mut console = console::Console::init();
    console.clear_screen(Color::LightRed);

    console.print_string("Hello world");

    console.print_char('A');
    console.print_int(1234567890);

    let boxed = box 5;
    console.print_int(*boxed);

    console.print_int(pci_config_read(0,0,0,0) as u32);

    console.print_string("End world");
}

// Stubs for functions needed to build as static lib.
#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop{} }

// Dynamic allocation of memory
static mut heap:u8 = 0;

#[lang="exchange_malloc"]
unsafe fn allocate(size: usize, _align: usize) -> *mut u8 {
    //TODO: Implement support more for than 1 byte :)
    let p:*mut u8 = &mut heap;
    return p;
}
#[lang="exchange_free"]
unsafe fn deallocate(ptr: *mut u8, _size: usize, _align: usize) {
    //TODO: Implement
}

// PCI, from OSDev wiki
fn pci_config_read(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    let address:u32;
    let lbus:u32 = bus as u32;
    let lslot:u32 = slot as u32;
    let lfunc:u32 = func as u32;
    let tmp:u16;

    address = ((lbus << 16) | (lslot << 11) |
              (lfunc << 8) | (offset as u32 & 0xfc) | (0x80000000 as u32));

    /* (offset & 2) * 8) = 0 will choose the first word of the 32 bits register */
    tmp = ((asm::in32 (0xCFC) >> ((offset as u32 & 2) * 8)) & 0xffff) as u16;
    return tmp;
}
