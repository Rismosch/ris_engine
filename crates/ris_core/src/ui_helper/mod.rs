pub mod metrics;
pub mod settings;

use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use imgui::Ui;

use ris_data::gameloop::frame::Frame;
use ris_data::god_state::GodState;
use ris_data::info::app_info::AppInfo;
use ris_data::settings::ris_yaml::RisYaml;
use ris_error::RisResult;
use ris_jobs::job_future::JobFuture;

const PINNED: &str = "pinned";
const UNASSIGNED: &str = "unassigned";

fn modules(app_info: &AppInfo) -> Vec<Box<dyn UiHelperModule>> {
    vec![
        Box::new(crate::ui_helper::metrics::Metrics::default()),
        Box::new(crate::ui_helper::settings::Settings::new(app_info)),
        // insert new UiHelperModule here
    ]
}

pub trait UiHelperModule {
    fn name(&self) -> &'static str;
    fn draw(&mut self, data: &mut UiHelperDrawData) -> RisResult<()>;
}

pub struct UiHelperDrawData<'a> {
    pub ui: &'a Ui,
    pub logic_future: Option<JobFuture<()>>,
    pub frame: Frame,
    pub state: Arc<GodState>,
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
    pinned: Vec<PinnedUiHelperModule>,
    next_pinned_id: usize,
    module_selected_event: Option<ModuleSelectedEvent>,
    config_filepath: PathBuf,
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
                ris_log::error!("failed to deserialize UiHelper: {}", e);

                Ok(Self {
                    modules: modules(app_info),
                    pinned: Vec::new(),
                    next_pinned_id: 0,
                    module_selected_event: None,
                    config_filepath,
                })
            }
        }
    }

    fn serialize(&self) -> RisResult<()> {
        let pinned_strings = self
            .pinned
            .iter()
            .map(|x| match x.module_index {
                Some(index) => index.to_string(),
                None => String::from(UNASSIGNED),
            })
            .collect::<Vec<_>>();

        let pinned_string = pinned_strings.join(", ");

        let mut yaml = RisYaml::default();
        yaml.add_key_value(PINNED, &pinned_string);

        let mut file = std::fs::File::create(&self.config_filepath)?;
        let file_content = yaml.to_string()?;
        let bytes = file_content.as_bytes();
        file.write_all(bytes)?;

        Ok(())
    }

    fn deserialize(config_filepath: &Path, app_info: &AppInfo) -> RisResult<Self> {
        // read file
        let mut file = std::fs::File::open(config_filepath)?;
        let file_size = ris_file::seek!(file, SeekFrom::End(0))?;
        ris_file::seek!(file, SeekFrom::Start(0))?;
        let mut bytes = vec![0; file_size as usize];
        ris_file::read!(file, &mut bytes)?;
        let file_content = String::from_utf8(bytes)?;
        let yaml = RisYaml::try_from(file_content.as_str())?;

        // parse yaml
        let modules = modules(app_info);
        let mut pinned = Vec::new();

        let mut next_pinned_id = 0usize;

        for (i, entry) in yaml.entries.iter().enumerate() {
            let (key, value) = match &entry.key_value {
                Some(key_value) => key_value,
                None => continue,
            };

            match key.as_str() {
                PINNED => {
                    let splits = value.split(',');
                    for split in splits {
                        let trimmed = split.trim();
                        if trimmed.is_empty() {
                            continue;
                        }

                        let module_index = if trimmed == UNASSIGNED {
                            None
                        } else {
                            let index = trimmed.parse::<usize>()?;
                            if index < modules.len() {
                                Some(index)
                            } else {
                                None
                            }
                        };

                        pinned.push(PinnedUiHelperModule {
                            module_index,
                            id: next_pinned_id,
                        });

                        next_pinned_id = next_pinned_id.wrapping_add(1);
                    }
                }
                _ => return ris_error::new_result!("unkown key at line {}", i),
            }
        }

        Ok(Self {
            modules,
            pinned,
            next_pinned_id,
            module_selected_event: None,
            config_filepath: config_filepath.to_path_buf(),
        })
    }

    pub fn draw(&mut self, data: UiHelperDrawData) -> RisResult<()> {
        let result = data
            .ui
            .window("UiHelper")
            .movable(false)
            .position([0., 0.], imgui::Condition::Once)
            .size([200., 200.], imgui::Condition::FirstUseEver)
            .collapsed(true, imgui::Condition::FirstUseEver)
            .build(|| self.window_callback(data));

        match result {
            Some(result) => result,
            None => Ok(()),
        }
    }

    fn window_callback(&mut self, mut data: UiHelperDrawData) -> RisResult<()> {
        let ui = data.ui;

        let mut flags = imgui::TabBarFlags::empty();
        flags.set(imgui::TabBarFlags::AUTO_SELECT_NEW_TABS, true);
        flags.set(imgui::TabBarFlags::TAB_LIST_POPUP_BUTTON, true);
        flags.set(imgui::TabBarFlags::FITTING_POLICY_RESIZE_DOWN, true);
        if let Some(tab_bar) = ui.tab_bar_with_flags("##modules", flags) {
            // new tab button
            let new_tab = OsStr::new("+");
            let new_tab_ptr = new_tab.as_encoded_bytes().as_ptr() as *const i8;

            let mut flags = 0;
            flags |= imgui::sys::ImGuiTabItemFlags_Trailing;
            flags |= imgui::sys::ImGuiTabItemFlags_NoTooltip;
            if unsafe { imgui::sys::igTabItemButton(new_tab_ptr, flags as i32) } {
                self.pinned.push(PinnedUiHelperModule {
                    module_index: None,
                    id: self.next_pinned_id,
                });

                self.next_pinned_id = self.next_pinned_id.wrapping_add(1);
            }

            // imgui puts new tabs at the end. this is undesired, because renamed tabs are
            // considere new tabs. renaming a tab puts it at the end, messing up the
            // (de)serialization order of tabs. by assigning new ids to every tab, imgui thinks
            // everything is new, thus keeping th original order
            if self.module_selected_event.is_some() {
                for pinned_module in self.pinned.iter_mut() {
                    pinned_module.id = self.next_pinned_id;
                    self.next_pinned_id = self.next_pinned_id.wrapping_add(1);
                }
            }

            let mut n = 0;
            while n < self.pinned.len() {
                let pinned_module = &mut self.pinned[n];

                let name = match pinned_module.module_index {
                    Some(index) => self.modules[index].name(),
                    None => UNASSIGNED,
                };

                let name_with_id = format!("{}##pinned_module_{}", name, pinned_module.id);
                let mut open = true;
                let mut flags = imgui::TabItemFlags::empty();

                if let Some(ModuleSelectedEvent { active_tab }) = &self.module_selected_event {
                    flags.set(imgui::TabItemFlags::SET_SELECTED, *active_tab == n);
                }

                if let Some(tab_item) = ui.tab_item_with_flags(name_with_id, Some(&mut open), flags)
                {
                    match pinned_module.module_index {
                        Some(index) => self.modules[index].draw(&mut data)?,
                        None => {
                            let mut choices = Vec::with_capacity(self.modules.len() + 1);
                            choices.push("select module...");

                            for module in self.modules.iter() {
                                choices.push(module.name());
                            }

                            let mut index = 0;
                            ui.combo_simple_string("##select_module", &mut index, &choices);

                            if index > 0 {
                                pinned_module.module_index = Some(index - 1);
                                self.module_selected_event =
                                    Some(ModuleSelectedEvent { active_tab: n });
                            }
                        }
                    }

                    tab_item.end();
                }

                if open {
                    n += 1;
                } else {
                    self.pinned.remove(n);
                }
            }

            self.module_selected_event = None;

            tab_bar.end();
        }

        Ok(())
    }
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