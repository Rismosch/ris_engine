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
use ris_error::Extensions;
use ris_error::RisResult;
use ris_jobs::job_future::JobFuture;

const PINNED: &str = "pinned";
const SELECTED: &str = "selected";

fn modules() -> Vec<Box<dyn UiHelperModule>> {
    vec![
        Box::<crate::ui_helper::metrics::Metrics>::default(),
        Box::<crate::ui_helper::settings::Settings>::default(),
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

pub struct UiHelper {
    modules: Vec<Box<dyn UiHelperModule>>,
    pinned: Vec<usize>,
    selected: usize,
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

        match Self::deserialize(&config_filepath) {
            Ok(result) => Ok(result),
            Err(e) => {
                ris_log::error!("failed to deserialize UiHelper: {}", e);

                Ok(Self {
                    modules: modules(),
                    pinned: Vec::new(),
                    selected: 0,
                    config_filepath,
                })
            }
        }
    }

    fn serialize(&self) -> RisResult<()> {
        let selected_string = format!("{}", self.selected);
        let mut pinned_string = String::new();

        if let Some(first_pinned) = self.pinned.first() {
            let first_pinned_module = &self.modules[*first_pinned];
            pinned_string.push_str(&first_pinned_module.name().to_string());
        }

        for pinned in self.pinned.iter().skip(1) {
            let pinned_module = &self.modules[*pinned];
            pinned_string.push_str(&format!(", {}", pinned_module.name()));
        }

        let mut yaml = RisYaml::default();
        yaml.add_key_value(PINNED, &pinned_string);
        yaml.add_key_value(SELECTED, &selected_string);

        let mut file = std::fs::File::create(&self.config_filepath)?;
        let file_content = yaml.to_string()?;
        let bytes = file_content.as_bytes();
        file.write_all(bytes)?;

        Ok(())
    }

    fn deserialize(config_filepath: &Path) -> RisResult<Self> {
        // read file
        let mut file = std::fs::File::open(config_filepath)?;
        let file_size = ris_file::seek!(file, SeekFrom::End(0))?;
        ris_file::seek!(file, SeekFrom::Start(0))?;
        let mut bytes = vec![0; file_size as usize];
        ris_file::read!(file, &mut bytes)?;
        let file_content = String::from_utf8(bytes)?;
        let yaml = RisYaml::try_from(file_content.as_str())?;

        // parse yaml
        let modules = modules();
        let mut pinned = Vec::new();
        let mut selected = 0;

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
                        let pinned_index = trimmed.parse::<usize>()?;
                        if pinned.contains(&pinned_index) {
                            continue;
                        }

                        if pinned_index < modules.len() {
                            pinned.push(pinned_index);
                        }
                    }
                }
                SELECTED => selected = value.parse::<usize>()?,
                _ => return ris_error::new_result!("unkown key at line {}", i),
            }
        }

        Ok(Self {
            modules,
            pinned,
            selected,
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



        //ui.show_demo_window(&mut true);

        //let module_names = self.modules.iter().map(|x| x.name()).collect::<Vec<_>>();
        //self.selected = usize::min(self.selected, module_names.len() - 1);

        //let mut is_pinned = self.pinned.contains(&self.selected);
        //if ui.checkbox("##pinned_checkbox", &mut is_pinned) {
        //    if is_pinned {
        //        self.pinned.push(self.selected);
        //    } else if let Some(index) = self.pinned.iter().find(|x| **x == self.selected) {
        //        self.pinned.remove(*index);
        //    }
        //}

        //ui.set_next_item_width(ui.content_region_avail()[0]);
        //ui.combo_simple_string("##modules", &mut self.selected, &module_names);

        //if let Some(tab_bar) = ui.tab_bar("##pinned_tabs") {
        //    let mut draw_unpinned_module = !self.pinned.contains(&self.selected);

        //    for pinned_module_index in self.pinned.iter() {
        //        let pinned_module = &mut self.modules[*pinned_module_index];
        //        let is_selected = *pinned_module_index == self.selected;

        //        let mut flags = imgui::TabItemFlags::empty();
        //        flags.set(imgui::TabItemFlags::SET_SELECTED, is_selected);
        //        if let Some(tab_item) = ui.tab_item_with_flags(pinned_module.name(), None, flags) {
        //            pinned_module.draw(&mut data)?;
        //            //draw_unpinned_module = false;
        //            tab_item.end();
        //        }
        //    }

        //    if draw_unpinned_module {
        //        // currently selected module is not pinned
        //        let selected_module = &mut self.modules[self.selected];

        //        let mut flags = imgui::TabItemFlags::empty();
        //        flags.set(imgui::TabItemFlags::SET_SELECTED, true);
        //        if let Some(tab_item) = ui.tab_item_with_flags("not pinned", None, flags) {
        //            selected_module.draw(&mut data)?;

        //            tab_item.end();
        //        }
        //    }

        //    tab_bar.end();
        //}

        //if let Some(tab_bar) = ui.tab_bar("##pinned_tabs") {
        //    let selected = self.selected;
        //    let selected_module = &self.modules[selected];
        //    let selected_module = selected_module.name().to_string();
        //    let is_pinned = self.pinned.contains(&selected_module);

        //    let mut unpin = None;

        //    for (pinned_index, pinned_module) in self.pinned.iter().enumerate() {
        //        let os_name = std::ffi::OsStr::new(&pinned_module);
        //        let os_name_ptr = os_name.as_encoded_bytes().as_ptr() as *const i8;

        //        if unsafe{imgui::sys::igTabItemButton(os_name_ptr, 0)} {
        //            if let Some(new_index) = self.modules.iter().position(|x| x.name() == pinned_module) {
        //                if *pinned_module == selected_module {
        //                    unpin = Some(pinned_index);
        //                } else {
        //                    self.selected = new_index;
        //                }
        //            }
        //        }
        //    }

        //    if let Some(pinned_index) = unpin {
        //        self.pinned.remove(pinned_index);
        //    }

        //    if !is_pinned {
        //        let os_name = std::ffi::OsStr::new(&selected_module);
        //        let os_name_ptr = os_name.as_encoded_bytes().as_ptr() as *const i8;

        //        let mut flags = 0;
        //        flags |= 1 << 0; // UnsavedDocument
        //        flags |= 1 << 7; // Trailing
        //        flags |= 1 << 8; // NoAssumedClosure

        //        if unsafe{imgui::sys::igTabItemButton(os_name_ptr, flags)} {
        //            self.pinned.push(selected_module);
        //        }
        //    }

        //    tab_bar.end();
        //}

        //let selected_module = &mut self.modules[self.selected];
        //selected_module.draw(data)?;

        ui.separator();

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
