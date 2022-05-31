use crate::gameloop;
use ris_data::frame_buffer::FrameBuffer;
use ris_rng::rng::Rng;

pub struct Engine{
    frame_buffer: FrameBuffer,
}

impl Engine{
    pub fn new() -> Result<Engine, Box<dyn std::error::Error>> {
        let frame_buffer = FrameBuffer::new(4);
        let rng = Rng::new();

        unsafe {
            ris_sdl::init()?;
    
            // ris_input::init();
        }

        let engine = Engine{frame_buffer};

        Ok(engine)
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>>{
        gameloop::run()
    }
}