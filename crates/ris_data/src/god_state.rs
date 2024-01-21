use std::sync::Arc;
use std::sync::RwLock;

use ris_jobs::job_system;

use crate::settings::Settings;

pub struct GodStateData {
    pub settings: Settings,
}

pub type GodStateLock = RwLock<GodStateData>;

pub struct GodState {
    pub current: GodStateLock,
    pub previous: GodStateLock,
}

impl GodStateData {
    pub fn new(settings: Settings) -> GodStateLock {
        let data = GodStateData { settings };

        RwLock::new(data)
    }

    pub fn reset(&mut self) {
        self.settings.reset();
    }
}

impl GodState {
    pub fn new(current: GodStateLock, previous: GodStateLock) -> Arc<Self> {
        let double_buffer = GodState { current, previous };

        Arc::new(double_buffer)
    }

    pub fn current_read(&self) {

    }

    pub fn current_write(&self) {

    }

    pub fn prev_read(&self) {

    }

    pub fn prev_write(&self) {

    }

    pub fn copy_current_to_previous(&self) {
        let mut current = job_system::lock_write(&self.current);
        let mut previous = job_system::lock_write(&self.previous);

        if current.settings.changed() {
            previous.settings = current.settings.clone();
        }

        current.reset();
    }
}
