use std::{time::{Instant, Duration}, thread};

extern crate sdl2;
use sdl2::event::Event;

use ris_data::{frame_buffer};

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

    thread::sleep(Duration::from_millis(50));

    for event in ris_sdl::event_pump::poll_iter() {
        // println!("{:?}",event);

        if let Event::Quit { .. } = event {
            return false;
        };
    }

    ris_input::keyboard::update();

    let num1 = ris_input::keyboard::hold(sdl2::keyboard::Scancode::Kp1);
    let num2 = ris_input::keyboard::hold(sdl2::keyboard::Scancode::Kp2);
    let num3 = ris_input::keyboard::hold(sdl2::keyboard::Scancode::Kp3);
    let num4 = ris_input::keyboard::hold(sdl2::keyboard::Scancode::Kp4);

    let frame = frame_buffer::get(1);
    let mut fps = frame.fps;
    if frame_buffer::count() % 1000 == 0{
        fps = 1_000_000_000 / frame_buffer::delta().as_nanos();
    }
    
    frame_buffer::get_mut(0).fps = fps;

    println!(
        "{}\t{}\t{}\t{}\t{}",
        num1,
        num2,
        num3,
        num4,
        fps
    );

    true
}
