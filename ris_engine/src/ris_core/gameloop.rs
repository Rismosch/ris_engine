use std::time::{Duration, Instant};
use std::thread;

use crate::ris_core::*;

pub fn run(frame_buffer_lenght: usize) -> Result<(), Box<dyn std::error::Error>> {
    let max_index = frame_buffer_lenght - 1;

    let mut count = 0;
    let mut index = 0;

    let mut frame_buffer = Vec::with_capacity(frame_buffer_lenght);

    let test = (0 - (frame_buffer_lenght as isize)) as usize;
    for i in 0..frame_buffer_lenght {
        frame_buffer.push(frame::Frame::new(frame::IDEAL_DELTA, i, test + i));
    }

    loop {
        let now = Instant::now();

        // game logic here
        thread::sleep(frame::IDEAL_DELTA);

        let delta = now.elapsed();

        let last_frame = frame::Frame::new(delta, index, count);

        println!(
            "{} {} {}",
            last_frame.delta.as_millis(),
            last_frame.index,
            last_frame.number
        );

        count += 1;

        if index >= max_index {
            index = 0;
        } else {
            index += 1;
        }
    }

    return Ok(());
}
