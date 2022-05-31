use std::time::Duration;

use crate::frame::*;

pub struct FrameBuffer {
    frames: Vec<Frame>,
    frames_length: usize,
    max_index: usize,
    index: usize,
    count: usize,
    delta: Duration,
}

impl FrameBuffer {
    pub fn new(frame_buffer_lenght: usize) -> FrameBuffer {
        let mut frame_buffer = FrameBuffer {
            frames: Vec::with_capacity(frame_buffer_lenght),
            frames_length: frame_buffer_lenght,
            max_index: frame_buffer_lenght - 1,
            index: 0,
            count: 0,
            delta: IDEAL_DELTA,
        };

        let number_offset = (0 - (frame_buffer_lenght as isize)) as usize;
        for i in 0..frame_buffer_lenght {
            let frame = Frame::new(IDEAL_DELTA, number_offset + i);
            frame_buffer.frames.push(frame);
        }

        frame_buffer
    }

    pub fn add(&mut self, delta: Duration) {
        let frame = &mut self.frames[self.index];

        let number = self.count;
        frame.set(delta, number);

        self.count += 1;

        if self.index >= self.max_index {
            self.index = 0;
        } else {
            self.index += 1;
        }

        self.calculate_delta();
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn delta(&self) -> Duration {
        self.delta
    }

    pub fn fps(&self) -> u128 {
        1_000_000_000 / self.delta.as_nanos()
    }

    pub fn get(&self, offset: usize) -> &Frame {
        let index = self.get_index(offset);

        &self.frames[index as usize]
    }

    pub fn get_mut(&mut self, offset: usize) -> &mut Frame {
        let index = self.get_index(offset);

        &mut self.frames[index as usize]
    }

    fn calculate_delta(&mut self) {
        let mut sum = Duration::ZERO;
        for frame in self.frames.iter() {
            sum += frame.delta();
        }

        self.delta = sum / self.frames_length as u32;
    }

    fn get_index(&self, offset: usize) -> isize {
        let previous_index = self.index as isize;

        let offset = 1 + offset as isize;

        if previous_index < offset {
            previous_index - offset + self.frames_length as isize
        } else {
            previous_index - offset
        }
    }
}
