use core::str::from_utf8;
use kernel::array::Array;

pub struct String {
    chars: Array
}

impl String {
    pub fn new() -> String {
        String {chars: Array::new()}
    }

    pub fn append(&mut self, c: u8) {
        self.chars.push(c);
    }
}

impl AsRef<str> for String {
    fn as_ref(&self) -> &str {
        if self.chars.length > 0 {
            from_utf8(self.chars.as_slice()).unwrap()
        } else {
            ""
        }
    }
}
