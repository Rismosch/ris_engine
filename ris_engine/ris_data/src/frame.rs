use std::{time::Duration};

const MAX_DELTA: Duration = Duration::from_secs(1);
pub const IDEAL_DELTA: Duration = Duration::from_millis(1000 / 60);

pub struct Frame {
    delta: Duration,
    number: usize,
    
    pub fps: u128,
}

impl Frame {
    pub fn new(delta: Duration, number: usize) -> Frame {
        let delta = calculate_delta(delta);

        let fps = 0;
        Frame { delta, number, fps }
    }

    pub fn delta(&self) -> Duration {
        self.delta
    }

    pub fn number(&self) -> usize {
        self.number
    }

    pub fn set(&mut self, delta: Duration, number: usize) {
        let delta = calculate_delta(delta);
        self.delta = delta;
        self.number = number;
    }
}

fn calculate_delta(delta: Duration) -> Duration {
    if delta > MAX_DELTA {
        IDEAL_DELTA
    } else {
        delta
    }
}
