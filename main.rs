#![no_std]

#![feature(no_std)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(box_syntax)]
#![feature(core)]
#![feature(unique)]

extern crate core;

use console::Color;

// Must be re-exported to be called from assembly
pub use interrupt::int_handler;

mod console;
mod asm;
mod interrupt;
mod error;

use core::ptr::Unique;

#[lang="owned_box"]
pub struct Box<T>(Unique<T>);

#[no_mangle]
#[no_stack_check]
pub fn main() {
    let mut console = console::Console::init();
    console.clear_screen(Color::Black);

    console.print_string("Welcome to rOSt.\n");

    console.print_string("Begin PCI Scan...\n");
    for bus in 0..255 {
        for slot in 0..31 {
            let vendor = pci_check_vendor(bus,slot); 
            if vendor != 0xFFFF {
                console.print_string("Device found: ");
                console.print_int(bus as u32);
                console.print_string("-");
                console.print_int(slot as u32);
                console.print_char('\n');
                console.print_string("Vendor: ");
                console.print_int(vendor as u32);
                console.print_char('\n');
            }
        }
    }
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
fn pci_check_vendor(bus: u8, slot: u8) -> u16 {
    let vendor:u16 = pci_config_read_word(bus,slot,0,0);
    return vendor;
}

fn pci_config_read_word(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    let address = ((bus as u32) << 16) | ((slot as u32) << 11) |
              ((func as u32) << 8) | ((offset as u32) & 0xfc) | (0x80000000 as u32);
    asm::out32(0xCF8, address);
    let tmp = ((asm::in32(0xCFC) >> ((offset & 2) * 8)) & 0xffff) as u16;
    return tmp;
}
