use std::rc::Rc;

use crate::gameloop::GameLoop;
use ris_data::frame_buffer::FrameBuffer;
use ris_input::mouse::Mouse;
use ris_sdl::{event_pump::EventPump, video::Video};

pub struct Engine {
    game_loop: GameLoop<EventPump>,
    _video: Video,
    _mouse: Rc<Mouse>,
}

impl Engine {
    pub fn new() -> Result<Engine, Box<dyn std::error::Error>> {
        let sdl_context = sdl2::init()?;

        let video = Video::new(&sdl_context)?;
        let mut event_pump = EventPump::new(&sdl_context)?;

        let frame_buffer = FrameBuffer::new(4);

        let mouse = Mouse::new(&mut event_pump);
        let game_loop = GameLoop::new(event_pump, frame_buffer);

        let engine = Engine {
            game_loop,
            _video: video,
            _mouse: mouse,
        };

        Ok(engine)
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.game_loop.run()
    }
}
