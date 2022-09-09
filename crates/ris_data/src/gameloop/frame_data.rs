use std::time::{Duration, Instant};

const DELTAS_COUNT: usize = 4;
const MAX_DELTA: Duration = Duration::from_millis(500);
const IDEAL_DELTA: Duration = Duration::from_millis(1000 / 60);

pub struct FrameData {
    number: usize,
    deltas: [Duration; DELTAS_COUNT],
    delta: Duration,
    last_bump: Instant,
}

impl FrameData {
    pub fn bump(&mut self) {
        self.number += 1;

        let now = Instant::now();

        let current_delta = now - self.last_bump;
        let delta_to_set = if current_delta > MAX_DELTA {
            IDEAL_DELTA
        } else {
            current_delta
        };

        let index = self.number % DELTAS_COUNT;
        self.deltas[index] = delta_to_set;

        let mut nanos_sum = 0;
        for delta in self.deltas {
            nanos_sum += delta.as_nanos();
        }
        let nanos_average = nanos_sum / DELTAS_COUNT as u128;
        self.delta = Duration::from_nanos(nanos_average as u64);

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

impl Default for FrameData {
    fn default() -> Self {
        FrameData {
            last_bump: Instant::now(),
            deltas: [IDEAL_DELTA; DELTAS_COUNT],
            delta: Duration::ZERO,
            number: 0,
        }
    }
}
