use crate::info::app_info::AppInfo;

#[derive(Default, Clone)]
pub struct JobSettings {
    changed: bool,

    workers: usize,
}

impl JobSettings {
    pub fn new(app_info: &AppInfo) -> Self {
        Self {
            changed: false,
            workers: app_info.cpu.cpu_count / 2,
        }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn reset(&mut self) {
        self.changed = false;
    }

    pub fn get_workers(&self) -> usize {
        self.workers
    }

    pub fn set_workers(&mut self, value: usize) {
        self.changed = true;
        self.workers = value;
    }
}
