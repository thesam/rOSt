extern {
    fn red_screen_of_death();
}

pub fn kernel_panic() {
    unsafe {
        red_screen_of_death();
    }
}
