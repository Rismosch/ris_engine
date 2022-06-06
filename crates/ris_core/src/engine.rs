use std::{
    thread,
    time::{Duration, Instant},
};

use ris_data::frame_buffer::FrameBuffer;
use ris_input::{
    buttons::IButtons,
    input::{IInput, Input},
};
use ris_sdl::video::Video;

use sdl2::event::Event;
use sdl2::EventPump;

pub struct Engine {
    _video: Video,
    event_pump: EventPump,
    frame_buffer: FrameBuffer,
    input: Input,
}

impl Engine {
    pub fn new() -> Result<Engine, String> {
        let sdl_context = sdl2::init()?;

        let _video = Video::new(&sdl_context)?;
        let event_pump = sdl_context.event_pump()?;

        let frame_buffer = FrameBuffer::new(4);

        let input = Input::new(&sdl_context)?;

        let engine = Engine {
            _video,
            event_pump,
            frame_buffer,
            input,
        };

        Ok(engine)
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            let now = Instant::now();

            let pump_wants_to_quit = self.pump_events();
            let game_wants_to_quit = self.game_logic();

            let delta = now.elapsed();

            self.frame_buffer.add(delta);

            if pump_wants_to_quit || game_wants_to_quit {
                break;
            }
        }

        Ok(())
    }

    fn pump_events(&mut self) -> bool {
        self.input.pre_update();

        for event in self.event_pump.poll_iter() {
            // println!("{:?}", event);

            if let Event::Quit { .. } = event {
                return true;
            };

            self.input.update(&event);
        }

        self.input.post_update(&self.event_pump);

        false
    }

    fn game_logic(&mut self) -> bool {
        thread::sleep(Duration::from_millis(50));
        // println!("{}", self.frame_buffer.fps());

        println!("{:#034b}", self.input.general().buttons().hold());

        false
    }
}
