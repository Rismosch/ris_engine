use crate::ui_helper::UiHelperDrawData;
use crate::ui_helper::UiHelperModule;

#[derive(Default)]
pub struct Settings {}

impl UiHelperModule for Settings {
    fn name(&self) -> &'static str {
        "Settings"
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> ris_error::RisResult<()> {
        if let Some(future) = data.logic_future.take() {
            future.wait(None)?
        }

        let ui = data.ui;

        ui.text("settings");

        Ok(())
    }
}
