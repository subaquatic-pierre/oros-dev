#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    col_pos: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
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
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_char: byte,
                    color_code,
                };

                // advance col pos
                self.col_pos += 1;
            }
        }
    }

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

    fn new_line(&mut self) {
        todo!()
    }
}

pub fn print_string() {
    let mut writer = Writer::new();

    writer.write_byte(b'H');
    writer.write_string("ello, ");
    writer.write_string(" WorlD!!!!!!")
}
