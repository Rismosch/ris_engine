use ris_data::{
    info::{
        app_info::{app_info, AppInfo},
        package_info::PackageInfo,
    },
};
use ris_input::input::Input;
use ris_video::video::Video;
use sdl2::EventPump;

pub struct GodObject {
    pub _video: Video,
    pub event_pump: EventPump,
    pub input: Input,
    pub app_info: AppInfo,
}

impl GodObject {
    pub fn new(package_info: PackageInfo) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;

        let _video = Video::new(&sdl_context)?;
        let event_pump = sdl_context.event_pump()?;

        let input = Input::new(&sdl_context)?;

        let app_info = app_info(package_info);

        let god_object = GodObject {
            _video,
            event_pump,
            input,
            app_info,
        };

        Ok(god_object)
    }
}
