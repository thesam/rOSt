#![no_std]

#![feature(asm)]
#![feature(lang_items)]

#[allow(unstable)]
extern crate core;

use console::Color;

// Must be re-exported to be called from assembly
pub use interrupt::int_handler;

mod console;
mod asm;
mod interrupt;

#[no_mangle]
#[no_stack_check]
pub fn main() {
    let mut console = console::Console::init();
    console.clear_screen(Color::LightRed);
    console.print_string("Hello world");
    console.print_string("End world");
    console.print_char('A');
    console.print_int(1234567890);
}

// Stubs for functions needed to build as static lib.
#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop{} }


