use prisma::FromColor;
use serde::Deserialize;

use super::pixelstripcommand::PixelValues;

pub type HSV = prisma::Hsv<f64>;
pub type RGB = prisma::Rgb<u8>;

pub trait PixelSource {
    fn to_rgb(&self) -> RGB;
}

impl PixelSource for HSV {
    fn to_rgb(&self) -> RGB {
        let rgb_t = prisma::Rgb::from_color(self);
        rgb_t.color_cast()
    }
}

pub fn hsv_rom_rgb(rgb: &PixelValues) -> HSV {
    let red = f64::from(rgb.red) / 255.0;
    let green = f64::from(rgb.green) / 255.0;
    let blue = f64::from(rgb.blue) / 255.0;
    let rgb_t: prisma::Rgb<f64> = prisma::Rgb::new(red, green, blue);
    HSV::from_color(&rgb_t)
}
