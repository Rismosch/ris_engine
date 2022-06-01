use std::{
    rc::Rc,
    thread,
    time::{Duration, Instant},
};

use ris_sdl::event_pump::IEventPump;

use ris_data::frame_buffer::FrameBuffer;

pub struct GameLoop<TEventPump: IEventPump> {
    event_pump: TEventPump,
    frame_buffer: FrameBuffer,
}

impl<TEventPump: IEventPump> GameLoop<TEventPump> {
    pub fn new(event_pump: TEventPump, frame_buffer: FrameBuffer) -> GameLoop<TEventPump> {
        GameLoop {
            event_pump,
            frame_buffer,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let now = Instant::now();

            self.event_pump.pump();

            self.game_logic();

            let delta = now.elapsed();

            self.frame_buffer.add(delta);

            if self.event_pump.wants_to_quit() {
                break;
            }
        }

        Ok(())
    }

    fn game_logic(&self) {
        thread::sleep(Duration::from_millis(50));
    }
}
