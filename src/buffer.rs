const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

use core::fmt;

pub struct Writer {
    pub column_position: usize,
}

impl Writer {
    pub fn new() -> Writer {
        Writer { column_position: 0 }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s : &str) -> fmt::Result {
        unsafe {
            for byte in s.bytes() {
                let offset = self.column_position * 2;
                *VGA_BUFFER.offset(offset as isize) = byte;
                *VGA_BUFFER.offset(offset as isize + 1) = 0x0f;
                self.column_position += 1;
            }
        }
        Ok(())
    }
}

pub static mut WRITER: Writer = Writer { column_position: 0 };

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            unsafe {
                WRITER.write_fmt(format_args!($($arg)*)).unwrap();
            }
        }
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*));
    };
}