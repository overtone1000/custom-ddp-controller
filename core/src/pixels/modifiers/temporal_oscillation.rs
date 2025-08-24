use std::time::{Duration, Instant};

pub struct TemporalOscillation
{
    start:Instant,
    period_in_milliseconds_u128:u128,
    period_in_milliseconds_f64:f64
}

impl TemporalOscillation
{
    pub fn new(start:Instant, period_in_milliseconds_u32:u32)->TemporalOscillation
    {
        TemporalOscillation{
            start,
            period_in_milliseconds_u128:u128::from(period_in_milliseconds_u32),
            period_in_milliseconds_f64:f64::from(period_in_milliseconds_u32)
        }
    }

    pub fn get_rotation_at_time(&self, time:&Instant)->f64
    {
        let difference = match time.checked_duration_since(self.start){
            Some(diff) => diff,
            None => {
                match self.start.checked_duration_since(*time)
                {
                    Some(diff)=>diff,
                    None=>{
                        eprintln!("Couldn't calculate duration. Returning 0 length.");
                        Duration::new(0,0)
                    }
                }
            }
        };

        ((difference.as_millis() % self.period_in_milliseconds_u128) as f64) / self.period_in_milliseconds_f64
    }
}