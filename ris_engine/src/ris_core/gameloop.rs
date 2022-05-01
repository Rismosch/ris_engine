use std::time::{Duration, Instant};
use std::thread;

use crate::ris_core::*;

pub fn run(frame_buffer_lenght: usize) -> Result<(), Box<dyn std::error::Error>> {
    let max_index = frame_buffer_lenght - 1;

    let mut count = 0;
    let mut index = 0;
    
    frame_buffer::init(frame_buffer_lenght);

    loop {
        let now = Instant::now();

        // game logic here
        thread::sleep(frame::IDEAL_DELTA);

        let delta = now.elapsed();

        let last_frame = frame::Frame::new(delta, count);

        println!(
            "{} {}",
            last_frame.delta.as_millis(),
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
