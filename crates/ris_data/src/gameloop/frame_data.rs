use std::time::{Duration, Instant};

const MAX_DELTA: Duration = Duration::from_millis(500);
pub const IDEAL_DELTA: Duration = Duration::from_millis(1000 / 60);

#[derive(Clone, Copy)]
pub struct FrameData {
    number: usize,
    delta: Duration,
    last_bump: Instant,
}

impl FrameData {
    pub fn new() -> Self {
        FrameData {
            last_bump: Instant::now(),
            delta: Duration::ZERO,
            number: 0,
        }
    }

    pub fn bump(&mut self) {
        self.number += 1;
        let now = Instant::now();
        self.delta = calculate_delta(now - self.last_bump);
        self.last_bump = now;
    }

    pub fn number(&self) -> usize {
        self.number
    }

    pub fn delta(&self) -> Duration {
        self.delta
    }

    pub fn fps(&self) -> u128 {
        1_000_000_000 / self.delta.as_nanos()
    }
}

fn calculate_delta(delta: Duration) -> Duration {
    if delta > MAX_DELTA {
        IDEAL_DELTA
    } else {
        delta
    }
}