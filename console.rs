#[allow(unstable)]
extern crate core;

use core::marker::Copy;
use core::option::Option;
use core::iter::Iterator;
use core::str::StrExt;

use asm;

pub struct Console {
    pub position:u32
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
        self.move_cursor(0);
    }

    #[allow(unstable)]
    pub fn print_string(&mut self,string: &str) {
        let mut bytes = string.bytes();
        loop {
            match bytes.next() {
                Option::Some(x) => {
                    unsafe {
                        *((0xb8000 + self.position*2) as *mut u8) = x;
                        *((0xb8000 + self.position*2 + 1) as *mut u8) = 0x0f;
                    }
                    self.position += 1;
                },
                Option::None =>{break}
            }
        }
        self.update_cursor();
    }

    fn update_cursor(&self) {
        self.move_cursor(self.position);
    }

    fn move_cursor(&self,pos: u32) {
        asm::outb(0x3D4,15);
        asm::outb(0x3D5,(pos & 0xff) as u8);
        asm::outb(0x3D4,14);
        asm::outb(0x3D5,((pos >> 8) & 0xff) as u8);
    }
}
