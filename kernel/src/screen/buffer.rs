use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use core::{
    fmt::{self, Write},
    ptr,
};
use noto_sans_mono_bitmap::{
    get_raster, get_raster_width, FontWeight, RasterHeight, RasterizedChar,
};

const LINE_SPACING: usize = 2;
const LETTER_SPACING: usize = 0;
const BORDER_PADDING: usize = 1;

/// Height of each char raster. The font size is ~0.84% of this. Thus, this is the line height that
/// enables multiple characters to be side-by-side and appear optically in one line in a natural way.
pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;

/// The width of each single symbol of the mono space font.
pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);

/// Backup character if a desired symbol is not available by the font.
/// The '�' character requires the feature "unicode-specials".
pub const BACKUP_CHAR: char = '�';

pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;

pub struct FrameBufferWriter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
}

impl FrameBufferWriter {
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut writer = Self {
            framebuffer,
            info,
            x_pos: 0,
            y_pos: 0,
        };
        writer.clear();
        writer
    }

    fn new_line(&mut self) {
        self.y_pos += CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.carriage_return()
    }

    fn carriage_return(&mut self) {
        self.x_pos = BORDER_PADDING;
    }

    pub fn clear(&mut self) {
        self.x_pos = BORDER_PADDING;
        self.y_pos = BORDER_PADDING;

        self.framebuffer.fill(0);
    }

    fn width(&self) -> usize {
        self.info.width
    }

    fn height(&self) -> usize {
        self.info.height
    }

    /// Writes single char to buffer, Takes care of special control chars,
    /// such as newlines and carriage returns
    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.new_line(),
            '\r' => self.carriage_return(),
            c => {
                // check if at end of line, got to new line
                let new_x_pos = self.x_pos + CHAR_RASTER_WIDTH;
                if new_x_pos >= self.width() {
                    self.new_line();
                }

                let new_y_pos = self.y_pos + CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
                if new_y_pos >= self.height() {
                    self.clear();
                }

                self.write_rendered_char(FrameBufferWriter::get_char_raster(c));
            }
        }
    }

    /// Prints rendered char into the frame buffer
    /// updates `self.x_pos`
    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x_pos + x, self.y_pos + y, *byte);
            }
        }

        // move cursor forward
        self.x_pos += rendered_char.width() + LETTER_SPACING;
    }

    fn write_pixel(&mut self, x: usize, y: usize, pixel_intensity: u8) {
        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [pixel_intensity, pixel_intensity, pixel_intensity / 2, 0],
            PixelFormat::Bgr => [pixel_intensity / 2, pixel_intensity, pixel_intensity, 0],
            PixelFormat::U8 => [if pixel_intensity > 200 { 0xf } else { 0 }, 0, 0, 0],
            other => {
                // set a supported pixel format before panicking to avoid double
                // panic
                self.info.pixel_format = PixelFormat::Rgb;
                panic!(
                    "pixel format not supported in FrameBufferWriter, {:?}",
                    other
                );
            }
        };

        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;

        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);

        unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };
    }

    // ---
    // static methods
    // ---

    /// returns raster of given char of backup char if not found
    fn get_char_raster(c: char) -> RasterizedChar {
        match get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT) {
            Some(char) => char,
            None => get_raster(BACKUP_CHAR, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
                .expect("Should get backup char"),
        }
    }
}

unsafe impl Send for FrameBufferWriter {}
unsafe impl Sync for FrameBufferWriter {}

impl Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c)
        }
        Ok(())
    }
}
