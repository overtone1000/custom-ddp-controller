use prisma::FromColor;
use serde::Deserialize;

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