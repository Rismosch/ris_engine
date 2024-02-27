pub mod metrics;
pub mod settings;

use std::sync::Arc;

use imgui::Ui;

use ris_data::gameloop::frame::Frame;
use ris_data::god_state::GodState;
use ris_data::info::app_info::AppInfo;
use ris_error::RisResult;
use ris_jobs::job_future::JobFuture;

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
}

impl UiHelper {
    pub fn new(app_info: &AppInfo) -> Self {
        let selected = 0;

        Self {
            modules: modules(),
            pinned: Vec::new(),
            selected,
        }
    }

    pub fn draw(&mut self, data: UiHelperDrawData) -> RisResult<()> {
        let retval = data.ui.window("UiHelper")
            .position([0., 0.], imgui::Condition::Once)
            .movable(false)
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
