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
    console.print_string("End world");
    console.print_char('A');
    //console.print_int(1234567890);
    //let boxed = box 5;
    //console.print_int(*boxed);
}

// Stubs for functions needed to build as static lib.
#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop{} }

static mut heap:u8 = 0;

#[lang="exchange_malloc"]
unsafe fn allocate(size: usize, _align: usize) -> *mut u8 {
//    let p = libc::malloc(size as libc::size_t) as *mut u8;

//    // malloc failed
//    if p as usize == 0 {
//        abort();
//    }
//
//    p
    let p:*mut u8 = &mut heap;
    return p;
}
#[lang="exchange_free"]
unsafe fn deallocate(ptr: *mut u8, _size: usize, _align: usize) {
//    libc::free(ptr as *mut libc::c_void)
}

