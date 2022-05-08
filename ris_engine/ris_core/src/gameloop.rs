use std::thread;
use std::time::Instant;

use ris_data::*;
use ris_rng::rng;

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
    thread::sleep(frame::IDEAL_DELTA);
    let previous = frame_buffer::get(3);

    println!(
        "{}\t{}\t{}\t{}",
        previous.number(),
        previous.delta().as_millis(),
        frame_buffer::delta().as_millis(),
        rng::range_i(0, 9),
    );

    true
}
