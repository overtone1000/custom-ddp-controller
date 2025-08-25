use std::time::Instant;

use angular_units::Deg;
use prisma::channel::PosNormalChannelScalar;

use crate::pixels::{modifiers::{temporal_oscillation::TemporalOscillation, ModifierChainable, ModifierParameters, ModifierResult, TWO_PI}, pixel::HSV, pixelstrip::PixelStrip};

pub struct RainbowOscillationModifier
{
    forward_motion:TemporalOscillation,
    oscillation:TemporalOscillation
}

const OSCILLATION_PERIOD:u32 = 30000;
const FORWARD_MOTION_PERIOD:u32 = 24001; //Should be near oscillation but slightly changed

impl RainbowOscillationModifier
{
    pub fn new()->RainbowOscillationModifier
    {
        let now=Instant::now();
        let forward_motion = TemporalOscillation::new(now, FORWARD_MOTION_PERIOD);
        let oscillation = TemporalOscillation::new(now, OSCILLATION_PERIOD);
        RainbowOscillationModifier{
            forward_motion,
            oscillation
        }
    }
}

impl ModifierChainable for RainbowOscillationModifier
{
    fn run(&mut self, pixel_strip:&mut PixelStrip, params:&ModifierParameters)->ModifierResult
    {
        let forward_motion_fraction=self.forward_motion.get_fraction_at_time(&params.time);
        let oscillation_fraction=self.oscillation.get_fraction_at_time(&params.time);
        let oscillation_motion=((oscillation_fraction*TWO_PI).sin()+1.0)/2.0;
        
        let pixel_count_f64:f64=(pixel_strip.count() as u32).into();

        //Calculate a funky curve with values along both x and y ranging from 0.0 to 1.0 and map the curve to the pixel strip based on pixel location
        let funky_curve = |location_fraction:f64|
        {
            //First oscillate to avoid static nodes
            let location_fraction=(location_fraction+oscillation_motion)%1.0;
            
            //Non-linearize to stretch out colors
            let location_fraction = ((location_fraction.powi(2)*TWO_PI).cos()+1.0)/2.0;

            //Forward motion to move curve along strip and to change which color is stretched
            let location_fraction = location_fraction+forward_motion_fraction;

            //This prevents a bug with large angles. unreachable match in HSV gets called.
            location_fraction%1.0
        };

        for a in 0..pixel_strip.count() {
            let location_fraction = f64::from(a as u32) / pixel_count_f64;

            let rotation = funky_curve(location_fraction);
            //let rotation = rotation % 1.0;

            let final_angle = Deg(360.0 * rotation);

            let hue = HSV::new(final_angle, 1.0, 1.0);

            pixel_strip.set_pixel_hsv(a, hue);
        }

        ModifierResult::Continue
    }
}