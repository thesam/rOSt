use array::Array;
use core::str::from_utf8;

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
        match self.chars.content {
            Some(chars) => {
                from_utf8(chars as &mut [u8]).unwrap()
            },
            None => ""
        }
    }
}
