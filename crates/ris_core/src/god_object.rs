use sdl2::keyboard::Scancode;

use ris_asset::asset_loader;
use ris_asset::asset_loader::AssetLoaderGuard;
use ris_asset::loader::scenes_loader;
use ris_asset::loader::scenes_loader::Scenes;
use ris_asset::AssetId;
use ris_data::gameloop::frame_data::FrameDataCalculator;
use ris_data::gameloop::logic_data::LogicData;
use ris_data::gameloop::output_data::OutputData;
use ris_data::god_state::GodStateDoubleBuffer;
use ris_data::god_state::GodStateQueue;
use ris_data::god_state::InnerGodState;
use ris_data::info::app_info::AppInfo;
use ris_data::settings::serializer::SettingsSerializer;
use ris_data::settings::Settings;
use ris_jobs::job_system;
use ris_jobs::job_system::JobSystemGuard;
use ris_util::error::RisResult;
use ris_video::video::Video;

use crate::logic_frame::LogicFrame;
use crate::output_frame::OutputFrame;

#[cfg(debug_assertions)]
fn import_assets() -> RisResult<()> {
    ris_log::debug!("importing assets...");

    use ris_asset::asset_importer::*;
    import_all(DEFAULT_SOURCE_DIRECTORY, DEFAULT_TARGET_DIRECTORY)?;

    ris_log::debug!("assets imported!");
    Ok(())
}

#[cfg(not(debug_assertions))]
fn import_assets() -> RisResult<()> {
    Ok(())
}

#[cfg(debug_assertions)]
fn scenes_id() -> AssetId {
    AssetId::Directory(String::from("root.ris_scenes"))
}

#[cfg(not(debug_assertions))]
fn scenes_id() -> AssetId {
    AssetId::Compiled(0)
}

pub struct GodObject {
    pub app_info: AppInfo,
    pub settings_serializer: SettingsSerializer,
    pub frame_data_calculator: FrameDataCalculator,
    pub logic_frame: LogicFrame,
    pub output_frame: OutputFrame,
    pub logic_data: LogicData,
    pub output_data: OutputData,
    pub scenes: Scenes,

    pub state_double_buffer: GodStateDoubleBuffer,

    // guards
    pub asset_loader_guard: AssetLoaderGuard,
    pub job_system_guard: JobSystemGuard,
}

impl GodObject {
    pub fn new(app_info: AppInfo) -> RisResult<Self> {
        // settings
        let settings_serializer = SettingsSerializer::new(&app_info);
        let settings = match settings_serializer.deserialize() {
            Some(settings) => settings,
            None => {
                let new_settings = Settings::default();
                settings_serializer.serialize(&new_settings)?;
                new_settings
            }
        };

        // job system
        let cpu_count = app_info.cpu.cpu_count;
        let workers = job_system::determine_thread_count(&app_info, &settings);
        let job_system_guard = unsafe {
            job_system::init(
                job_system::DEFAULT_BUFFER_CAPACITY,
                cpu_count,
                workers,
                true,
            )
        };

        // assets
        import_assets()?;
        let asset_loader_guard = unsafe { asset_loader::init(&app_info)? };

        // sdl
        let sdl_context =
            sdl2::init().map_err(|e| ris_util::new_err!("failed to init sdl2: {}", e))?;
        let event_pump = sdl_context
            .event_pump()
            .map_err(|e| ris_util::new_err!("failed to get event pump: {}", e))?;
        let controller_subsystem = sdl_context
            .game_controller()
            .map_err(|e| ris_util::new_err!("failed to get controller subsystem: {}", e))?;

        // scenes
        let scenes_id = scenes_id();
        let scenes_bytes = ris_util::unroll!(
            asset_loader::load(scenes_id).wait(),
            "failed to load ris_scenes"
        )?;
        let scenes = scenes_loader::load(&scenes_bytes)?;

        // video
        let video = Video::new(&sdl_context, scenes.material.clone())?;

        // gameloop
        let logic_frame = LogicFrame::new(event_pump, controller_subsystem);
        let output_frame = OutputFrame::new(video);

        let frame_data_calculator = FrameDataCalculator::default();
        let mut logic_data = LogicData::default();
        let output_data = OutputData::default();

        logic_data.keyboard.keymask[0] = Scancode::Return;
        logic_data.keyboard.keymask[15] = Scancode::W;
        logic_data.keyboard.keymask[16] = Scancode::S;
        logic_data.keyboard.keymask[17] = Scancode::A;
        logic_data.keyboard.keymask[18] = Scancode::D;
        logic_data.keyboard.keymask[19] = Scancode::Up;
        logic_data.keyboard.keymask[20] = Scancode::Down;
        logic_data.keyboard.keymask[21] = Scancode::Left;
        logic_data.keyboard.keymask[22] = Scancode::Right;
        logic_data.keyboard.keymask[28] = Scancode::Kp8;
        logic_data.keyboard.keymask[29] = Scancode::Kp2;
        logic_data.keyboard.keymask[30] = Scancode::Kp4;
        logic_data.keyboard.keymask[31] = Scancode::Kp6;

        // god state
        let front = InnerGodState::new(settings.clone());
        let back = InnerGodState::new(settings);
        let prev_queue = GodStateQueue::default();
        let state_double_buffer = GodStateDoubleBuffer {
            front,
            back,
            prev_queue,
        };

        // god object
        let god_object = GodObject {
            app_info,
            settings_serializer,
            frame_data_calculator,
            logic_frame,
            output_frame,
            logic_data,
            output_data,
            scenes,

            state_double_buffer,

            // guards
            asset_loader_guard,
            job_system_guard,
        };

        Ok(god_object)
    }
}
