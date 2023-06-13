use std::time::{Duration, Instant};

const DELTAS_COUNT: usize = 1024;
const MAX_DELTA: Duration = Duration::from_millis(500);
const IDEAL_DELTA: Duration = Duration::from_millis(1000 / 60);

#[derive(Default)]
pub struct FrameDataCalculator{
    current: FrameData,
}

#[derive(Clone)]
pub struct FrameData {
    number: usize,
    all_deltas: [Duration; DELTAS_COUNT],
    delta: Duration,
    last_bump: Instant,
}

impl FrameDataCalculator{
    pub fn bump(&mut self){
        let current = &mut self.current;
        current.number += 1;

        let now = Instant::now();

        let current_delta = now - current.last_bump;
        let delta_to_set = if current_delta > MAX_DELTA {
            IDEAL_DELTA
        } else {
            current_delta
        };

        let index = current.number % DELTAS_COUNT;
        current.all_deltas[index] = delta_to_set;

        let mut nanos_sum = 0;
        for delta in current.all_deltas {
            nanos_sum += delta.as_nanos();
        }
        let nanos_average = nanos_sum / DELTAS_COUNT as u128;
        current.delta = Duration::from_nanos(nanos_average as u64);

        current.last_bump = now;
    }

    pub fn current(&self) -> &FrameData{
        &self.current
    }
}

impl FrameData {
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
            all_deltas: [IDEAL_DELTA; DELTAS_COUNT],
            delta: Duration::ZERO,
            number: 0,
        }
    }
}
