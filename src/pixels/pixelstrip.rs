use ddp_rs::{connection, error::DDPError};
use prisma::{FromColor, Rgb};

use super::pixel::{PixelSource, HSV};

#[derive(Clone)]
pub struct PixelStrip {
    raw_bytes: Vec<u8>,
    pixel_colors: Vec<HSV>,
}

const RED_OFFSET: usize = 0;
const GREEN_OFFSET: usize = 1;
const BLUE_OFFSET: usize = 2;
const COLORS_PER_PIXEL: usize = 3;

fn set_byte(mutable_bytes: &mut [u8], byte_index: usize, value: u8) {
    match mutable_bytes.get_mut(byte_index) {
        Some(byte) => *byte = value,
        None => eprintln!("Access error."),
    }
}

impl PixelStrip {
    pub fn create(pixel_count: usize) -> PixelStrip {
        let mut pixel_vector = Vec::with_capacity(pixel_count);
        for _ in 0..pixel_count {
            pixel_vector.push(HSV::default())
        }

        let byte_count = pixel_count * COLORS_PER_PIXEL;
        let mut byte_vector = Vec::with_capacity(byte_count);
        for _ in 0..byte_count {
            byte_vector.push(0);
        }
        PixelStrip {
            raw_bytes: byte_vector,
            pixel_colors: pixel_vector,
        }
    }

    pub fn count(&self) -> usize {
        self.pixel_colors.len()
    }

    pub fn set_pixel_hsv(&mut self, index: usize, hsv: HSV) {
        match self.pixel_colors.get_mut(index) {
            Some(pixel_color) => {
                *pixel_color = hsv;
            }
            None => {
                eprintln!("Access error.");
            }
        }
    }

    fn flush_pixels_to_bytes(&mut self) {
        let pixel_colors_iterator = self.pixel_colors.iter_mut();
        let mutable_bytes = &mut self.raw_bytes;
        for (pixel_index, pixel) in pixel_colors_iterator.enumerate() {
            let rgb = pixel.to_rgb();
            let pixel_base = pixel_index * COLORS_PER_PIXEL;
            set_byte(mutable_bytes, pixel_base + RED_OFFSET, rgb.red());
            set_byte(mutable_bytes, pixel_base + GREEN_OFFSET, rgb.green());
            set_byte(mutable_bytes, pixel_base + BLUE_OFFSET, rgb.blue());
        }
    }

    pub fn write_to_connection(
        &self,
        conn: &mut connection::DDPConnection,
    ) -> Result<usize, DDPError> {
        conn.write(&self.raw_bytes)
    }

    pub fn flush_and_write(
        &mut self,
        conn: &mut connection::DDPConnection,
    ) -> Result<usize, DDPError> {
        self.flush_pixels_to_bytes();
        self.write_to_connection(conn)
    }
}
