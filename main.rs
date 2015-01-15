#![no_std]

#![feature(asm)]

extern crate core;

use core::iter::Iterator;
use core::str::StrExt;
use core::marker::Copy;
use console::Console;
use console::Color;

mod console;
mod asm;
mod interrupt;

static console: console::Console = console::Console {position:0}; 

#[no_mangle]
#[no_stack_check]
pub fn main() { 
    console.clear_screen(Color::LightRed);
    console.print_string("Hello world");
    interrupt::init_pic();
    interrupt::enable_keyboard_interrupt();
    interrupt::init_interrupt_handlers();
    console.print_string("End world");
}

#[no_mangle]
pub extern fn int_handler() {
    console.clear_screen(Color::Blue);
}

