use ris_data::info::app_info::AppInfo;
use ris_video::video::Video;

use crate::gameloop::input_frame::InputFrame;

pub struct GodObject {
    pub _video: Video,
    pub input_frame: InputFrame,
    pub app_info: AppInfo,
}

impl GodObject {
    pub fn new(app_info: AppInfo) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;

        let _video = Video::new(&sdl_context)?;
        let event_pump = sdl_context.event_pump()?;

        let controller_subsystem = sdl_context.game_controller()?;

        let input_frame = InputFrame::new(event_pump, controller_subsystem);

        let god_object = GodObject {
            _video,
            input_frame,
            app_info,
        };

        Ok(god_object)
    }
}
