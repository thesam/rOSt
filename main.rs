#![no_std]

#![allow(improper_ctypes)]

#![feature(asm)]

extern crate core;

use core::iter::Iterator;
use core::str::StrExt;
use core::option::Option;

use core::marker::Copy;

impl Copy for Color {}

#[allow(dead_code)]
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
    move_cursor(0,string.len() as u8);
}

fn move_cursor(row: u8, col: u8) {
    let pos:u16 = (row*80 + col) as u16;
    outb(0x3D4,15);
    outb(0x3D5,(pos & 0xff) as u8);
    outb(0x3D4,14);
    outb(0x3D5,((pos >> 8) & 0xff) as u8);
}

fn outb(port:u16, value:u8) {
    unsafe {
        asm!(
        "out $0,$1"
        :
        : "{ax}"(value), "{dx}"(port)
        )
    }
}

struct IDTEntry {
    offset_lo: u16,
    selector: u16,
    zero: u8,
    type_attr: u8,
    offset_hi: u16       
}

impl Copy for IDTEntry {}

fn empty_entry() -> IDTEntry {
    IDTEntry {offset_lo: 0, selector: 0, zero: 0, type_attr: 0, offset_hi: 0}     
}

fn lidt() {
    struct IDT {
        entries: [IDTEntry; 256]
    }
    struct IDTR {
        length: u16,
        base: u32
    }
    let mut idt = IDT {entries: [empty_entry();256]};
    unsafe {
        let idt_addr:*mut IDT = &mut idt;
        let idtr = IDTR {length: 64*256, base: (idt_addr) as u32};
        asm!("lidt ($0)"::"{ax}"(&idtr))
    }
}

extern {
    fn memset() -> isize;
}

fn test() {
    unsafe {
        let x = memset();
    }
}

#[no_mangle]
#[no_stack_check]
pub fn main() {
    clear_screen(Color::LightRed);
    print_string("Hello world");
    test();
    //lidt();
}
