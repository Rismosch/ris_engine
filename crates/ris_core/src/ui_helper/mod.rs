use std::ffi::CString;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::ptr;
use std::time::Instant;

use imgui::Ui;
use imgui::WindowFlags;
use imgui::WindowFocusedFlags;
use sdl2::keyboard::Scancode;

use ris_data::asset_id::AssetId;
use ris_data::gameloop::frame::Frame;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::god_state::GodState;
use ris_data::info::app_info::AppInfo;
use ris_data::settings::ris_yaml::RisYaml;
use ris_error::RisResult;
use ris_jobs::job_future::JobFuture;
use ris_ptr::ArefCell;
use ris_ptr::StrongPtr;
use ris_ptr::WeakPtr;

pub mod modules;
pub mod selection;
pub mod util;

use selection::Selector;

use modules::asset_browser::AssetBrowser;
use modules::gizmo::GizmoModule;
use modules::hierarchy::HierarchyModule;
use modules::inspector::InspectorModule;
use modules::log::LogModule;
use modules::metrics::MetricsModule;
use modules::settings::SettingsModule;

const CRASH_TIMEOUT_IN_SECS: u64 = 3;

const WINDOW_OFFSET: f32 = 19.0;
const WINDOW_SIZE: [f32; 2] = [200.0, 300.0];

const WINDOW_KEY: &str = "window_";
const WINDOW_SEPARATOR: char = ',';

pub trait IUiHelperModule {
    fn name() -> &'static str
    where
        Self: Sized;
    fn build(shared_state: SharedStateWeakPtr) -> Box<dyn IUiHelperModule>
    where
        Self: Sized;
    fn draw(&mut self, data: &mut UiHelperDrawData) -> RisResult<()>;
}

#[allow(clippy::type_complexity)]
pub struct UiHelperModuleBuilder {
    pub name: String,
    pub build: Box<dyn Fn(SharedStateWeakPtr) -> Box<dyn IUiHelperModule>>,
}

macro_rules! module {
    ($ui_module:ident) => {{
        UiHelperModuleBuilder {
            name: $ui_module::name().to_string(),
            build: Box::new($ui_module::build),
        }
    }};
}

macro_rules! module_vec {
    ($($ui_module:ident),+ $(,)*) => {{
        vec![$(module!($ui_module)),+]
    }};
}

fn builders() -> RisResult<Vec<UiHelperModuleBuilder>> {
    let modules = module_vec![
        AssetBrowser,
        GizmoModule,
        HierarchyModule,
        InspectorModule,
        LogModule,
        MetricsModule,
        SettingsModule,
        // add new modules here
    ];

    // assert valid names
    let mut existing_names = std::collections::hash_set::HashSet::new();

    for module in modules.iter() {
        let name = &module.name;
        if existing_names.contains(name) {
            return ris_error::new_result!(
                "module names must be unique! offending name: \"{}\"",
                name
            );
        }

        existing_names.insert(name);

        let splits = name.split('.').collect::<Vec<_>>();
        if splits.len() != 1 {
            return ris_error::new_result!(
                "module name must not contain `.` (dot)! offending name: \"{}\"",
                name
            );
        }
    }

    Ok(modules)
}

pub struct SharedState {
    app_info: AppInfo,
    selector: Selector,
    loaded_chunks: Vec<Option<AssetId>>,
}

impl SharedState {
    fn new(app_info: AppInfo) -> SharedStateStrongPtr {
        StrongPtr::new(ArefCell::new(Self {
            app_info,
            selector: Selector::default(),
            loaded_chunks: Vec::new(),
        }))
    }

    fn chunk(&mut self, index: usize) -> Option<AssetId> {
        self.reserve_chunks(index);
        self.loaded_chunks[index].clone()
    }

    fn set_chunk(&mut self, index: usize, value: Option<AssetId>) {
        self.reserve_chunks(index);
        self.loaded_chunks[index] = value;
    }

    fn reserve_chunks(&mut self, index: usize) {
        let total_chunks = self.loaded_chunks.len() as isize;
        let iindex = index as isize;
        let chunks_to_add = iindex - total_chunks + 1;
        for _ in 0..chunks_to_add {
            self.loaded_chunks.push(None);
        }
    }
}

pub type SharedStateStrongPtr = StrongPtr<ArefCell<SharedState>>;
pub type SharedStateWeakPtr = WeakPtr<ArefCell<SharedState>>;

pub struct UiHelperDrawData<'a> {
    pub ui: &'a Ui,
    pub frame: Frame,
    pub state: &'a mut GodState,
    pub window_drawable_size: (u32, u32),
}

