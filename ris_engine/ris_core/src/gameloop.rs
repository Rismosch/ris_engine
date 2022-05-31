use std::{
    thread,
    time::{Duration, Instant},
};

use ris_sdl::event_pump::EventPump;

use ris_data::frame_buffer::FrameBuffer;

pub struct GameLoop {
    event_pump: EventPump,
    frame_buffer: FrameBuffer,
}

impl GameLoop {
    pub fn new(event_pump: EventPump, frame_buffer: FrameBuffer) -> GameLoop {
        GameLoop {
            event_pump,
            frame_buffer,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let now = Instant::now();

            self.event_pump.pump();

            // ris_input::keyboard::update();
            // ris_input::mouse::update();

            self.game_logic();

            let delta = now.elapsed();

            self.frame_buffer.add(delta);

            if self.event_pump.wants_to_quit {
                break;
            }
        }

        Ok(())
    }

    fn game_logic(&self) {
        thread::sleep(Duration::from_millis(50));
    }
}
