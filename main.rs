#![no_std]

#![allow(improper_ctypes)]

#![feature(asm)]

extern crate core;

use core::iter::Iterator;
use core::str::StrExt;
use core::str;
use core::option::Option;

use core::marker::Copy;
use core::slice::AsSlice;

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

struct IDT {
    entries: [IDTEntry; 256]
}
struct IDTR {
    length: u16,
    base: u32
}

static mut idt : IDT = IDT {entries: [IDTEntry {offset_lo: 0, selector: 0, zero: 0, type_attr: 0, offset_hi: 0};256]};

fn lidt() {
    unsafe {
        let idt_addr:*mut IDT = &mut idt;
        let idtr = IDTR {length: 256*8-1, base: (idt_addr) as u32};
        asm!("lidt ($0)"::"{ax}"(&idtr))
    }
}

extern {
    fn asm_int_handler();
}

fn init_int_49() {
    unsafe {
        let fnptr:unsafe extern fn() = asm_int_handler;
        let fnptr_addr = fnptr as u32;
        idt.entries[49] = IDTEntry {
            offset_lo: fnptr_addr as u16,
            selector: 0,
            zero: 0,
            type_attr: 0x8e,
            offset_hi: (fnptr_addr >> 16) as u16
        };
    }
}

extern {
    fn asm_int_49(foo : u32);
}

fn int_49(handler:unsafe extern fn()) {
    unsafe {
        asm_int_49(handler as u32);
    }
}

#[no_mangle]
#[no_stack_check]
pub fn main() {
    clear_screen(Color::LightRed);
    print_string("Hello world");
//    lidt();
//    init_int_49();
//    int_49(asm_int_handler);
    print_int(1234567890);
}

fn int_handler() {
    clear_screen(Color::Blue);
}

fn int_handler2() {
    clear_screen(Color::Green);
}

fn print_int(x:u32) {
    let mut left = x;
    let mut out:[u8;20] = [10;20];
    let mut pos = 0; 

    loop {
        let rem = (left % 10) as u8;
        out[pos] = rem;
        pos = pos + 1;
        if (left < 10) {
            break;
        } else {
            left = left / 10;
        }
    }


    let mut i = 0;
    loop {
        if (i == 20) {
            break
        }
        unsafe {
            *((0xb8000 + i*2) as *mut u8) = out[20-1-i] + 48;
            *((0xb8000 + i*2 + 1) as *mut u8) = 0x0f;
        }            
        i = i + 1;   
    }
}
