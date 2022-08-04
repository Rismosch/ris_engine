use ris_data::{
    frame_buffer::FrameBuffer,
    info::{
        app_info::{app_info, AppInfo},
        package_info::PackageInfo,
    },
};
use ris_input::input::Input;
use ris_video::video::Video;
use sdl2::EventPump;

pub struct GlobalContainer {
    pub _video: Video,
    pub event_pump: EventPump,
    pub frame_buffer: FrameBuffer,
    pub input: Input,
    pub app_info: AppInfo,
}

pub fn bootstrap(package_info: PackageInfo) -> Result<GlobalContainer, String> {
    let sdl_context = sdl2::init()?;

    let _video = Video::new(&sdl_context)?;
    let event_pump = sdl_context.event_pump()?;

    let frame_buffer = FrameBuffer::new(4);

    let input = Input::new(&sdl_context)?;

    let app_info = app_info(package_info);

    let global_container = GlobalContainer {
        _video,
        event_pump,
        frame_buffer,
        input,
        app_info,
    };

    Ok(global_container)
}
