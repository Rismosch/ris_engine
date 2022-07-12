use ris_data::frame_buffer::FrameBuffer;
use ris_input::input::Input;
use ris_sdl::video::Video;
use sdl2::EventPump;

pub struct GlobalContainer {
    pub _video: Video,
    pub event_pump: EventPump,
    pub frame_buffer: FrameBuffer,
    pub input: Input,
}

pub fn bootstrap() -> Result<GlobalContainer, String> {
    let sdl_context = sdl2::init()?;

    let _video = Video::new(&sdl_context)?;
    let event_pump = sdl_context.event_pump()?;

    let frame_buffer = FrameBuffer::new(4);

    let input = Input::new(&sdl_context)?;

    let global_container = GlobalContainer {
        _video,
        event_pump,
        frame_buffer,
        input,
    };

    Ok(global_container)
}
