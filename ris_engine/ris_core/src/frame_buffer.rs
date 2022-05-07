use std::time::Duration;

const MAX_DELTA: Duration = Duration::from_secs(1);
pub const IDEAL_DELTA: Duration = Duration::from_millis(1000 / 60);

static mut FRAMES: Vec<Frame> = Vec::new();
static mut FRAMES_LENGTH: usize = 0;
static mut MAX_INDEX: usize = 0;
static mut COUNT: usize = 0;
static mut INDEX: usize = 0;
static mut DELTA: Duration = IDEAL_DELTA;

pub struct Frame {
    delta: Duration,
    number: usize,
}

impl Frame {
    fn new(delta: Duration, number: usize) -> Frame {
        if delta > MAX_DELTA {
            Frame {
                delta: IDEAL_DELTA,
                number,
            }
        } else {
            Frame { delta, number }
        }
    }

    pub fn delta(&self) -> Duration {
        self.delta
    }
    pub fn number(&self) -> usize {
        self.number
    }
}

pub unsafe fn init(frame_buffer_lenght: usize) {
    FRAMES = Vec::with_capacity(frame_buffer_lenght);

    FRAMES_LENGTH = frame_buffer_lenght;
    MAX_INDEX = frame_buffer_lenght - 1;
    COUNT = 0;
    INDEX = 0;
    DELTA = IDEAL_DELTA;

    let number_offset = (0 - (frame_buffer_lenght as isize)) as usize;
    for i in 0..frame_buffer_lenght {
        let frame = Frame::new(IDEAL_DELTA, number_offset + i);
        FRAMES.push(frame);
    }
}

pub unsafe fn add(delta: Duration) {
    let mut frame = &mut FRAMES[INDEX];
    frame.delta = delta;
    frame.number = COUNT;

    COUNT += 1;

    if INDEX >= MAX_INDEX {
        INDEX = 0;
    } else {
        INDEX += 1;
    }

    calculate_delta();
}

pub fn get(offset: usize) -> &'static Frame {
    unsafe {
        let previous_index = INDEX as isize;
        let offset = 1 + offset as isize;

        let index = if previous_index < offset {
            previous_index - offset + FRAMES_LENGTH as isize
        } else {
            previous_index - offset
        };

        &FRAMES[index as usize]
    }
}

pub fn delta() -> Duration {
    unsafe { DELTA }
}

fn calculate_delta() {
    unsafe {
        let mut sum = Duration::ZERO;
        for frame in FRAMES.iter() {
            sum += frame.delta;
        }

        DELTA = sum / FRAMES_LENGTH as u32;
    }
}
