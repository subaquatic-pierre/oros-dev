use conquer_once::raw::OnceCell;
use core::fmt::{self, Result, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

use super::color::{Buffer, Color, ColorCode, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH};

pub struct Writer {
    col_pos: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            col_pos: 0,
            color_code: ColorCode::new(Color::Yellow, Color::Black),
            buffer: unsafe {
                {
                    &mut *(0xb8000 as *mut Buffer)
                }
            },
        }
    }

    // write ASCII byte to VGA buffer
    pub fn write_byte(&mut self, byte: u8) {
        // match on ASCII byte character
        match byte {
            b'\n' => self.new_line(),
            byte => {
                // advance to next line if buffer width reached
                if self.col_pos >= BUFFER_WIDTH {
                    self.new_line()
                }

                // get current col and row
                let row = BUFFER_HEIGHT - 1;
                let col = self.col_pos;

                // get color code
                let color_code = self.color_code;

                // write byte to buffer
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    color_code,
                });

                // advance col pos
                self.col_pos += 1;
            }
        }
    }

    // write string
    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                // check character is ASCII compliant
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of ASCII table
                _ => self.write_byte(0xfe),
            }
        }
    }

    // move buffer up one line, loose the top most line
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let char = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(char)
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.col_pos = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let space = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(space);
        }
    }
}

// implement Write for writer to use write! macro
impl Write for Writer {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_string(s);
        Ok(())
    }
}

// Create global Writer static type
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

// private crate print function used in println! marco
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    })
}

#[cfg(test)]
mod test {
    use crate::{
        println,
        screen::{buffer::WRITER, color::BUFFER_HEIGHT},
    };

    #[test_case]
    fn test_println_simple() {
        println!("This is a simple test {},", "!")
    }

    #[test_case]
    fn test_println_multiple() {
        for _ in 0..100 {
            println!("Printing many times to the screen");
        }
    }

    #[test_case]
    fn test_println_buffer() {
        use core::fmt::Write;
        use x86_64::instructions::interrupts;

        let s = "Some test string that fits on a single line";
        interrupts::without_interrupts(|| {
            let mut writer = WRITER.lock();
            writeln!(writer, "\n{}", s).expect("writeln failed");
            for (i, c) in s.chars().enumerate() {
                let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
                assert_eq!(char::from(screen_char.ascii_char), c);
            }
        });
    }
}
