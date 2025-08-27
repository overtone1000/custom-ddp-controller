use std::time::Instant;

use fadeout::{curtain::CurtainModifier, dropout::DropoutModifier};

use super::pixelstrip::PixelStrip;

use display::rainbow_oscillation::RainbowOscillationModifier;

mod demos;
pub mod display;
pub mod fadeout;
pub mod temporal_oscillation;
pub mod temporal_progression;

pub const TWO_PI: f64 = std::f64::consts::PI * 2.0;

pub enum PixelModifier {
    RainbowOscillation(RainbowOscillationModifier),
    Curtain(CurtainModifier),
    Dropout(DropoutModifier)
}

pub struct ModifierParameters {
    pub time: Instant,
}

pub enum ModifierResult {
    Continue,
    RemoveThisModifier,
    RemoveAllModifiers,
}

pub trait ModifierChainable {
    fn run(&mut self, pixel_strip: &mut PixelStrip, params: &ModifierParameters) -> ModifierResult;
}

impl ModifierChainable for PixelModifier {
    fn run(&mut self, pixel_strip: &mut PixelStrip, params: &ModifierParameters) -> ModifierResult {
        match self {
            PixelModifier::RainbowOscillation(modifier) => {modifier.run(pixel_strip, params)}
            PixelModifier::Curtain(modifier) => modifier.run(pixel_strip, params),
            PixelModifier::Dropout(modifier) => modifier.run(pixel_strip, params),
        }
    }
}

impl PixelModifier {
    pub fn new_rainbow_oscillation() -> PixelModifier {
        PixelModifier::RainbowOscillation(RainbowOscillationModifier::new())
    }
}
