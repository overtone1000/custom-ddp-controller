use angular_units::Deg;

use crate::pixels::{pixel::HSV, pixelstrip::PixelStrip};

use super::{temporal_oscillation::TemporalOscillation, ModifierChainable, ModifierParameters, ModifierResult};

pub struct RainbowOscillationModifier
{
    oscillator:TemporalOscillation
}

impl ModifierChainable for RainbowOscillationModifier
{
    fn run(&mut self, pixel_strip:&mut PixelStrip, params:&ModifierParameters)->ModifierResult
    {
        
        let rotational_offset_from_time=self.oscillator.get_rotation_at_time(&params.time);
        
        let pixel_count_f64:f64=(pixel_strip.count() as u32).into();

        for a in 0..pixel_strip.count() {
            let rotational_offset_from_location = f64::from(a as u32) / pixel_count_f64;
            let total_offset =
                rotational_offset_from_location + rotational_offset_from_time;
            let total_offset = total_offset % 1.0; //This prevents a bug with large angles.
            let final_angle = Deg(360.0 * total_offset);

            let hue = HSV::new(final_angle, 1.0, 1.0);

            pixel_strip.set_pixel_hsv(a, hue);
        }

        ModifierResult::Continue
    }
}