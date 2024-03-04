use ris_data::info::app_info::AppInfo;
use ris_data::settings::serializer::SettingsSerializer;

use crate::ui_helper::UiHelperDrawData;
use crate::ui_helper::UiHelperModule;

pub struct Settings {
    app_info: AppInfo,
}

impl Settings {
    pub fn new(app_info: &AppInfo) -> Self {
        Self {
            app_info: app_info.clone(),
        }
    }
}

impl UiHelperModule for Settings {
    fn name(&self) -> &'static str {
        "settings"
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> ris_error::RisResult<()> {
        if let Some(future) = data.logic_future.take() {
            future.wait(None)?
        }
        let ui = data.ui;
        let mut settings = data.state.front.settings.borrow_mut();

        if ui.collapsing_header("jobs", imgui::TreeNodeFlags::empty()) {
            let mut workers = settings.job().get_workers();
            if ui.slider("workers", 1, self.app_info.cpu.cpu_count, &mut workers) {
                settings.job_mut().set_workers(workers);
            }
        }

        if ui.button("save") {
            settings.request_save();
        }

        if ui.button("load") {
            let serializer = SettingsSerializer::new(&self.app_info);
            if let Some(deserialized_settings) = serializer.deserialize(&self.app_info) {
                *settings = deserialized_settings;
            }
        }

        if ui.button("restore default") {
            *settings = ris_data::settings::Settings::new(&self.app_info);
        }

        Ok(())
    }
}
