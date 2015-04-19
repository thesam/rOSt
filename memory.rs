// Dynamic allocation of memory
static mut heap:u8 = 0;

#[lang="exchange_malloc"]
unsafe fn allocate(size: usize, _align: usize) -> *mut u8 {
    //TODO: Implement support more for than 1 byte :)
    let p:*mut u8 = &mut heap;
    return p;
}
#[lang="exchange_free"]
unsafe fn deallocate(ptr: *mut u8, _size: usize, _align: usize) {
    //TODO: Implement
}

