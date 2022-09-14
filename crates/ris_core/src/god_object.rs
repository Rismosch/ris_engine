use ris_data::info::{
    app_info::{app_info, AppInfo},
    package_info::PackageInfo,
};
use ris_video::video::Video;

use crate::gameloop::input_frame::InputFrame;

pub struct GodObject {
    pub _video: Video,
    pub input_frame: InputFrame,
    pub app_info: AppInfo,
}

impl GodObject {
    pub fn new(package_info: PackageInfo) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;

        let _video = Video::new(&sdl_context)?;
        let event_pump = sdl_context.event_pump()?;

        let input_frame = InputFrame::new(event_pump);

        let app_info = app_info(package_info);

        let god_object = GodObject {
            _video,
            input_frame,
            app_info,
        };

        Ok(god_object)
    }
}
