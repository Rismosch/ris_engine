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
    previous: Duration,
    average: Duration,
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

        let previous = current_instant - prev_instant;
        let average = (current_instant - last_instant).div_f32(FRAME_COUNT as f32);

        let previous = if previous > MAX_DURATION {
            IDEAL_DURATION
        } else {
            previous
        };

        let average = if average > MAX_DURATION {
            IDEAL_DURATION
        } else {
            average
        };

        let number = self.number;
        Frame { number, previous, average }
    }
}

impl Frame {
    pub fn number(&self) -> usize {
        self.number
    }

    pub fn previous_duration(&self) -> Duration {
        self.previous
    }

    pub fn previous_seconds(&self) -> f32 {
        self.previous_duration().as_secs_f32()
    }

    pub fn previous_fps(&self) -> f32 {
        1. / self.previous_seconds()
    }

    pub fn average_duration(&self) -> Duration {
        self.average
    }

    pub fn average_seconds(&self) -> f32 {
        self.average_duration().as_secs_f32()
    }

    pub fn average_fps(&self) -> f32 {
        1. / self.average_seconds()
    }
}
