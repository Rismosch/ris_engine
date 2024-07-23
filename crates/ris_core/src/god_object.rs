use sdl2::keyboard::Scancode;

use ris_asset::asset_loader;
use ris_asset::asset_loader::AssetLoaderGuard;
use ris_asset::RisGodAsset;
use ris_data::gameloop::frame::FrameCalculator;
use ris_data::god_state::GodState;
use ris_data::info::app_info::AppInfo;
use ris_data::settings::serializer::SettingsSerializer;
use ris_data::settings::Settings;
use ris_debug::gizmo::GizmoGuard;
use ris_debug::profiler::ProfilerGuard;
use ris_error::RisResult;
use ris_jobs::job_system;
use ris_jobs::job_system::JobSystemGuard;
use ris_video::gizmo::gizmo_renderer::GizmoRenderer;
use ris_video::imgui::imgui_backend::ImguiBackend;
use ris_video::imgui::imgui_renderer::ImguiRenderer;
use ris_video::imgui::RisImgui;
use ris_video::vulkan::base::VulkanBase;

use crate::logic_frame::LogicFrame;
use crate::output_frame::OutputFrame;
use crate::ui_helper::UiHelper;

#[cfg(debug_assertions)]
fn import_assets() -> RisResult<()> {
    use ris_asset::asset_importer;

    ris_log::debug!("importing assets...");

    asset_importer::import_all(
        asset_importer::DEFAULT_SOURCE_DIRECTORY,
        asset_importer::DEFAULT_TARGET_DIRECTORY,
        Some("temp"),
    )?;

    ris_log::debug!("assets imported!");
    Ok(())
}

#[cfg(not(debug_assertions))]
fn import_assets() -> RisResult<()> {
    Ok(())
}

pub struct GodObject {
    pub app_info: AppInfo,
    pub settings_serializer: SettingsSerializer,
    pub frame_calculator: FrameCalculator,
    pub logic_frame: LogicFrame,
    pub output_frame: OutputFrame,
    pub god_asset: RisGodAsset,

    pub state: GodState,

    // guards, must be dropped last.
    // they are dropped in the order they are listed.
    pub gizmo_guard: GizmoGuard,
    pub profiler_guard: ProfilerGuard,
    pub asset_loader_guard: AssetLoaderGuard,
    pub job_system_guard: JobSystemGuard,
}

impl GodObject {
    pub fn new(app_info: AppInfo) -> RisResult<Self> {
        // settings
        let settings_serializer = SettingsSerializer::new(&app_info);
        let settings = match settings_serializer.deserialize(&app_info) {
            Some(settings) => settings,
            None => {
                let new_settings = Settings::new(&app_info);
                settings_serializer.serialize(&new_settings)?;
                new_settings
            }
        };

        // job system
        let cpu_count = app_info.cpu.cpu_count;
        let workers = crate::determine_thread_count(&app_info, &settings);
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

        // profiling
        let profiler_guard = unsafe { ris_debug::profiler::init() }?;

        // sdl
        let sdl_context =
            sdl2::init().map_err(|e| ris_error::new!("failed to init sdl2: {}", e))?;
        let event_pump = sdl_context
            .event_pump()
            .map_err(|e| ris_error::new!("failed to get event pump: {}", e))?;
        let controller_subsystem = sdl_context
            .game_controller()
            .map_err(|e| ris_error::new!("failed to get controller subsystem: {}", e))?;

        // god asset
        let god_asset_id = asset_loader_guard.god_asset_id.clone();
        let god_asset_bytes = asset_loader::load_async(god_asset_id).wait(None)??;
        let god_asset = RisGodAsset::load(&god_asset_bytes)?;

        // video
        let video_subsystem = sdl_context
            .video()
            .map_err(|e| ris_error::new!("failed to get video subsystem: {}", e))?;
        let window = video_subsystem
            .window("ris_engine", 640, 480)
            //.resizable()
            .position_centered()
            .vulkan()
            .build()?;

        let vulkan_base = VulkanBase::initialize(&app_info, &window, &god_asset)?;

        // imgui
        let mut imgui_backend = ImguiBackend::init(&app_info)?;
        let context = imgui_backend.context();
        let imgui_renderer = ImguiRenderer::init(&vulkan_base, &god_asset, context)?;
        let imgui = RisImgui {
            backend: imgui_backend,
            renderer: imgui_renderer,
        };

        // gizmo
        let gizmo_guard = unsafe { ris_debug::gizmo::init() }?;
        let gizmo_renderer = GizmoRenderer::init(&vulkan_base)?;

        // gameloop
        let ui_helper = UiHelper::new(&app_info)?;
        let logic_frame = LogicFrame::new(event_pump, sdl_context.keyboard(), controller_subsystem);
        let output_frame = OutputFrame::new(window, vulkan_base, imgui, gizmo_renderer, ui_helper)?;

        let frame_calculator = FrameCalculator::default();

        // god state
        let mut state = GodState::new(settings);

        {
            let input = &mut state.input;
            input.keyboard.keymask[0] = Scancode::Return;
            input.keyboard.keymask[15] = Scancode::W;
            input.keyboard.keymask[16] = Scancode::S;
            input.keyboard.keymask[17] = Scancode::A;
            input.keyboard.keymask[18] = Scancode::D;
            input.keyboard.keymask[19] = Scancode::Up;
            input.keyboard.keymask[20] = Scancode::Down;
            input.keyboard.keymask[21] = Scancode::Left;
            input.keyboard.keymask[22] = Scancode::Right;
            input.keyboard.keymask[28] = Scancode::Kp8;
            input.keyboard.keymask[29] = Scancode::Kp2;
            input.keyboard.keymask[30] = Scancode::Kp4;
            input.keyboard.keymask[31] = Scancode::Kp6;
        }

        // god object
        let god_object = GodObject {
            app_info,
            settings_serializer,
            frame_calculator,
            logic_frame,
            output_frame,
            god_asset,

            state,

            // guards
            gizmo_guard,
            profiler_guard,
            asset_loader_guard,
            job_system_guard,
        };

        Ok(god_object)
    }
}
