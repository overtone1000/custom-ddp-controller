use std::time::{Duration, Instant};

pub struct TemporalProgression
{
    start:Instant,
    duration:Duration
}

impl TemporalProgression
{
    pub fn new(start:Instant, duration:Duration)->TemporalProgression
    {
        TemporalProgression{
            start,
            duration
        }
    }

    pub fn get_progress_at_time(&self, time:&Instant)->f64
    {
        let difference = time.saturating_duration_since(self.start);
        let ratio = difference.div_duration_f64(self.duration);
        ratio.clamp(0.0,1.0)
    }
}