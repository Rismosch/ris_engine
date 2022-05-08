use std::io::Write;
use std::thread;
use std::time::Instant;

use crate::frame_buffer;

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
    thread::sleep(frame_buffer::IDEAL_DELTA);
    let previous = frame_buffer::get(3);

    let test = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    println!("{:#0x}",test);

    // let mut test = ris_rng::pcg::PCG32::seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

    let mut file = std::fs::File::create("numbers.csv").unwrap();

    for i in 0..1_000 {
        let random_number = rng::next();
        let to_write = format!("{}; ", random_number);
        file.write(to_write.as_bytes());
        println!("{}", to_write);
    }

    return false;

    println!(
        "{}\t{}\t{}",
        previous.number(),
        previous.delta().as_millis(),
        frame_buffer::delta().as_millis(),
    );

    true
}
