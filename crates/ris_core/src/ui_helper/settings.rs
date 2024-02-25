use crate::ui_helper::UiHelperDrawData;
use crate::ui_helper::UiHelperModule;

#[derive(Default)]
pub struct Settings {}

impl UiHelperModule for Settings {
    fn name(&self) -> &'static str {"Settings"}

    fn modifies_state(&self) -> bool {true}

    fn draw(&mut self, data: UiHelperDrawData) -> ris_error::RisResult<()> {
        let ui = data.ui;

        ui.text("settings");

        Ok(())
    }
}
