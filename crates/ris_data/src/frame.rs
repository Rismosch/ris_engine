use std::time::Duration;

const MAX_DELTA: Duration = Duration::from_millis(500);
pub const IDEAL_DELTA: Duration = Duration::from_millis(1000 / 60);

pub struct Frame {
    delta: Duration,
    number: usize,
}

impl Frame {
    pub fn new(delta: Duration, number: usize) -> Frame {
        let delta = calculate_delta(delta);

        Frame { delta, number }
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
