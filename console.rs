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
    #[no_stack_check]
    pub fn print_string(&mut self,string: &str) {
        let mut bytes = string.chars();
        loop {
            match bytes.next() {
                Option::Some(x) => {
                    self.print_char(x);
                },
                Option::None =>{break}
            }
        }
    }

    pub fn print_char(&mut self,c: char) {
        if (c == '\n') {
            self.position += 80;
            let col = self.position % 80;
            self.position -= col;
        } else {
            unsafe {
                *((0xb8000 + self.position*2) as *mut u8) = c as u8;
                *((0xb8000 + self.position*2 + 1) as *mut u8) = 0x0f;
            }
            self.position += 1;
        }
        self.update_cursor();
    }

    pub fn print_int(&mut self, number: u32) {
        let mut length = 0;
        let mut output:[u8;64] = [0;64];
        let mut left = number;
        loop {
            //TODO: Must be a better way to write to vector without bounds checking
            let output_addr:*const u8 = &output[0];
            let output_addr_u32:u32=output_addr as u32;
            unsafe {
                *((output_addr_u32 + length) as *mut u8) = (left % 10) as u8;
            }
            length += 1;
            left = left / 10;
            if (left == 0) {
                break;                
            }
        }
        loop {
            let c:char = (output[(length-1) as usize] + 0x30) as char;
            self.print_char(c);
            length -= 1;
            if (length == 0) {
                break;
            }
        }
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
