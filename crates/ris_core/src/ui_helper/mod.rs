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

const PINNED: &str = "pinned";
const UNASSIGNED: &str = "unassigned";

fn modules(app_info: &AppInfo) -> RisResult<Vec<Box<dyn UiHelperModule>>> {
    let modules: Vec<Box<dyn UiHelperModule>> = vec![
        GizmoModule::new(),
        MetricsModule::new(app_info),
        SettingsModule::new(app_info),
        // add new modules here...
    ];

    // assert valid names
    let mut existing_names = std::collections::hash_set::HashSet::new();

    for module in modules.iter() {
        let name = module.name();
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

pub trait UiHelperModule {
    fn name(&self) -> &'static str;
    fn draw(&mut self, data: &mut UiHelperDrawData) -> RisResult<()>;
}

pub struct UiHelperDrawData<'a> {
    pub ui: &'a Ui,
    pub frame: Frame,
    pub state: &'a mut GodState,
}

struct PinnedUiHelperModule {
    pub module_index: Option<usize>,
    pub id: usize,
}

struct ModuleSelectedEvent {
    active_tab: usize,
}

pub struct UiHelper {
    modules: Vec<Box<dyn UiHelperModule>>,
    module_selected_event: Option<ModuleSelectedEvent>,
    config_filepath: PathBuf,
    crash_timestamp: Instant,
    restart_timestamp: Instant,
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

                Ok(Self {
                    modules: modules(app_info)?,
                    module_selected_event: None,
                    config_filepath,
                    crash_timestamp: Instant::now(),
                    restart_timestamp: Instant::now(),
                })
            }
        }
    }

    fn serialize(&self) -> RisResult<()> {
        Ok(())
    }

    fn deserialize(config_filepath: &Path, app_info: &AppInfo) -> RisResult<Self> {
        let mut modules = modules(app_info)?;

        Ok(Self {
            modules,
            module_selected_event: None,
            config_filepath: config_filepath.to_path_buf(),
            crash_timestamp: Instant::now(),
            restart_timestamp: Instant::now(),
        })
    }

    pub fn draw(&mut self, data: UiHelperDrawData) -> RisResult<GameloopState> {
        let result = data
            .ui
            .window("UiHelper")
            .movable(false)
            .position([0.0, 0.0], imgui::Condition::Once)
            .menu_bar(true)
            .title_bar(false)
            .resizable(false)
            .draw_background(false)
            .build(|| self.window_callback(data));

        match result {
            Some(result) => result,
            None => Ok(GameloopState::WantsToContinue),
        }
    }

    fn window_callback(&mut self, mut data: UiHelperDrawData) -> RisResult<GameloopState> {
        let UiHelperDrawData {
            ui,
            frame,
            state,
        } = data;

        ui.show_demo_window(&mut false);

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
            }

            if let Some(_) = ui.begin_menu("debug") {
                if ui.menu_item("spawn window (F5)") {
                    ris_log::debug!("spawn window");
                }

                if ui.menu_item("rebuild renderers (F6)") {
                    reimport_assets(&mut reimport_asset_future)?;
                    state.event_rebuild_renderers = true;
                }

                if ui.menu_item("reimport assets (F7)") {
                    reimport_assets(&mut reimport_asset_future)?;
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

        if state.input.keyboard.keys.is_down(Scancode::F6) {
            reimport_assets(&mut reimport_asset_future)?;
            state.event_rebuild_renderers = true;
        }

        if state.input.keyboard.keys.is_down(Scancode::F7) {
            reimport_assets(&mut reimport_asset_future)?;
        }

        if let Some(future) = reimport_asset_future.take() {
            future.wait(None)?;
        }

        Ok(GameloopState::WantsToContinue)
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

