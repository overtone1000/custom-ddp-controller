use std::{
    collections::BTreeSet, f64::consts::PI, time::{Duration, Instant}
};

use angular_units::Deg;
use rand::seq::SliceRandom;

use crate::pixels::{
    modifiers::{
        temporal_progression::{TemporalProgression, TemporalProgressionStatus},
        ModifierChainable, ModifierResult,
    },
    pixel::{self, HSV},
};

struct InstantiatedOnFirstIteration
{
    on_pixels:Vec<usize>,
    pixel_count_f64:f64
}

pub struct DropoutModifier {
    progress:TemporalProgression,
    off_pixels:Vec<usize>,
    iof:Option<InstantiatedOnFirstIteration>,
}

impl DropoutModifier {
    pub fn new(
        start: Instant,
        duration: Duration,
    ) -> DropoutModifier {
        let progress = TemporalProgression::new(start, duration);
        DropoutModifier {
            progress,
            off_pixels:Vec::new(),
            iof:None
        }
    }
}

impl ModifierChainable for DropoutModifier {
    fn run(
        &mut self,
        pixel_strip: &mut crate::pixels::pixelstrip::PixelStrip,
        params: &crate::pixels::modifiers::ModifierParameters,
    ) -> crate::pixels::modifiers::ModifierResult {
        let progress: TemporalProgressionStatus = self.progress.get_progress_at_time(&params.time);

        if self.iof.is_none()
        {
            let mut on_pixels=Vec::new();
            for i in 0..pixel_strip.count()
            {
                on_pixels.push(i);
            }

            //Shuffle them to start
            on_pixels.shuffle(&mut rand::rng());

            self.iof=Some(
                InstantiatedOnFirstIteration { 
                    on_pixels, 
                    pixel_count_f64: f64::from(pixel_strip.count() as u32) 
                }
            );
        }

        match progress {
            TemporalProgressionStatus::Running(progress) => {

                match &mut self.iof
                {
                    Some(iof)=>{
                        let target_off_pixels = (progress*iof.pixel_count_f64).floor() as usize;
                        let pixels_to_turn_off=target_off_pixels-self.off_pixels.len();
                        
                        for _ in 0..pixels_to_turn_off
                        {
                            match iof.on_pixels.pop()
                            {
                                Some(pixel_to_turn_off)=>{
                                    self.off_pixels.push(pixel_to_turn_off);
                                }
                                None=>{
                                    eprintln!("Should be unreachable.");
                                }
                            }
                        }
                    },
                    None=>{eprintln!("This shouldn't be reachable.");}
                }

                let black = HSV::new(Deg(0.0), 0.0, 0.0);
                for pixel_index in &self.off_pixels
                {
                    pixel_strip.set_pixel_hsv(*pixel_index, black.clone());
                }

                ModifierResult::Continue
            }
            TemporalProgressionStatus::NotYetStarted => ModifierResult::Continue,
            TemporalProgressionStatus::Finished => {
                pixel_strip.all_pixels_to_black();
                ModifierResult::RemoveAllModifiers
            }
        }
    }
}
