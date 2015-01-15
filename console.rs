extern crate core;

use core::marker::Copy;
use core::option::Option;
use core::iter::Iterator;
use core::str::StrExt;

use asm;

pub struct Console {
    pub position: u8
}

#[allow(dead_code)]
pub enum Color {
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

struct IntRange {
    cur: isize,
    max: isize
}

impl IntRange {
    fn next(&mut self) -> Option<isize> {
        if self.cur < self.max {
            self.cur += 1;
            Option::Some(self.cur - 1)
        } else {
            Option::None
        }
    }
}

fn range(lo: isize, hi: isize) -> IntRange {
    IntRange { cur: lo, max: hi }
}

impl Copy for Color {}

impl Console {
    pub fn clear_screen(&self,background: Color) {
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

    pub fn print_string(&self,string: &str) {
        let mut bytes = string.bytes();
        let mut count = 0;
        loop {
            match bytes.next() {
                Option::Some(x) => {
                    unsafe {
                        *((0xb8000 + count) as *mut u8) = x;
                        *((0xb8000 + count + 1) as *mut u8) = 0x0f;
                    }
                    count = count + 2;
                },
                Option::None =>{break}
            }
        }
        self.move_cursor(0,string.len() as u8);
    }

    pub fn move_cursor(&self, row: u8, col: u8) {
        let pos:u16 = (row*80 + col) as u16;
        asm::outb(0x3D4,15);
        asm::outb(0x3D5,(pos & 0xff) as u8);
        asm::outb(0x3D4,14);
        asm::outb(0x3D5,((pos >> 8) & 0xff) as u8);
    }
}
