#![no_std]

#![feature(asm)]

extern crate core;

use core::iter::Iterator;
use core::str::StrExt;
use core::marker::Copy;
use console::Console;
use console::Color;

// Must be re-exported to be called from assembly
pub use interrupt::int_handler;

mod console;
mod asm;
mod interrupt;

static console: console::Console = console::Console {position:0}; 

#[no_mangle]
#[no_stack_check]
pub fn main() { 
    console.clear_screen(Color::LightRed);
    console.print_string("Hello world");
    interrupt::register_handler(on_keyboard_interrupt);
    console.print_string("End world");
}

fn on_keyboard_interrupt() {
    console.clear_screen(Color::Blue);
}



