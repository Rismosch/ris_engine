use ris_data::info::app_info::AppInfo;
use ris_data::settings::ris_yaml::RisYaml;
use ris_data::settings::serializer::SettingsSerializer;
use ris_data::settings::Settings;
use ris_error::RisResult;

use crate::ui_helper::UiHelperDrawData;
use crate::ui_helper::UiHelperModule;

pub struct SettingsModule {
    app_info: AppInfo,
    saved: bool,
}

impl SettingsModule {
    pub fn new(app_info: &AppInfo) -> Box<Self> {
        Box::new(Self {
            app_info: app_info.clone(),
            saved: true,
        })
    }
}

impl UiHelperModule for SettingsModule {
    fn name(&self) -> &'static str {
        "settings"
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> RisResult<()> {
        Ok(())
        //if let Some(future) = data.logic_future.take() {
        //    future.wait(None)?
        //}
        //let ui = data.ui;
        //let mut settings = data.state.front.settings.borrow_mut();

        //if ui.collapsing_header("jobs", imgui::TreeNodeFlags::empty()) {
        //    let mut workers = settings.job().get_workers();
        //    if ui.slider("workers", 1, self.app_info.cpu.cpu_count, &mut workers) {
        //        settings.job_mut().set_workers(workers);
        //        self.saved = false;
        //    }
        //}

        //let mut header_flags = imgui::TreeNodeFlags::empty();
        //header_flags.set(imgui::TreeNodeFlags::DEFAULT_OPEN, true);
        //header_flags.set(imgui::TreeNodeFlags::BULLET, !self.saved);
        //if ui.collapsing_header("settings file", header_flags) {
        //    {
        //        let disabled_token = ui.begin_disabled(self.saved);

        //        if ui.button("save") {
        //            settings.request_save();
        //            self.saved = true;
        //        }

        //        ui.same_line();
        //        if ui.button("load") {
        //            let serializer = SettingsSerializer::new(&self.app_info);
        //            if let Some(deserialized_settings) = serializer.deserialize(&self.app_info) {
        //                *settings = deserialized_settings;
        //            }
        //            self.saved = true;
        //        }

        //        if !self.saved {
        //            ui.same_line();
        //            ui.label_text("", "settings are not saved!");
        //        }

        //        disabled_token.end();
        //    }

        //    if ui.button("restore default") {
        //        *settings = Settings::new(&self.app_info);
        //        self.saved = false;
        //    }
        //}

        //Ok(())
    }

    fn always(&mut self, _data: &mut UiHelperDrawData) -> RisResult<()> {
        Ok(())
    }

    fn serialize(&self) -> RisResult<RisYaml> {
        Ok(RisYaml::default())
    }

    fn deserialize(&mut self, _yaml: RisYaml) -> RisResult<()> {
        Ok(())
    }
}
