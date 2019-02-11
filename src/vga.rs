use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use core::fmt;

lazy_static! {
    static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        pos: 0,
        buf: unsafe { &mut *(0xb8000 as *mut Buffer) },
        color_code: 0
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: u8,
}

#[repr(transparent)]
struct Buffer {
    chars: [Volatile<ScreenChar>; 25 * 80],
}

struct Writer {
    pos: usize,
    color_code: u8,
    buf: &'static mut Buffer,
}

impl Writer {
    fn write_byte_unchecked(&mut self, byte: u8) {
        self.buf.chars[self.pos].write(ScreenChar {
            ascii_character: byte,
            color_code: self.color_code,
        });
        self.pos += 1;
    } 

    fn write_next_line(&mut self) {
        self.pos = (1 + self.pos / 80) * 80;
        if self.pos >= 25 * 80 {
            for i in 1..25 {
                for j in 0..80 {
                    let index = i * 80 + j;
                    self.buf.chars[index - 80].write(self.buf.chars[index].read());
                }
            } 
            let blank = ScreenChar {
                ascii_character: b' ',
                color_code: self.color_code, 
            };
            for j in 0..80 {
                let index = 24 * 80 + j;
                self.buf.chars[index].write(blank);
            }
            self.pos -= 80;
        }
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.write_next_line(),
            0x20...0x7e => self.write_byte_unchecked(byte),
            _ => self.write_byte_unchecked(0xfe),
        }
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments, color_code: u8) {
    use core::fmt::Write;
    let mut writer = WRITER.lock();
    writer.color_code = color_code;
    writer.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*), 14));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}


#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*), 4));
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}
