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
    fn modifies_state(&self) -> bool;
    fn draw(&mut self, data: UiHelperDrawData) -> RisResult<()>;
}

pub struct UiHelperDrawData<'a> {
    pub ui: &'a Ui,
    pub frame: Frame,
    pub state: Arc<GodState>,
}

struct Module {
    inner: Box<dyn UiHelperModule>,
    pinned: bool,
}

pub struct UiHelper {
    modules: Vec<Module>,
    selected: usize,
}

impl UiHelper {
    pub fn new(app_info: &AppInfo) -> Self {
        let selected = 0;

        let modules = modules()
            .into_iter()
            .map(|x| Module{
                inner: x,
                pinned: false,
            })
            .collect::<Vec<_>>();


        Self {
            modules,
            selected,
        }
    }

    pub fn draw(&mut self, data: UiHelperDrawData, logic_future: JobFuture<()>) -> RisResult<()> {
        let retval = data.ui.window("UiHelper")
            .position([0., 0.], imgui::Condition::Once)
            .movable(false)
            .build(|| self.window_callback(data, logic_future));

        match retval {
            Some(value) => value,
            None => Ok(()),
        }
    }

    fn window_callback(&mut self, data: UiHelperDrawData, logic_future: JobFuture<()>) -> RisResult<()> {
        let ui = data.ui;

        let module_names = self.modules
            .iter()
            .map(|x| x.inner.name())
            .collect::<Vec<_>>();
        self.selected = usize::min(self.selected, module_names.len() - 1);
        let selected_module = &mut self.modules[self.selected];

        ui.checkbox("##", &mut selected_module.pinned);
        ui.same_line();
        ui.combo_simple_string(
            "UiHelperModule",
            &mut self.selected,
            &module_names,
        );

        ui.separator();

        let selected_module = &mut self.modules[self.selected];

        // when the module modifies state, then the logic frame MUST finish before drawing the
        // module, to avoid any data races
        if selected_module.inner.modifies_state() {
            logic_future.wait(Some(std::time::Duration::from_secs(1)))?;
        }

        selected_module.inner.draw(data)?;

        ui.separator();

        Ok(())
    }
}
