use std::{
    f64::consts::PI,
    time::{Duration, Instant},
};

use angular_units::Deg;

use crate::pixels::{
    modifiers::{
        temporal_progression::{TemporalProgression, TemporalProgressionStatus},
        ModifierChainable, ModifierResult,
    },
    pixel::{self, HSV},
};

pub struct CurtainModifier {
    progress: TemporalProgression,
    curtain_edge_width: f64,
    pixel_sets: Vec<(usize, usize)>,
}

impl CurtainModifier {
    pub fn new(
        start: Instant,
        duration: Duration,
        curtain_edge_width: f64,
        pixel_sets: Vec<(usize, usize)>,
    ) -> CurtainModifier {
        let progress = TemporalProgression::new(start, duration);
        CurtainModifier {
            progress,
            curtain_edge_width,
            pixel_sets: pixel_sets,
        }
    }
}

impl ModifierChainable for CurtainModifier {
    fn run(
        &mut self,
        pixel_strip: &mut crate::pixels::pixelstrip::PixelStrip,
        params: &crate::pixels::modifiers::ModifierParameters,
    ) -> crate::pixels::modifiers::ModifierResult {
        let progress: TemporalProgressionStatus = self.progress.get_progress_at_time(&params.time);

        match progress {
            TemporalProgressionStatus::Running(progress) => {
                let inverse_progress = 1.0 - progress;
                for pixel_set in &self.pixel_sets {
                    //Curtain starts with the front of the "edge" region (a sinusoidal fade to black) at the start of the set of pixels
                    //Curtain ends with the back of the "edge" region to the end of the set of pixels
                    //So, its location relative to a set of pixels is dependent on the number of pixels
                    //Calculate parameters of interest here.

                    let is_inverted = pixel_set.1 < pixel_set.0;
                    let pixel_count_f64 =
                        f64::from(((pixel_set.1.abs_diff(pixel_set.0)) + 1) as u32);
                    let curtain_edge_front = progress * (pixel_count_f64 + self.curtain_edge_width);
                    let curtain_edge_back =
                        progress * pixel_count_f64 - inverse_progress * self.curtain_edge_width;

                    let range = if is_inverted {
                        pixel_set.1..=pixel_set.0
                    } else {
                        pixel_set.0..=pixel_set.1
                    };

                    for a in range {
                        let location = f64::from(a as u32);

                        //If the pixels are inverted then invert location
                        let location = if is_inverted {
                            pixel_count_f64 - location
                        } else {
                            location
                        };

                        if location > curtain_edge_front {
                            //do nothing! curtain hasn't reached pixel yet
                        } else if location < curtain_edge_back {
                            //just turn the pixel off
                            pixel_strip.set_pixel_hsv(a, HSV::new(Deg(0.0), 0.0, 0.0));
                        } else {
                            match pixel_strip.get_pixel_hsv(a) {
                                Some(current) => {
                                    let brightness_coefficient =
                                        (location - curtain_edge_back) / self.curtain_edge_width;

                                    let brightness_coefficient = brightness_coefficient.powi(2);

                                    //(((curtain_edge_front
                                    //   - location / self.curtain_edge_width)
                                    //   * PI)
                                    //   .cos()
                                    //   + 1.0)
                                    //   / 2.0;

                                    let mut new_hsv = current.clone();
                                    new_hsv.set_value(current.value() * brightness_coefficient);

                                    pixel_strip.set_pixel_hsv(a, new_hsv);
                                }
                                None => {
                                    eprintln!("Pixel access error.");
                                }
                            }
                        }
                    }
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
