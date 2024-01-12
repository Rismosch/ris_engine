use std::path::PathBuf;
use std::sync::Arc;

use sdl2::keyboard::Scancode;

use ris_asset::asset_loader;
use ris_asset::asset_loader::AssetLoaderGuard;
use ris_asset::loader::scenes_loader;
use ris_asset::loader::scenes_loader::Scenes;
use ris_asset::AssetId;
use ris_data::gameloop::frame::FrameCalculator;
use ris_data::gameloop::logic_data::LogicData;
use ris_data::gameloop::output_data::OutputData;
use ris_data::god_state::GodState;
use ris_data::god_state::GodStateData;
use ris_data::info::app_info::AppInfo;
use ris_data::settings::serializer::SettingsSerializer;
use ris_data::settings::Settings;
use ris_error::RisResult;
use ris_jobs::job_system;
use ris_jobs::job_system::JobSystemGuard;
use ris_video::imgui::RisImgui;
use ris_video::imgui::backend::ImguiBackend;
use ris_video::imgui::renderer::ImguiRenderer;
use ris_video::vulkan::renderer::Renderer;

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
    pub frame_calculator: FrameCalculator,
    pub logic_frame: LogicFrame,
    pub output_frame: OutputFrame,
    pub logic_data: LogicData,
    pub output_data: OutputData,
    pub scenes: Scenes,

    pub state: Arc<GodState>,

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
            sdl2::init().map_err(|e| ris_error::new!("failed to init sdl2: {}", e))?;
        let event_pump = sdl_context
            .event_pump()
            .map_err(|e| ris_error::new!("failed to get event pump: {}", e))?;
        let controller_subsystem = sdl_context
            .game_controller()
            .map_err(|e| ris_error::new!("failed to get controller subsystem: {}", e))?;

        // scenes
        let scenes_id = scenes_id();
        let scenes_bytes = ris_error::unroll!(
            asset_loader::load_async(scenes_id).wait(),
            "failed to load ris_scenes"
        )?;
        let scenes = scenes_loader::load(&scenes_bytes)?;

        // video
        let renderer = Renderer::initialize(&sdl_context, scenes.clone())?;

        // imgui
        let mut imgui_backend = ImguiBackend::init(&app_info)?;
        let context = imgui_backend.context();
        let imgui_renderer = ImguiRenderer::init(context)?;
        let imgui = RisImgui{
            backend: imgui_backend,
            renderer: imgui_renderer,
        };

        // gameloop
        let logic_frame = LogicFrame::new(event_pump, sdl_context.keyboard(), controller_subsystem);
        let output_frame = OutputFrame::new(renderer, imgui);

        let frame_calculator = FrameCalculator::default();
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
        let current = GodStateData::new(settings.clone());
        let previous = GodStateData::new(settings);
        let state = GodState::new(current, previous);

        // god object
        let god_object = GodObject {
            app_info,
            settings_serializer,
            frame_calculator,
            logic_frame,
            output_frame,
            logic_data,
            output_data,
            scenes,

            state,

            // guards
            asset_loader_guard,
            job_system_guard,
        };

        Ok(god_object)
    }
}
