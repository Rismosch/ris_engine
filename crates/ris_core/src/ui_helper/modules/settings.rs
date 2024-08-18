use ris_data::settings::serializer::SettingsSerializer;
use ris_data::settings::Settings;
use ris_error::RisResult;

use crate::ui_helper::IUiHelperModule;
use crate::ui_helper::SharedStateWeakPtr;
use crate::ui_helper::UiHelperDrawData;

pub struct SettingsModule {
    shared_state: SharedStateWeakPtr,
    saved: bool,
}

impl SettingsModule {}

impl IUiHelperModule for SettingsModule {
    fn name() -> &'static str {
        "settings"
    }

    fn build(shared_state: SharedStateWeakPtr) -> Box<dyn IUiHelperModule> {
        Box::new(Self {
            shared_state,
            saved: true,
        })
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> RisResult<()> {
        let ui = data.ui;
        let settings = &mut data.state.settings;

        if ui.collapsing_header("jobs", imgui::TreeNodeFlags::empty()) {
            let mut workers = settings.job().get_workers();

            let cpu_count = self.shared_state.borrow().app_info.cpu.cpu_count;
            if ui.slider("workers", 1, cpu_count, &mut workers) {
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
                    let app_info = &self.shared_state.borrow().app_info;
                    let serializer = SettingsSerializer::new(app_info);
                    if let Some(deserialized_settings) = serializer.deserialize(app_info) {
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
                let app_info = &self.shared_state.borrow().app_info;
                *settings = Settings::new(app_info);
                self.saved = false;
            }
        }

        Ok(())
    }
}
