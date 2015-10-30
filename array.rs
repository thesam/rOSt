use memory;

pub struct Array {
    pub content: Option<[u8]>,
}

impl Array {
    pub fn new() -> Array {
        Array {content: Option::None, length: 0}
    }

    pub fn push(&mut self, value: u8) {
        let newcontent = memory::alloc(self.length + 1);
        match self.content {
            Some(content) => {

            },
            None => {
                unsafe {
                    *newcontent = value;
                }
                self.content = Some(newcontent);
            }
        }
    }
}
