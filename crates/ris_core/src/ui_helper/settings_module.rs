use ris_data::info::app_info::AppInfo;
use ris_data::settings::serializer::SettingsSerializer;
use ris_data::settings::Settings;
use ris_error::RisResult;

use crate::ui_helper::UiHelperDrawData;
use crate::ui_helper::IUiHelperModule;

pub struct SettingsModule {
    app_info: AppInfo,
    saved: bool,
}

impl SettingsModule {
}

impl IUiHelperModule for SettingsModule {
    fn name() -> &'static str {
        "settings"
    }

    fn new(app_info: &AppInfo) -> Box<dyn IUiHelperModule> {
        Box::new(Self {
            app_info: app_info.clone(),
            saved: true,
        })
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> RisResult<()> {
        let ui = data.ui;
        let settings = &mut data.state.settings;

        if ui.collapsing_header("jobs", imgui::TreeNodeFlags::empty()) {
            let mut workers = settings.job().get_workers();
            if ui.slider("workers", 1, self.app_info.cpu.cpu_count, &mut workers) {
                settings.job_mut().set_workers(workers);
                self.saved = false;
            }
        }

        let mut header_flags = imgui::TreeNodeFlags::empty();
        header_flags.set(imgui::TreeNodeFlags::DEFAULT_OPEN, true);
        header_flags.set(imgui::TreeNodeFlags::BULLET, !self.saved);
        if ui.collapsing_header("settings file", header_flags) {
            {
                let disabled_token = ui.begin_disabled(self.saved);

                if ui.button("save") {
                    settings.request_save();
                    self.saved = true;
                }

                ui.same_line();
                if ui.button("load") {
                    let serializer = SettingsSerializer::new(&self.app_info);
                    if let Some(deserialized_settings) = serializer.deserialize(&self.app_info) {
                        *settings = deserialized_settings;
                    }
                    self.saved = true;
                }

                if !self.saved {
                    ui.same_line();
                    ui.label_text("", "settings are not saved!");
                }

                disabled_token.end();
            }

            if ui.button("restore default") {
                *settings = Settings::new(&self.app_info);
                self.saved = false;
            }
        }

        Ok(())
    }
}
