#![no_std]

#![feature(asm)]

#[allow(unstable)]
extern crate core;

use console::Console;
use console::Color;

// Must be re-exported to be called from assembly
pub use interrupt::int_handler;

mod console;
mod asm;
mod interrupt;

static mut console: console::Console = console::Console {position: 0}; 

#[no_mangle]
#[no_stack_check]
pub fn main() {
    unsafe { 
        console.clear_screen(Color::LightRed);
        console.print_string("Hello world");
        interrupt::register_handler(on_keyboard_interrupt);
        console.print_string("End world");
    }
}

#[no_stack_check]
fn on_keyboard_interrupt() {
    let scancode = asm::inb(0x60);
    // Keydown
    if (scancode & 0b10000000 == 0) {
        let c = scancode_to_char(scancode);
        unsafe {
            console.print_char(c);
        }
    }
}

fn scancode_to_char(scancode: u8) -> char {
    let mut charmap:[char;256] = ['?';256];
    charmap[0x2] = '1';
    charmap[0x3] = '2';
    charmap[0x4] = '3';
    charmap[0x5] = '4';
    charmap[0x6] = '5';
    charmap[0x7] = '6';
    charmap[0x8] = '7';
    charmap[0x9] = '8';
    charmap[0xA] = '9';
    charmap[0xB] = '0';

    return charmap[scancode as usize];
}



