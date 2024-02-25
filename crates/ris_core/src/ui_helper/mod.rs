use std::sync::Arc;

use imgui::Ui;

use ris_data::god_state::GodState;
use ris_data::info::app_info::AppInfo;
use ris_error::RisResult;
use ris_jobs::job_future::JobFuture;

type ModuleVec = Vec<Box<dyn UiHelperModule>>;

pub struct UiHelper {
    modules: ModuleVec,
    pinned: ModuleVec,
    selected: usize,
}

pub struct UiHelperDrawData<'a> {
    pub ui: &'a Ui,
    pub state: Arc<GodState>,
    pub logic_future: JobFuture<()>,
}

pub trait UiHelperModule {
    fn name(&self) -> &'static str;
    fn modifies_state(&self) -> bool;
    fn draw(&mut self) -> RisResult<()>;
}

impl UiHelper {
    pub fn new(app_info: &AppInfo) -> Self {
        let modules = vec![];
        let pinned = vec![];
        let selected = 0;

        Self {
            modules,
            pinned,
            selected,
        }
    }

    pub fn draw(&mut self, data: UiHelperDrawData) -> RisResult<()> {
        let retval = data.ui.window("UiHelper")
            .position([0., 0.], imgui::Condition::Once)
            .build(|| self.window_callback(data));

        match retval {
            Some(value) => value,
            None => Ok(()),
        }
    }

    fn window_callback(&mut self, data: UiHelperDrawData) -> RisResult<()> {
        let ui = data.ui;

        ui.checkbox("##", &mut true);
        ui.list_box("##", &mut 0, &["eins", "zwei", "drei"], 3);

        ui.separator();

        Ok(())
    }
}
