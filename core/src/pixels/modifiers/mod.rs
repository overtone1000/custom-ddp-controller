use std::time::Instant;

use rainbowoscillation::RainbowOscillationModifier;
use temporal_oscillation::TemporalOscillation;
use waveout::WaveoutModifier;

use super::pixelstrip::PixelStrip;

mod demos;
mod rainbowoscillation;
mod waveout;
mod temporal_oscillation;
mod temporal_progression;

pub enum PixelModifier
{
    RainbowOscillation(RainbowOscillationModifier),
    Waveout(WaveoutModifier)
}

pub struct ModifierParameters {
    pub time:Instant
}

pub enum ModifierResult {
    Continue,
    RemoveThisModifier,
    RemoveAllModifiers
}

pub trait ModifierChainable {
    fn run(&mut self, pixel_strip:&mut PixelStrip, params:&ModifierParameters)->ModifierResult;
}

impl ModifierChainable for PixelModifier
{
    fn run(&mut self, pixel_strip:&mut PixelStrip, params:&ModifierParameters)->ModifierResult
    {
        match self
        {
            PixelModifier::RainbowOscillation(rainbow_oscillation_modifier) => rainbow_oscillation_modifier.run(pixel_strip,params),
            PixelModifier::Waveout(waveout_modifier) => waveout_modifier.run(pixel_strip,params),
        }
    }
}

impl PixelModifier
{
    pub fn new_rainbow_oscillation(period:u32) -> PixelModifier
    {
        PixelModifier::RainbowOscillation(
            RainbowOscillationModifier::new(TemporalOscillation::new(Instant::now(),period))
        )
    }
}