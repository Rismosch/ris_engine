use std::time::Duration;

use crate::frame::*;

static mut FRAMES: Vec<Frame> = Vec::new();
static mut FRAMES_LENGTH: usize = 0;
static mut MAX_INDEX: usize = 0;
static mut COUNT: usize = 0;
static mut INDEX: usize = 0;

static mut DELTA: Duration = IDEAL_DELTA;

/// # Safety
/// Should only be called by the main thread.
/// This method modifies global static variables, and thus is inherently unsafe.
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

/// # Safety
/// Should only be called by the main thread.
/// This method modifies global static variables, and thus is inherently unsafe.
pub unsafe fn add(delta: Duration) {
    let frame = &mut FRAMES[INDEX];

    let number = COUNT;
    frame.set(delta, number);

    COUNT += 1;

    if INDEX >= MAX_INDEX {
        INDEX = 0;
    } else {
        INDEX += 1;
    }

    calculate_durations();
}

pub fn count() -> usize {
    unsafe { COUNT }
}

pub fn get(offset: usize) -> &'static Frame {
    let index = get_index(offset);

    unsafe { &FRAMES[index as usize] }
}

pub fn get_mut(offset: usize) -> &'static mut Frame {
    let index = get_index(offset);

    unsafe { &mut FRAMES[index as usize] }
}

pub fn delta() -> Duration {
    unsafe { DELTA }
}

fn calculate_durations() {
    unsafe {
        let mut sum = Duration::ZERO;
        for frame in FRAMES.iter() {
            sum += frame.delta();
        }

        DELTA = sum / FRAMES_LENGTH as u32;
    }
}

fn get_index(offset: usize) -> isize {
    let previous_index = unsafe { INDEX } as isize;

    let offset = 1 + offset as isize;

    if previous_index < offset {
        previous_index - offset + unsafe { FRAMES_LENGTH } as isize
    } else {
        previous_index - offset
    }
}
