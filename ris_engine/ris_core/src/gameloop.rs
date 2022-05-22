use std::{time::Instant, thread};

extern crate sdl2;
use sdl2::event::Event;

use ris_data::*;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // let mut event_pump = ris_data::sdl2::context().event_pump()?;

    loop {
        let now = Instant::now();

        let running = game_logic(/* &mut event_pump */);

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

fn game_logic(/* event_pump: &mut EventPump */) -> bool {
    thread::sleep(std::time::Duration::from_millis(50));

    // Service any and all pending Windows messages.
    for event in ris_sdl::event_pump::poll_iter() {
        if let Event::Quit { .. } = event {
            return false;
        };
    }

    ris_input::keyboard::update();

    let key = sdl2::keyboard::Scancode::L;
    let down = ris_input::keyboard::down(key);
    let up = ris_input::keyboard::up(key);
    let hold = ris_input::keyboard::hold(key);

    println!("{}\t{}\t{}\t{}", down, up, hold, ris_data::frame_buffer::delta().as_millis());

    // Create a set of pressed Keys.
    // let keys = events
    //     .keyboard_state()
    //     .pressed_scancodes()
    //     .filter_map(Keycode::from_scancode)
    //     .collect();

    // // Get the difference between the new and old sets.
    // let new_keys = &keys - &prev_keys;
    // let old_keys = &prev_keys - &keys;

    // if !new_keys.is_empty() || !old_keys.is_empty() {
    //     println!("new_keys: {:?}\told_keys:{:?}", new_keys, old_keys);
    // }

    // prev_keys = keys;

    true
}
