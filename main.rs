#![no_std]

#![feature(no_std)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(box_syntax)]
#![feature(core)]
#![feature(unique)]
#![feature(core_str_ext)]

//extern crate core;

use console::Color;

// Must be re-exported to be called from assembly
pub use interrupt::int_handler;

mod console;
mod asm;
mod interrupt;
mod error;
mod pci;
mod memory;

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
            let vendor = pci::check_vendor(bus,slot);
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

    let foo = console.read_string();
    console.print_string("You wrote: ");
    console.print_string(foo);
}

// Stubs for functions needed to build as static lib.
#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop{} }
