pub mod metrics;
pub mod settings;

use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use imgui::Ui;

use ris_data::gameloop::frame::Frame;
use ris_data::god_state::GodState;
use ris_data::settings::ris_yaml::RisYaml;
use ris_data::info::app_info::AppInfo;
use ris_error::Extensions;
use ris_error::RisResult;
use ris_file::io;
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
    fn draw(&mut self, data: UiHelperDrawData) -> RisResult<()>;
}

pub struct UiHelperDrawData<'a> {
    pub ui: &'a Ui,
    pub logic_future: JobFuture<()>,
    pub frame: Frame,
    pub state: Arc<GodState>,
}

pub struct UiHelper {
    modules: Vec<Box<dyn UiHelperModule>>,
    pinned: Vec<String>,
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
            },
        }


    }

    fn serialize(&self) -> RisResult<()> {
        let selected_string = format!("{}", self.selected);
        let mut pinned_string = String::new();

        if let Some(first_module_name) = self.pinned.first() {
            pinned_string.push_str(first_module_name);
        }

        for module_name in self.pinned.iter().skip(1) {
            pinned_string.push_str(&format!(", {}", module_name));
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
        let file_size = file.seek(SeekFrom::End(0))?;
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

                        if pinned.contains(&trimmed.to_string()) {
                            continue;
                        }

                        let module = modules.iter().find(|x| x.name() == trimmed).unroll()?;
                        pinned.push(module.name().to_string());
                    }
                },
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
        let retval = data.ui.window("UiHelper")
            .movable(false)
            .position([0., 0.], imgui::Condition::Once)
            .size([200., 200.], imgui::Condition::Once)
            .collapsed(true, imgui::Condition::FirstUseEver)
            .build(|| self.window_callback(data));

        match retval {
            Some(value) => value,
            None => Ok(()),
        }
    }

    fn window_callback(&mut self, data: UiHelperDrawData) -> RisResult<()> {
        let ui = data.ui;

        let module_names = self.modules
            .iter()
            .map(|x| x.name())
            .collect::<Vec<_>>();
        self.selected = usize::min(self.selected, module_names.len() - 1);
        let selected_module = &self.modules[self.selected];
        let mut pinned = self.pinned.contains(&selected_module.name().to_string());

        if ui.checkbox("##pinned", &mut pinned) {
            if pinned {
                self.pinned.push(selected_module.name().to_string());
            } else if let Some(index) = self.pinned.iter().position(|x| *x == selected_module.name()) {
                self.pinned.remove(index);
            }
        }

        ui.same_line();
        let checkbox_half_width = ui.item_rect_size()[0];
        ui.set_next_item_width(ui.window_size()[0] - checkbox_half_width * 2.);
        ui.combo_simple_string(
            "##modules",
            &mut self.selected,
            &module_names,
        );

        if !self.pinned.is_empty() {
            ui.new_line();
        }
        for pinned_module in self.pinned.iter() {
            ui.same_line();
            if ui.button(pinned_module) {
                if let Some(index) = self.modules.iter().position(|x| *x.name() == *pinned_module) {
                    self.selected = index;
                }
            }
        }

        ui.separator();

        let selected_module = &mut self.modules[self.selected];
        selected_module.draw(data)?;

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

