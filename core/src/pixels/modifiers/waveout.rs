use crate::pixels::pixelstrip::PixelStrip;

use super::{temporal_progression::TemporalProgression, ModifierChainable, ModifierParameters, ModifierResult};

pub struct WaveoutModifier
{
    progressor:TemporalProgression
}

impl ModifierChainable for WaveoutModifier
{
    fn run(&mut self, pixel_strip:&mut PixelStrip, params:&ModifierParameters)->ModifierResult
    {

        ModifierResult::Continue
    }
}