use std::sync::Arc;
use std::sync::RwLock;

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
        let data = GodStateData {
            settings,
        };

        RwLock::new(data)
    }

    pub fn reset(&mut self) {
        self.settings.reset();
    }
}

impl GodState {
    pub fn new(current: GodStateLock, previous: GodStateLock) -> Arc<Self> {
        let double_buffer = GodState {
            current,
            previous,
        };

        Arc::new(double_buffer)
    }
}

