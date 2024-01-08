use std::time::Duration;
use std::time::Instant;

const FRAME_COUNT: usize = 32;
const MAX_DURATION: Duration = Duration::from_millis((1000. / 2.) as u64); // 2 fps
const IDEAL_DURATION: Duration = Duration::from_millis((1000. / 60.) as u64); // 60 fps

pub struct FrameCalculator {
    number: usize,
    instants: [Instant; FRAME_COUNT],
}

#[derive(Copy, Clone)]
pub struct Frame {
    number: usize,
    prev: Duration,
    avg: Duration,
}

impl Default for FrameCalculator {
    fn default() -> Self {
        Self {
            number: 0,
            instants: [Instant::now(); FRAME_COUNT],
        }
    }
}

impl FrameCalculator {
    pub fn bump_and_create_frame(&mut self) -> Frame {
        let current_index = self.number % FRAME_COUNT;
        let prev_index = self.number.wrapping_sub(1) % FRAME_COUNT;
        let last_index = self.number.wrapping_add(1) % FRAME_COUNT;

        self.instants[current_index] = Instant::now();
        self.number = self.number.wrapping_add(1);

        let current_instant = self.instants[current_index];
        let prev_instant = self.instants[prev_index];
        let last_instant = self.instants[last_index];

        let prev = current_instant - prev_instant;
        let avg = (current_instant - last_instant).div_f32(FRAME_COUNT as f32);

        let prev = if prev > MAX_DURATION {
            IDEAL_DURATION
        } else {
            prev
        };

        let avg = if avg > MAX_DURATION {
            IDEAL_DURATION
        } else {
            avg
        };

        let number = self.number;
        Frame { number, prev, avg }
    }
}

impl Frame {
    pub fn number(&self) -> usize {
        self.number
    }

    pub fn prev_duration(&self) -> Duration {
        self.prev
    }

    pub fn prev(&self) -> f32 {
        self.prev_duration().as_secs_f32()
    }

    pub fn prev_fps(&self) -> f32 {
        1. / self.prev()
    }

    pub fn avg_duration(&self) -> Duration {
        self.avg
    }

    pub fn avg(&self) -> f32 {
        self.avg_duration().as_secs_f32()
    }

    pub fn avg_fps(&self) -> f32 {
        1. / self.avg()
    }
}
