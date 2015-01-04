#![no_std]
#![allow(ctypes)]
#![feature(lang_items)]

extern crate core;

use core::iter;
use core::str::StrExt;

use core::kinds::Copy;

impl Copy for Color {}

enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}

enum Option<T> {
    None,
    Some(T)
}

struct IntRange {
    cur: int,
    max: int
}

impl IntRange {
    fn next(&mut self) -> Option<int> {
        if self.cur < self.max {
            self.cur += 1;
            Option::Some(self.cur - 1)
        } else {
            Option::None
        }
    }
}

fn range(lo: int, hi: int) -> IntRange {
    IntRange { cur: lo, max: hi }
}

fn clear_screen(background: Color) {
    let mut r = range(0, 80 * 25);
    loop {
        match r.next() {
            Option::Some(x) => {
                unsafe {
                    *((0xb8000 + x * 2) as *mut u16) = (background as u16) << 12;
                }
            },
            Option::None =>{break}
        }
    }
}

fn print_string(string: &str) {
        unsafe {
            let character = 'A' as u8;
            *((0xb8000) as *mut u8) = character;
            *((0xb8000+1) as *mut u8) = 0x0f;
        }
    let bytes = string.chars();
        unsafe {
            let character = 'B' as u8;
            *((0xb8000) as *mut u8) = character;
            *((0xb8000+1) as *mut u8) = 0x0f;
        }
}

#[no_mangle]
#[no_split_stack]
pub fn main() {
    clear_screen(Color::LightRed);
    print_string("A");
}
