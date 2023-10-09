use bootloader_api::info::FrameBuffer;
use bootloader_api::BootInfo;
use conquer_once::raw::OnceCell;
use core::fmt::{self, Result, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

pub mod buffer;
pub mod color;
pub mod macros;
pub mod vga;

// // Create global Writer static type
// lazy_static! {
//     pub static ref WRITER: Mutex<buffer::FrameBufferWriter> =
//         Mutex::new(buffer::FrameBufferWriter::new(None, None));
// }

pub fn init(framebuffer: FrameBuffer) {
    let buffer: FrameBuffer = framebuffer;
}

// private crate print function used in println! marco
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    // interrupts::without_interrupts(|| {
    //     WRITER.lock().write_fmt(args).unwrap();
    // })
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
