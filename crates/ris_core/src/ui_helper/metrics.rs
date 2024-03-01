use crate::ui_helper::UiHelperDrawData;
use crate::ui_helper::UiHelperModule;

#[derive(Default)]
pub struct Metrics;

impl UiHelperModule for Metrics {
    fn name(&self) -> &'static str {
        "Metrics"
    }

    fn draw(&mut self, data: UiHelperDrawData) -> ris_error::RisResult<()> {
        let ui = data.ui;

        ui.text("metrics");

        Ok(())
    }
}
