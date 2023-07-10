use ris_data::info::app_info::AppInfo;
use ris_video::video::Video;

use crate::gameloop::input_frame::InputFrame;
use crate::gameloop::output_frame::OutputFrame;

pub struct GodObject {
    pub input_frame: InputFrame,
    pub output_frame: OutputFrame,
    pub app_info: AppInfo,
}

impl GodObject {
    pub fn new(app_info: AppInfo) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;

        let video = Video::new(&sdl_context)?;
        let event_pump = sdl_context.event_pump()?;

        let controller_subsystem = sdl_context.game_controller()?;

        let input_frame = InputFrame::new(event_pump, controller_subsystem);
        let output_frame = OutputFrame::new(video)?;

        let god_object = GodObject {
            input_frame,
            output_frame,
            app_info,
        };

        Ok(god_object)
    }
}
