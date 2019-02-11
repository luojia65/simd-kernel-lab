use lazy_static::lazy_static;

lazy_static! {
    pub static ref POSITION: (usize, usize) = (0, 0);
}

pub fn print_byte(byte: u8) {
    let vga_buffer = 0xb8000 as *mut u8;
    unsafe {
        let i = 0;
        *vga_buffer.offset(i as isize * 2) = byte;
        *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    }
}