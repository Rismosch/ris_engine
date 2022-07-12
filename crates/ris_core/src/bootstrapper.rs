use ris_data::{frame_buffer::FrameBuffer, info::runtime_info::{RuntimeInfo, runtime_info}};
use ris_input::input::Input;
use ris_video::video::Video;
use sdl2::EventPump;

pub struct GlobalContainer {
    pub _video: Video,
    pub event_pump: EventPump,
    pub frame_buffer: FrameBuffer,
    pub input: Input,
    pub runtime_info: RuntimeInfo,
}

pub fn bootstrap() -> Result<GlobalContainer, String> {
    let sdl_context = sdl2::init()?;

    let _video = Video::new(&sdl_context)?;
    let event_pump = sdl_context.event_pump()?;

    let frame_buffer = FrameBuffer::new(4);

    let input = Input::new(&sdl_context)?;

    let runtime_info = runtime_info();

    let global_container = GlobalContainer {
        _video,
        event_pump,
        frame_buffer,
        input,
        runtime_info,
    };

    Ok(global_container)
}
