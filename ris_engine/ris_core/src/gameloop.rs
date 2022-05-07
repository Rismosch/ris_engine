use std::thread;
use std::time::Instant;

use crate::frame_buffer;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let now = Instant::now();

        let running = game_logic();

        let delta = now.elapsed();
        unsafe {
            frame_buffer::add(delta);
        }

        if !running {
            break;
        }
    }

    Ok(())
}

fn game_logic() -> bool {
    thread::sleep(frame_buffer::IDEAL_DELTA);
    let previous = frame_buffer::get(3);

    println!(
        "{}\t{}\t{}",
        previous.number(),
        previous.delta().as_millis(),
        frame_buffer::delta().as_millis(),
    );

    true
}
