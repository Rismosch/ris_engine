use crate::gameloop::GameLoop;
use ris_data::frame_buffer::FrameBuffer;
use ris_sdl::{event_pump::EventPump, video::Video};

pub struct Engine {
    game_loop: GameLoop,
    _video: Video,
}

impl Engine {
    pub fn new() -> Result<Engine, Box<dyn std::error::Error>> {
        let sdl_context = sdl2::init()?;

        let video = Video::new(&sdl_context)?;
        let event_pump = EventPump::new(&sdl_context)?;

        let frame_buffer = FrameBuffer::new(4);

        let game_loop = GameLoop::new(event_pump, frame_buffer);

        let engine = Engine {
            game_loop,
            _video: video,
        };

        Ok(engine)
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.game_loop.run()
    }
}
