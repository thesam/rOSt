extern crate core;

use core::option::Option;
use core::iter::Iterator;
//use core::str::StrExt;
use core::str::from_utf8;
use kernel::array::Array;

use kernel::asm;
use kernel::interrupt;
use kernel::string::String;

pub struct Console {
    position:u32,
    inputbuffer:[char;512],
    inputposition:usize,
    inputready:bool
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
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

pub static mut console: Console = Console {position: 0, inputbuffer: [' ';512], inputready: false, inputposition: 0};

impl Console {
    pub fn init() -> &'static mut Console {
        interrupt::register_handler(on_keyboard_interrupt);
        unsafe {
            return &mut console;
        }
    }

    pub fn clear_screen(&self,background: Color) {
        let mut r = 0..80 * 25;
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

    #[no_stack_check]
    pub fn print_string(&mut self,string: &str) {
        let mut bytes = string.chars();
        loop {
            match bytes.next() {
                Option::Some(x) => {
                    if x == '\0' {
                        break;
                    }
                    self.print_char(x);
                },
                Option::None =>{break}
            }
        }
    }

    #[no_stack_check]
    pub fn print_char(&mut self,c: char) {
        if c == '\n' {
            let col = self.position % 80;
            self.position -= col;
            let current_row = self.current_row();
            if (current_row < 24) {
                self.position += 80;
            } else {
                self.scroll_content_up();
            }
        } else {
            unsafe {
                *((0xb8000 + self.position*2) as *mut u8) = c as u8;
                *((0xb8000 + self.position*2 + 1) as *mut u8) = 0x0f;
            }
            let col = self.position % 80;
            if (col == 79 && self.current_row() == 24) {
                self.scroll_content_up();
                self.position -= col;
            } else {
                self.position += 1;
            }
        }
        self.update_cursor();
    }

    fn scroll_content_up(&self) {
        // Scroll everything one row up.
        let mut r = 0..80 * 25;
        loop {
            match r.next() {
                Option::Some(x) => {
                    unsafe {
                        *((0xb8000 + x*2) as *mut u16) = *((0xb8000 + (x+80)*2) as *mut u16)
                    }
                },
                Option::None =>{break}
            }
        }
    }

    #[no_stack_check]
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

    pub fn read_string(&mut self,  buf: &mut [u8;128]) -> String {
        Array::new();
        self.inputready = false;
         while !self.inputready {
            //TODO: This loop should be empty
            asm::nop();
        }
        let mut buf = String::new();
        for i in  0..self.inputposition {
            buf.append(self.inputbuffer[i] as u8);
        }
        self.inputposition = 0;
        return buf;
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

    fn current_row(&self) -> u32 {
        return self.position / 80;
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
            console.inputbuffer[console.inputposition] = c;
            console.inputposition += 1;
            if c == '\n' {
                console.inputready = true;
            }
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
    charmap[0xC] = '-';
    charmap[0xD] = '=';
    charmap[0xE] = 'f';
    charmap[0xF] = '\t';
    charmap[0x10] = 'q';
    charmap[0x11] = 'w';
    charmap[0x12] = 'e';
    charmap[0x13] = 'r';
    charmap[0x14] = 't';
    charmap[0x15] = 'y';
    charmap[0x16] = 'u';
    charmap[0x17] = 'i';
    charmap[0x18] = 'o';
    charmap[0x19] = 'p';
    charmap[0x1a] = '[';
    charmap[0x1B] = ']';
    charmap[0x1C] = '\n';
    charmap[0x1D] = 'L'; // left control
    charmap[0x1E] = 'a';
    charmap[0x1F] = 's';
    charmap[0x20] = 'd';
    charmap[0x21] = 'f';
    charmap[0x22] = 'g';
    charmap[0x23] = 'h';
    charmap[0x24] = 'j';
    charmap[0x25] = 'k';
    charmap[0x26] = 'l';
    charmap[0x27] = ';';
    charmap[0x28] = '\'';
    charmap[0x29] = '`';
    charmap[0x2a] = 'L'; // Left shift
    charmap[0x2b] = '\\';
    charmap[0x2c] = 'z';
    charmap[0x2d] = 'x';
    charmap[0x2e] = 'c';
    charmap[0x2f] = 'v';
    charmap[0x30] = 'b';
    charmap[0x31] = 'n';
    charmap[0x32] = 'm';
    charmap[0x33] = ',';
    charmap[0x34] = '.';
    charmap[0x35] = '/';
    charmap[0x36] = 'R'; // Right shift
    charmap[0x37] = '*';
    charmap[0x38] = 'L'; // Left Alt
    charmap[0x39] = ' ';

    return charmap[scancode as usize];
}
