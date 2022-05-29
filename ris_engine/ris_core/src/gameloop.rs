use std::{
    thread,
    time::{Duration, Instant},
};

use ris_sdl::event_pump;

use ris_data::frame_buffer;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let now = Instant::now();

        event_pump::poll_all_events();

        ris_input::keyboard::update();
        ris_input::mouse::update();

        game_logic();

        let delta = now.elapsed();
        unsafe {
            frame_buffer::add(delta);
        }

        if event_pump::quit_was_called() {
            break;
        }
    }

    Ok(())
}

fn game_logic() {
    thread::sleep(Duration::from_millis(50));
}
