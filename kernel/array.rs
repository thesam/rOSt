use core::mem;
use core::slice;
use kernel::memory;

pub struct Array {
    content: *mut u8,
    pub length: usize
}

impl Array {
    pub fn new() -> Array {
        Array {content: 0 as *mut u8, length: 0}
    }

    pub fn push(&mut self, value: u8) {
        unsafe {
            //TODO: This is what we call a memory leak. This memory is never deallocated.
            let newcontent = memory::alloc(self.length + 1);
            let oldslice:&mut [u8] = slice::from_raw_parts_mut(self.content, self.length);
            let newslice:&mut [u8] = slice::from_raw_parts_mut(newcontent, self.length + 1);
            for i in 0..self.length {
                newslice[i] = oldslice[i];
            }
            newslice[self.length] = value;
            self.content = newcontent;
            self.length = self.length + 1;
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            let slice:&[u8] = slice::from_raw_parts_mut(self.content, self.length);
            slice
        }
    }
}
