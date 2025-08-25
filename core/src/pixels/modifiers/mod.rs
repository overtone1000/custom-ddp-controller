use std::time::Instant;

use fadeout::curtain::CurtainModifier;

use super::pixelstrip::PixelStrip;

use display::rainbow_oscillation::RainbowOscillationModifier;

mod demos;
pub mod display;
pub mod fadeout;
mod temporal_oscillation;
mod temporal_progression;

pub const TWO_PI:f64=std::f64::consts::PI*2.0;

pub enum PixelModifier
{
    RainbowOscillation(RainbowOscillationModifier),
    Curtain(CurtainModifier)
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
            PixelModifier::Curtain(curtain_modifier) => curtain_modifier.run(pixel_strip,params),
        }
    }
}

impl PixelModifier
{
    pub fn new_rainbow_oscillation() -> PixelModifier
    {
        PixelModifier::RainbowOscillation(
            RainbowOscillationModifier::new()
        )
    }
}