pub struct UiHelper {
    builders: Vec<UiHelperModuleBuilder>,

    windows: Vec<UiHelperWindow>,
    window_id: usize,

    config_filepath: PathBuf,

    shared_state: SharedStateStrongPtr,
    show_ui: bool,
    show_demo: bool,
    reimport_asset_future: Option<JobFuture<()>>,
    crash_timestamp: Instant,
    restart_timestamp: Instant,
    close_window_timestamp: Instant,
}

pub struct UiHelperWindow {
    id: usize,
    name: String,
    module: Option<Box<dyn IUiHelperModule>>,
}

impl Drop for UiHelper {
    fn drop(&mut self) {
        ris_log::debug!("dropping UiHelper...");

        if let Err(e) = self.serialize() {
            ris_log::error!("failed to serialize UiHelper: {}", e);
        }

        ris_log::info!("dropped UiHelper!");
    }
}

impl UiHelper {
    pub fn new(app_info: &AppInfo) -> RisResult<Self> {
        let mut dir = PathBuf::from(&app_info.file.pref_path);
        dir.push("ui_helper");

        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }

        let mut config_filepath = PathBuf::from(&dir);
        config_filepath.push("config.ris_yaml");

        match Self::deserialize(&config_filepath, app_info) {
            Ok(result) => Ok(result),
            Err(e) => {
                ris_log::error!(
                    "failed to deserialize UiHelper. generating new one... error: {}",
                    e
                );

                let now = Instant::now();

                Ok(Self {
                    builders: builders()?,

                    windows: Vec::new(),
                    window_id: 0,

                    config_filepath,

                    shared_state: SharedState::new(app_info.clone()),
                    show_ui: true,
                    show_demo: false,
                    reimport_asset_future: None,
                    crash_timestamp: now,
                    restart_timestamp: now,
                    close_window_timestamp: now,
                })
            }
        }
    }

    fn serialize(&self) -> RisResult<()> {
        let mut yaml = RisYaml::default();

        for (i, window) in self.windows.iter().enumerate() {
            if window.module.is_none() {
                continue;
            };

            let key = format!("window_{}", i);
            let value = format!("{}{} {}", window.id, WINDOW_SEPARATOR, window.name);
            yaml.add_key_value(&key, &value);
        }

        // write file
        let mut file = std::fs::File::create(&self.config_filepath)?;
        let file_content = yaml.to_string()?;
        let bytes = file_content.as_bytes();
        file.write_all(bytes)?;

        Ok(())
    }

    fn deserialize(config_filepath: &Path, app_info: &AppInfo) -> RisResult<Self> {
        // read file
        let mut file = std::fs::File::open(config_filepath)?;
        let file_size = ris_io::seek(&mut file, SeekFrom::End(0))?;
        ris_io::seek(&mut file, SeekFrom::Start(0))?;
        let mut bytes = vec![0; file_size as usize];
        ris_io::read(&mut file, &mut bytes)?;
        let file_content = String::from_utf8(bytes)?;
        let yaml = RisYaml::try_from(file_content.as_str())?;

        // parse yaml
        let builders = builders()?;

        let mut windows = Vec::new();
        let mut max_window_id = 0;
        let shared_state = SharedState::new(app_info.clone());

        for entry in yaml.entries.iter() {
            let Some((ref key, ref value)) = entry.key_value else {
                continue;
            };

            if !key.starts_with(WINDOW_KEY) {
                continue;
            }

            let splits = value.split(WINDOW_SEPARATOR).collect::<Vec<_>>();
            if splits.len() < 2 {
                continue;
            }

            let id_str = splits[0].trim();
            let name_str = splits[1].trim();

            let Ok(mut id) = id_str.parse::<usize>() else {
                continue;
            };

            let Some(builder_index) = builders.iter().position(|x| x.name == name_str) else {
                continue;
            };

            // value is correctly formatted. build module
            while windows.iter().any(|x: &UiHelperWindow| x.id == id) {
                id += 1;
            }

            max_window_id = usize::max(max_window_id, id);

            let builder = &builders[builder_index];
            let module = (builder.build)(shared_state.to_weak());

            let window = UiHelperWindow {
                id,
                name: builder.name.clone(),
                module: Some(module),
            };

            windows.push(window);
        }

        // create ui helper
        let now = Instant::now();

        Ok(Self {
            builders,

            windows,
            window_id: max_window_id + 1,

            config_filepath: config_filepath.to_path_buf(),

            shared_state,
            show_ui: true,
            show_demo: false,
            reimport_asset_future: None,
            crash_timestamp: now,
            restart_timestamp: now,
            close_window_timestamp: now,
        })
    }

    pub fn draw(&mut self, mut data: UiHelperDrawData) -> RisResult<GameloopState> {
        self.shared_state.borrow_mut().selector.update();

        let window_flags = WindowFlags::MENU_BAR
            | WindowFlags::NO_DOCKING
            | WindowFlags::NO_TITLE_BAR
            | WindowFlags::NO_COLLAPSE
            | WindowFlags::NO_RESIZE
            | WindowFlags::NO_MOVE
            | WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS
            | WindowFlags::NO_NAV_FOCUS;

        let size = [
            data.window_drawable_size.0 as f32,
            data.window_drawable_size.1 as f32,
        ];

        let result = if !self.show_ui {
            data.state.debug_ui_is_focused = false;
            None
        } else {
            data.ui
                .window("dockspace")
                .flags(window_flags)
                .position([0.0, 0.0], imgui::Condition::Always)
                .size(size, imgui::Condition::Always)
                .bg_alpha(0.0)
                .build(|| {
                    data.state.debug_ui_is_focused = data
                        .ui
                        .is_window_focused_with_flags(WindowFocusedFlags::ANY_WINDOW);

                    let id = "dockspace";
                    let id_cstr = CString::new(id)?;
                    let id_uint = unsafe { imgui::sys::igGetID_Str(id_cstr.as_ptr()) };
                    let size = imgui::sys::ImVec2 { x: 0.0, y: 0.0 };
                    let flags = 1 << 3; // ImGuiDockNodeFlags_PassthruCentralNode

                    unsafe { imgui::sys::igDockSpace(id_uint, size, flags, ptr::null()) };

                    self.menu_callback(&mut data)
                })
        };

        if data.state.input.keyboard.keys.is_hold(Scancode::F1) {
            let duration = Instant::now() - self.restart_timestamp;
            let seconds = duration.as_secs();

            if seconds >= CRASH_TIMEOUT_IN_SECS {
                ris_log::fatal!("manual restart requestd");
                return Ok(GameloopState::WantsToRestart);
            }
        } else {
            self.restart_timestamp = Instant::now();
        }

        if data.state.input.keyboard.keys.is_down(Scancode::F2) {
            self.show_ui = !self.show_ui;
        }

        if data.state.input.keyboard.keys.is_down(Scancode::F3) {
            self.show_demo = !self.show_demo;
        }

        if data.state.input.keyboard.keys.is_hold(Scancode::F4) {
            let duration = Instant::now() - self.crash_timestamp;
            let seconds = duration.as_secs();

            if seconds >= CRASH_TIMEOUT_IN_SECS {
                ris_log::fatal!("manual crash requested");
                return ris_error::new_result!("manual crash");
            }
        } else {
            self.crash_timestamp = Instant::now();
        }

        if data.state.input.keyboard.keys.is_down(Scancode::F5) {
            reimport_assets(&mut self.reimport_asset_future)?;
        }

        if data.state.input.keyboard.keys.is_down(Scancode::F6) {
            reimport_assets(&mut self.reimport_asset_future)?;
            data.state.event_rebuild_renderers = true;
        }

        if data.state.input.keyboard.keys.is_down(Scancode::F7) {
            let window = UiHelperWindow {
                id: self.window_id,
                name: "pick a module".to_string(),
                module: None,
            };

            self.windows.push(window);
            self.window_id = self.window_id.wrapping_add(1);
        }

        if data.state.input.keyboard.keys.is_hold(Scancode::F8) {
            let duration = Instant::now() - self.close_window_timestamp;
            let seconds = duration.as_secs();

            if seconds >= CRASH_TIMEOUT_IN_SECS {
                self.windows.clear();
            }
        } else {
            self.close_window_timestamp = Instant::now();
        }

        if self.show_demo {
            data.ui.show_demo_window(&mut self.show_demo);
        }

        if let Some(future) = self.reimport_asset_future.take() {
            future.wait(None)?;
        }

        match result {
            Some(result) => result,
            None => Ok(GameloopState::WantsToContinue),
        }
    }

    fn menu_callback(&mut self, data: &mut UiHelperDrawData) -> RisResult<GameloopState> {
        if let Some(_menu_bar) = data.ui.begin_menu_bar() {
            if let Some(_menu) = data.ui.begin_menu("start") {
                if data.ui.menu_item("restart (F1)") {
                    ris_log::fatal!("manual restart requestd");
                    return Ok(GameloopState::WantsToRestart);
                }

                if data.ui.menu_item("toggle ui (F2)") {
                    self.show_ui = !self.show_ui;
                }

                if data.ui.menu_item("toggle demo window (F3)") {
                    self.show_demo = !self.show_demo;
                }

                if data.ui.menu_item("crash (F4)") {
                    ris_log::fatal!("manual crash requested");
                    return ris_error::new_result!("manual crash");
                }

                if data.ui.menu_item("quit") {
                    return Ok(GameloopState::WantsToQuit);
                }
            }

            if let Some(_menu) = data.ui.begin_menu("debug") {
                if data.ui.menu_item("reimport assets (F5)") {
                    reimport_assets(&mut self.reimport_asset_future)?;
                }

                if data.ui.menu_item("rebuild renderers (F6)") {
                    reimport_assets(&mut self.reimport_asset_future)?;
                    data.state.event_rebuild_renderers = true;
                }

                data.ui.separator();

                if let Some(_spawn_window) = data.ui.begin_menu("spawn window (F7)") {
                    for builder in self.builders.iter() {
                        if data.ui.menu_item(&builder.name) {
                            let shared_state = self.shared_state.to_weak();
                            let module = (builder.build)(shared_state);

                            let window = UiHelperWindow {
                                id: self.window_id,
                                name: builder.name.clone(),
                                module: Some(module),
                            };

                            self.windows.push(window);
                            self.window_id = self.window_id.wrapping_add(1);
                        }
                    }
                }

                if data.ui.menu_item("close all windows (F8)") {
                    self.windows.clear();
                }
            }
        }

        let mut i = 0;

        while i < self.windows.len() {
            let window = &self.windows[i];

            let window_pos = (window.id + 1) as f32;
            let max_width = data.window_drawable_size.0 as f32 - WINDOW_SIZE[0];
            let max_height = data.window_drawable_size.1 as f32 - WINDOW_SIZE[1];
            let position_x = (WINDOW_OFFSET * window_pos) % max_width;
            let position_y = (WINDOW_OFFSET * window_pos) % max_height;

            let mut opened = true;

            let cond = 1 << 2; // ImGuiCond_FirstUseEver
            unsafe { imgui::sys::igSetNextWindowSize(WINDOW_SIZE.into(), cond) };
            unsafe {
                imgui::sys::igSetNextWindowPos(
                    [position_x, position_y].into(),
                    cond,
                    [0.0, 0.0].into(),
                )
            };

            let window_name =
                CString::new(format!("{}##ui_helper_window_{}", window.name, window.id))?;
            if unsafe { imgui::sys::igBegin(window_name.as_ptr(), &mut opened, 0) } {
                self.window_callback(i, data)?;
            }

            unsafe { imgui::sys::igEnd() };

            if opened {
                i += 1;
            } else {
                self.windows.remove(i);
            }
        }

        Ok(GameloopState::WantsToContinue)
    }

    fn window_callback(
        &mut self,
        window_index: usize,
        data: &mut UiHelperDrawData,
    ) -> RisResult<()> {
        let UiHelperDrawData { ui, .. } = data;

        let window = &mut self.windows[window_index];

        if window.module.is_none() {
            let mut choices = Vec::with_capacity(self.builders.len() + 1);
            choices.push("pick a module...");

            for builder in self.builders.iter() {
                choices.push(&builder.name);
            }

            let mut index = 0;
            ui.combo_simple_string("##selected_module", &mut index, &choices);

            if index > 0 {
                let builder = &self.builders[index - 1];
                let shared_state = self.shared_state.to_weak();
                let module = (builder.build)(shared_state);
                window.module = Some(module);
                window.name = builder.name.clone();
            }
        }

        if let Some(module) = &mut window.module {
            let result = module.draw(data);

            // returning an error may cause imgui to fail, because some end method may not be
            // called. this is bad, because this causes imgui to panic, which suppresses the
            // original error. thus we manually log the error, to avoid this suppression. this
            // may cause the error to be logged twice, but twice is better than not at all.
            if let Err(e) = &result {
                ris_log::error!("failed to draw module: {:?}", e);
            }

            result?;
        }

        Ok(())
    }
}

fn reimport_assets(import_asset_future: &mut Option<JobFuture<()>>) -> RisResult<()> {
    use ris_asset::asset_importer;

    if let Some(future) = import_asset_future.take() {
        future.wait(None)?;
    }

    let future = ris_jobs::job_system::submit(|| {
        let result = asset_importer::import_all(
            asset_importer::DEFAULT_SOURCE_DIRECTORY,
            asset_importer::DEFAULT_TARGET_DIRECTORY,
            Some("temp"),
        );

        if let Err(error) = result {
            ris_log::error!("failed to reimport assets: {}", error);
        }
    });

    *import_asset_future = Some(future);

    Ok(())
}
