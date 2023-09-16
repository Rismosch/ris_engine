use sdl2::keyboard::Scancode;

use ris_asset::asset_loader;
use ris_asset::asset_loader::AssetLoaderGuard;
use ris_data::gameloop::frame_data::FrameDataCalculator;
use ris_data::gameloop::input_data::InputData;
use ris_data::gameloop::logic_data::LogicData;
use ris_data::gameloop::output_data::OutputData;
use ris_data::info::app_info::AppInfo;
use ris_jobs::job_system;
use ris_jobs::job_system::JobSystemGuard;
use ris_log::log;
use ris_log::log::Appenders;
use ris_log::log::LogGuard;
use ris_log::log_level::LogLevel;
use ris_log::log_message::LogMessage;
use ris_util::ris_error::RisError;
use ris_video::video::Video;

use crate::gameloop::input_frame::InputFrame;
use crate::gameloop::logic_frame::LogicFrame;
use crate::gameloop::output_frame::OutputFrame;

pub struct GodObject {
    pub app_info: AppInfo,
    pub frame_data_calculator: FrameDataCalculator,
    pub input_frame: InputFrame,
    pub logic_frame: LogicFrame,
    pub output_frame: OutputFrame,
    pub input_data: InputData,
    pub logic_data: LogicData,
    pub output_data: OutputData,

    // guards
    pub asset_loader_guard: AssetLoaderGuard,
    pub job_system_guard: JobSystemGuard,
    pub log_guard: LogGuard,
}

impl GodObject {
    pub fn new(app_info: AppInfo) -> Result<Self, RisError> {
        // log
        use crate::appenders::file_appender::FileAppender;
        use ris_log::console_appender::ConsoleAppender;
        let appenders: Appenders = vec![ConsoleAppender::new(), FileAppender::new(&app_info)];
        let log_guard = log::init(LogLevel::Trace, appenders);

        let formatted_app_info = format!("{}", &app_info);
        ris_log::log::forward_to_appenders(LogMessage::Plain(formatted_app_info));

        // job system
        let cpu_count = app_info.cpu.cpu_count as usize;
        let workers = app_info.args.workers as usize;
        let job_system_guard = unsafe { job_system::init(1024, cpu_count, workers) };

        // assets
        let asset_loader_guard = asset_loader::init(&app_info)?;

        // sdl
        let sdl_context =
            sdl2::init().map_err(|e| ris_util::new_err!("failed to init sdl2: {}", e))?;
        let event_pump = sdl_context
            .event_pump()
            .map_err(|e| ris_util::new_err!("failed to get event pump: {}", e))?;
        let controller_subsystem = sdl_context
            .game_controller()
            .map_err(|e| ris_util::new_err!("failed to get controller subsystem: {}", e))?;

        // video
        let video = Video::new(&sdl_context)?;

        // gameloop
        let input_frame = InputFrame::new(event_pump, controller_subsystem);
        let logic_frame = LogicFrame::default();
        let output_frame = OutputFrame::new(video);

        let frame_data_calculator = FrameDataCalculator::default();
        let mut input_data = InputData::default();
        let logic_data = LogicData::default();
        let output_data = OutputData::default();

        input_data.keyboard.keymask[0] = Scancode::Return;
        input_data.keyboard.keymask[15] = Scancode::W;
        input_data.keyboard.keymask[16] = Scancode::S;
        input_data.keyboard.keymask[17] = Scancode::A;
        input_data.keyboard.keymask[18] = Scancode::D;
        input_data.keyboard.keymask[19] = Scancode::Up;
        input_data.keyboard.keymask[20] = Scancode::Down;
        input_data.keyboard.keymask[21] = Scancode::Left;
        input_data.keyboard.keymask[22] = Scancode::Right;
        input_data.keyboard.keymask[28] = Scancode::Kp8;
        input_data.keyboard.keymask[29] = Scancode::Kp2;
        input_data.keyboard.keymask[30] = Scancode::Kp4;
        input_data.keyboard.keymask[31] = Scancode::Kp6;

        // god object
        let god_object = GodObject {
            app_info,
            frame_data_calculator,
            input_frame,
            logic_frame,
            output_frame,
            input_data,
            logic_data,
            output_data,

            // guards
            asset_loader_guard,
            job_system_guard,
            log_guard,
        };

        Ok(god_object)
    }
}
