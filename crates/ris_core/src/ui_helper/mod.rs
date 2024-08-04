use std::ffi::OsStr;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

use imgui::Ui;
use sdl2::keyboard::Scancode;

use ris_data::gameloop::frame::Frame;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::god_state::GodState;
use ris_data::info::app_info::AppInfo;
use ris_data::settings::ris_yaml::RisYaml;
use ris_error::Extensions;
use ris_error::RisResult;
use ris_jobs::job_future::JobFuture;

pub mod gizmo_module;
pub mod metrics_module;
pub mod settings_module;
pub mod util;

use crate::ui_helper::gizmo_module::GizmoModule;
use crate::ui_helper::metrics_module::MetricsModule;
use crate::ui_helper::settings_module::SettingsModule;

const CRASH_TIMEOUT_IN_SECS: u64 = 3;

pub trait IUiHelperModule {
    fn name() -> &'static str where Self: Sized;
    fn new(app_info: &AppInfo) -> Box<dyn IUiHelperModule> where Self: Sized;
    fn draw(&mut self, data: &mut UiHelperDrawData) -> RisResult<()>;
}

pub struct UiHelperModuleBuilder {
    pub name: String,
    pub build: Box<dyn Fn(&AppInfo) -> Box<dyn IUiHelperModule>>,
}

macro_rules! module {
    ($ui_module:ident) => {{
        UiHelperModuleBuilder {
            name: $ui_module::name().to_string(),
            build: Box::new($ui_module::new),
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
        GizmoModule,
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

pub struct UiHelperDrawData<'a> {
    pub ui: &'a Ui,
    pub frame: Frame,
    pub state: &'a mut GodState,
}

pub struct UiHelper {
    app_info: AppInfo,
    builders: Vec<UiHelperModuleBuilder>,
    windows: Vec<UiHelperWindow>,
    window_id: usize,
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
                    app_info: app_info.clone(),
                    builders: builders()?,
                    windows: Vec::new(),
                    window_id: 0,
                    crash_timestamp: now,
                    restart_timestamp: now,
                    close_window_timestamp: now,
                })
            }
        }
    }

    fn serialize(&self) -> RisResult<()> {
        Ok(())
    }

    fn deserialize(config_filepath: &Path, app_info: &AppInfo) -> RisResult<Self> {
        let now = Instant::now();

        Ok(Self {
            builders: builders()?,
            app_info: app_info.clone(),
            windows: Vec::new(),
            window_id: 0,
            crash_timestamp: now,
            restart_timestamp: now,
            close_window_timestamp: now,
        })
    }

    pub fn draw(&mut self, data: UiHelperDrawData) -> RisResult<GameloopState> {
        let result = data
            .ui
            .window("UiHelperMenuBar")
            .movable(false)
            .position([-1.0, 0.0], imgui::Condition::Always)
            .menu_bar(true)
            .title_bar(false)
            .resizable(false)
            .draw_background(false)
            .build(|| self.menu_callback(data));

        match result {
            Some(result) => result,
            None => Ok(GameloopState::WantsToContinue),
        }
    }

    fn menu_callback(&mut self, mut data: UiHelperDrawData) -> RisResult<GameloopState> {
        let UiHelperDrawData {
            ui,
            frame,
            state,
        } = data;

        let mut reimport_asset_future = None;

        if let Some(_) = ui.begin_menu_bar() {
            if let Some(_) = ui.begin_menu("start") {
                if ui.menu_item("restart (F1)") {
                    ris_log::fatal!("manual restart requestd");
                    return Ok(GameloopState::WantsToRestart);
                }

                if ui.menu_item("crash (F4)") {
                    ris_log::fatal!("manual crash requested");
                    return ris_error::new_result!("manual crash");
                }

                if ui.menu_item("quit") {
                    return Ok(GameloopState::WantsToQuit);
                }
            }

            if let Some(_) = ui.begin_menu("debug") {
                if ui.menu_item("reimport assets (F5)") {
                    reimport_assets(&mut reimport_asset_future)?;
                }

                if ui.menu_item("rebuild renderers (F6)") {
                    reimport_assets(&mut reimport_asset_future)?;
                    state.event_rebuild_renderers = true;
                }

                if ui.menu_item("spawn window (F7)") {
                    ris_log::debug!("spawn window");
                }

                if ui.menu_item("close all windows (F8)") {
                    self.windows.clear();
                }
            }
        }

        if state.input.keyboard.keys.is_hold(Scancode::F1) {
            let duration = Instant::now() - self.restart_timestamp;
            let seconds = duration.as_secs();

            if seconds >= CRASH_TIMEOUT_IN_SECS {
                ris_log::fatal!("manual restart requestd");
                return Ok(GameloopState::WantsToRestart);
            }
        } else {
            self.restart_timestamp = Instant::now();
        }

        if state.input.keyboard.keys.is_hold(Scancode::F4) {
            let duration = Instant::now() - self.crash_timestamp;
            let seconds = duration.as_secs();

            if seconds >= CRASH_TIMEOUT_IN_SECS {
                ris_log::fatal!("manual crash requested");
                return ris_error::new_result!("manual crash");
            }
        } else {
            self.crash_timestamp = Instant::now();
        }

        if state.input.keyboard.keys.is_down(Scancode::F5) {
            reimport_assets(&mut reimport_asset_future)?;
        }

        if state.input.keyboard.keys.is_down(Scancode::F6) {
            reimport_assets(&mut reimport_asset_future)?;
            state.event_rebuild_renderers = true;
        }

        if state.input.keyboard.keys.is_down(Scancode::F7) {
            let window = UiHelperWindow {
                id: self.window_id,
                name: "pick a module".to_string(),
                module: None,
            };

            self.windows.push(window);

            self.window_id = self.window_id.wrapping_add(1);
        }

        if state.input.keyboard.keys.is_hold(Scancode::F8) {
            let duration = Instant::now() - self.close_window_timestamp;
            let seconds = duration.as_secs();

            if seconds >= CRASH_TIMEOUT_IN_SECS {
                self.windows.clear();
            }
        } else {
            self.close_window_timestamp = Instant::now();
        }

        // take ownership of data again. otherwise the loop below does not compile
        let mut data = UiHelperDrawData{
            ui,
            frame,
            state,
        };

        for i in 0..self.windows.len() {
            let window = &self.windows[i];

            ui
                .window(format!("{}##ui_helper_window_{}", window.name, window.id))
                .movable(true)
                .position([13.0, 42.0], imgui::Condition::Once)
                .build(|| self.window_callback(i, &mut data));
        }

        if let Some(future) = reimport_asset_future.take() {
            future.wait(None)?;
        }

        Ok(GameloopState::WantsToContinue)
    }

    fn window_callback(&mut self, window_index: usize, data: &mut UiHelperDrawData) -> RisResult<()> {
        let UiHelperDrawData {
            ui,
            frame,
            state,
        } = data;

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
                let module = (builder.build)(&self.app_info);
                window.module = Some(module);
                window.name = builder.name.clone();
            }
        }

        if let Some(module) = &mut window.module {
            module.draw(data)?;
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

