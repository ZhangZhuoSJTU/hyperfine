use std::time::Instant;

use crate::util::units::Second;

pub struct WallClockTimer {
    start: Instant,
}

impl WallClockTimer {
    pub fn start() -> WallClockTimer {
        WallClockTimer {
            start: Instant::now(),
        }
    }

    pub fn stop(&self) -> Second {
        let duration = self.start.elapsed();
        let whole_seconds = duration.as_secs() as f64;
        let fractional_seconds = f64::from(duration.subsec_nanos()) * 1e-9;
        whole_seconds + fractional_seconds
    }
}
