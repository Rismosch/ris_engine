use std::path::PathBuf;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::KeyboardUtil;
use sdl2::keyboard::Scancode;
use sdl2::EventPump;
use sdl2::GameControllerSubsystem;

use ris_asset::asset_loader;
use ris_asset::asset_loader::AssetLoaderGuard;
use ris_asset::RisGodAsset;
use ris_data::ecs::registry::Registry;
use ris_data::ecs::scene::SceneCreateInfo;
use ris_data::gameloop::frame::FrameCalculator;
use ris_data::god_state::GodState;
use ris_data::info::app_info::AppInfo;
use ris_data::settings::serializer::SettingsSerializer;
use ris_data::settings::Settings;
use ris_debug::gizmo::GizmoGuard;
use ris_debug::profiler::ProfilerGuard;
use ris_error::RisResult;
use ris_input::gamepad_logic::GamepadLogic;
use ris_jobs::job_system;
use ris_jobs::job_system::JobSystemGuard;
use ris_video_data::core::VulkanCore;
use ris_video_renderers::GizmoSegmentRenderer;
use ris_video_renderers::GizmoTextRenderer;
use ris_video_renderers::SceneRenderer;
#[cfg(feature = "ui_helper_enabled")]
use ris_video_renderers::ImguiRenderer;
use third_party::imgui;
use third_party::imgui::ImGuiContext;
use third_party::imgui::backend::ImGuiBackend;

use crate::output_frame::OutputFrame;
use crate::output_frame::Renderer;
//#[cfg(feature = "ui_helper_enabled")]
//use crate::ui_helper::UiHelper;

pub struct GodObject {
    // fields are dropped in the order they are listed.
    pub app_info: AppInfo,
    pub settings_serializer: SettingsSerializer,
    pub frame_calculator: FrameCalculator,
    pub event_pump: EventPump,
    pub keyboard_util: KeyboardUtil,
    pub gamepad_logic: GamepadLogic,
    pub imgui_backend: ImGuiBackend, // must be dropped before logic_frame
    pub output_frame: OutputFrame,
    pub god_asset: RisGodAsset,
    pub state: GodState,

    // guards, must be dropped last.
    pub gizmo_guard: GizmoGuard,
    pub profiler_guard: ProfilerGuard,
    pub asset_loader_guard: AssetLoaderGuard,
    pub job_system_guard: JobSystemGuard,
}

impl GodObject {
    pub fn new(app_info: AppInfo, registry: Registry) -> RisResult<Self> {
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
        let job_system_guard = job_system::init(
            job_system::DEFAULT_BUFFER_CAPACITY,
            cpu_count,
            workers,
            true,
        );

        // assets
        #[cfg(debug_assertions)]
        {
            use ris_asset::asset_importer;

            ris_log::debug!("importing assets...");

            asset_importer::import_all(
                asset_importer::DEFAULT_SOURCE_DIRECTORY,
                asset_importer::DEFAULT_IMPORT_DIRECTORY,
                asset_importer::DEFAULT_IN_USE_DIRECTORY,
                Some("temp"),
            )?;

            ris_log::debug!("assets imported!");
        }

        let asset_loader_guard = asset_loader::init(&app_info)?;

        // profiling
        let profiler_guard = ris_debug::profiler::init()?;

        // sdl
        let sdl_context =
            sdl2::init().map_err(|e| ris_error::new!("failed to init sdl2: {}", e))?;
        let event_pump = sdl_context
            .event_pump()
            .map_err(|e| ris_error::new!("failed to get event pump: {}", e))?;
        let controller_subsystem = sdl_context
            .game_controller()
            .map_err(|e| ris_error::new!("failed to get controller subsystem: {}", e))?;

        // input
        let keyboard_util = sdl_context.keyboard();
        let gamepad_logic = GamepadLogic::new(controller_subsystem);

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
            .resizable()
            .maximized()
            .position_centered()
            .vulkan()
            .build()?;

        let vulkan_core = unsafe { VulkanCore::alloc(&app_info.package.name, &window) }?;

        // scene renderer
        let scene_renderer = unsafe { SceneRenderer::alloc(&vulkan_core, &god_asset) }?;

        // gizmo renderer
        let gizmo_guard = ris_debug::gizmo::init()?;
        let gizmo_segment_renderer =
            unsafe { GizmoSegmentRenderer::alloc(&vulkan_core, &god_asset) }?;
        let gizmo_text_renderer = unsafe { GizmoTextRenderer::alloc(&vulkan_core, &god_asset) }?;

        // imgui
        imgui::imgui_checkversion();
        let mut context = ImGuiContext::create();

        let ui_helper_dir = PathBuf::from(&app_info.file.pref_path)
            .join("ui_helper");
        std::fs::create_dir_all(&ui_helper_dir)?;

        let imgui_ini_filepath = ui_helper_dir.join("imgui.ini");

        context.set_ini_filename(Some(imgui_ini_filepath));

        let mut io = context.get_io();

        let mut config_flags = io.config_flags();
        config_flags |= imgui::sys::imgui::ImGuiConfigFlags__ImGuiConfigFlags_NavEnableKeyboard;
        config_flags |= imgui::sys::imgui::ImGuiConfigFlags__ImGuiConfigFlags_NavEnableGamepad;
        config_flags |= imgui::sys::imgui::ImGuiConfigFlags__ImGuiConfigFlags_DockingEnable;
        config_flags |= imgui::sys::imgui::ImGuiConfigFlags__ImGuiConfigFlags_ViewportsEnable;
        io.set_config_flags(config_flags);

        io.fonts().add_font_default();
        io.fonts().build();

        context.style_colors_dark();

        // there is so much to wrap in imgui style. but since i am not intending to make changes to
        // the style, let alone heavy changes, i just talk to the bindings directly
        unsafe {
            let style = imgui::sys::imgui::ImGui_GetStyle();
            if config_flags & imgui::sys::imgui::ImGuiConfigFlags__ImGuiConfigFlags_ViewportsEnable != 0 {
                (*style).WindowRounding = 0.0;
                ((*style).Colors)[imgui::sys::imgui::ImGuiCol__ImGuiCol_WindowBg as usize].w =
                    1.0;
            }
        }


        #[cfg(feature = "ui_helper_enabled")]
        let (imgui_backend, imgui_renderer) = {
            let mut imgui_backend = unsafe { ImGuiBackend::init(context, &window) };
            let imgui_renderer =
                unsafe { ImguiRenderer::alloc(
                    &vulkan_core,
                    &god_asset,
                    &mut imgui_backend.context,
                ) }?;
            (imgui_backend, imgui_renderer)
        };

        // output frame
        //#[cfg(feature = "ui_helper_enabled")]
        //let ui_helper = UiHelper::new(&app_info)?;

        let renderer = Renderer {
            scene: scene_renderer,
            gizmo_segment: gizmo_segment_renderer,
            gizmo_text: gizmo_text_renderer,
            #[cfg(feature = "ui_helper_enabled")]
            imgui: imgui_renderer,
        };

        let output_frame = OutputFrame {
            current_frame: 0,
            renderer,
            //#[cfg(feature = "ui_helper_enabled")]
            //imgui_backend,
            //#[cfg(feature = "ui_helper_enabled")]
            //ui_helper,
            core: vulkan_core,
            window,
        };

        let frame_calculator = FrameCalculator::default();

        // god state
        let scene_create_info = SceneCreateInfo {
            registry: Some(registry),
            ..Default::default()
        };
        let mut state = GodState::new(settings, scene_create_info)?;

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
            imgui_backend,
            event_pump,
            keyboard_util,
            gamepad_logic,
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
