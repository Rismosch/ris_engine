use crate::ui_helper::UiHelperDrawData;
use crate::ui_helper::UiHelperModule;

#[derive(Default)]
pub struct MetricsModule;

impl UiHelperModule for MetricsModule {
    fn name(&self) -> &'static str {
        "metrics"
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> ris_error::RisResult<()> {
        let ui = data.ui;

        ui.text("metrics");

        Ok(())
    }
}
