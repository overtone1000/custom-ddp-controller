use ddp_rs::{connection, error::DDPError};

#[derive(Copy, Clone)]
pub struct PixelRgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub fn write_pixels(
    conn: &mut connection::DDPConnection,
    pixels: &[PixelRgb],
) -> Result<usize, DDPError> {
    let mut bytes: Vec<u8> = Vec::with_capacity(pixels.len() * 3);

    for pixel in pixels {
        bytes.push(pixel.red);
        bytes.push(pixel.green);
        bytes.push(pixel.blue);
    }

    conn.write(&bytes)
}
