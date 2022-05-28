use std::{
    thread,
    time::{Duration, Instant},
};

extern crate sdl2;
use ris_sdl::event_pump;
use sdl2::event::Event;

use ris_data::frame_buffer;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let now = Instant::now();

        ris_input::keyboard::update();
        ris_input::mouse::update();

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
    thread::sleep(Duration::from_millis(50));

    for event in ris_sdl::event_pump::poll_iter() {
        println!("{:?}",event);

        if let Event::Quit { .. } = event {
            return false;
        };
    }

    // println!("{}\t", frame_buffer::fps());

    true
}
