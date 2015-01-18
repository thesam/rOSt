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
        let c:u8 = scancode_to_char(scancode);
        unsafe {
            console.print_char(c);
        }
    }
}

fn scancode_to_char(scancode: u8) -> u8 {
    let translation_table:[u8;256] = ['?' as u8;256];
    return translation_table[scancode as usize];
}



