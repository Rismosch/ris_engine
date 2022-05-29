use std::{
    thread,
    time::{Duration, Instant},
};

use ris_sdl::event_pump;

use ris_data::frame_buffer;
use sdl2::keyboard::Scancode;

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

    let mouse_1 = 0;
    let mouse_2 = 1;
    let mouse_3 = 2;

    let key_1 = Scancode::Kp1;
    let key_2 = Scancode::Kp2;
    let key_3 = Scancode::Kp3;

    println!("{}\t{}\t{}\t{}\t{}\t{}\t{}",
        ris_input::keyboard::hold(key_1),
        ris_input::keyboard::hold(key_2),
        ris_input::keyboard::hold(key_3),
        ris_input::mouse::hold(mouse_1),
        ris_input::mouse::hold(mouse_2),
        ris_input::mouse::hold(mouse_3),
        frame_buffer::fps()
    )
    
}
