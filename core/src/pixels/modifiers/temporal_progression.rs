use std::time::{Duration, Instant};

pub struct TemporalProgression {
    start: Instant,
    duration: Duration,
    end: Option<Instant>,
}

#[derive(Debug)]
pub enum TemporalProgressionStatus {
    NotYetStarted,
    Running(f64),
    Finished,
}

impl TemporalProgression {
    pub fn new(start: Instant, duration: Duration) -> TemporalProgression {
        let end = start.checked_add(duration);

        TemporalProgression {
            start,
            duration,
            end,
        }
    }

    pub fn get_progress_at_time(&self, time: &Instant) -> TemporalProgressionStatus {
        if self.end.is_some_and(|end| time > &end) {
            TemporalProgressionStatus::Finished
        } else if time < &self.start {
            TemporalProgressionStatus::NotYetStarted
        } else {
            let difference = time.saturating_duration_since(self.start);
            let ratio = difference.div_duration_f64(self.duration);
            let ratio = ratio.clamp(0.0, 1.0);
            TemporalProgressionStatus::Running(ratio)
        }
    }
}
