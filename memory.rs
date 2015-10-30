// Dynamic allocation of memory
extern {
    // Not really a function, but we need the address to the heap
    fn heap_memory();
}

static mut heap_position:*mut u8 = heap_memory as *mut u8;

#[lang="exchange_malloc"]
unsafe fn allocate(size: usize, _align: usize) -> *mut u8 {
    let p:*mut u8 = heap_position;
    //TODO: align?
    heap_position = (heap_position as usize + size) as *mut u8;
    return p;
}

pub fn alloc(size: u32) -> *mut u8 {
    unsafe {
        return allocate(size as usize, 0 as usize);
    }
}

#[lang="exchange_free"]
unsafe fn deallocate(ptr: *mut u8, _size: usize, _align: usize) {
    //TODO: Implement
}
