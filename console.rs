#[allow(unstable)]
extern crate core;

use core::marker::Copy;
use core::option::Option;
use core::iter::Iterator;
use core::str::StrExt;
use core::iter::range;

use asm;
use interrupt;

pub struct Console {
    position:u32
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

impl Copy for Color {}

static mut console: Console = Console {position: 0}; 

impl Console {
    pub fn init() -> &'static mut Console {
        interrupt::register_handler(on_keyboard_interrupt);
        unsafe {
            return &mut console;
        }
    }

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
        if c == '\n' {
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
        let output:[u8;64] = [0;64];
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
            if left == 0 {
                break;                
            }
        }
        loop {
            let c:char = (output[(length-1) as usize] + 0x30) as char;
            self.print_char(c);
            length -= 1;
            if length == 0 {
                break;
            }
        }
    }

    fn update_cursor(&self) {
        self.move_cursor(self.position);
    }

    fn move_cursor(&self,pos: u32) {
        asm::out8(0x3D4,15);
        asm::out8(0x3D5,(pos & 0xff) as u8);
        asm::out8(0x3D4,14);
        asm::out8(0x3D5,((pos >> 8) & 0xff) as u8);
    }
}

#[no_stack_check]
fn on_keyboard_interrupt() {
    let scancode = asm::in8(0x60);
    // Keydown
    if scancode & 0b10000000 == 0 {
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
    charmap[0x1C] = '\n';

    return charmap[scancode as usize];
}
