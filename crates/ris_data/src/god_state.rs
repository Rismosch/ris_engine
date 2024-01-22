use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

use ris_jobs::job_system;

use crate::settings::Settings;

pub struct GodStateData {
    pub settings: Settings,
}

pub type GodStateLock = RwLock<GodStateData>;

pub struct GodState {
    front: GodStateLock,
    back: GodStateLock,
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
    pub fn new(front: GodStateLock, back: GodStateLock) -> Arc<Self> {
        let double_buffer = GodState { front, back };

        Arc::new(double_buffer)
    }

    pub fn front(&self) -> RwLockReadGuard<GodStateData> {
        job_system::lock_read(&self.front)
    }

    pub fn front_mut(&self) -> RwLockWriteGuard<GodStateData>{
        job_system::lock_write(&self.front)
    }

    pub fn back(&self) -> RwLockReadGuard<GodStateData>{
        job_system::lock_read(&self.back)
    }

    pub fn back_mut(&self) -> RwLockWriteGuard<GodStateData> {
        job_system::lock_write(&self.back)
    }

    pub fn copy_front_to_back(&self) {
        let mut front = self.front_mut();
        let mut back = self.back_mut();

        if front.settings.changed() {
            back.settings = front.settings.clone();
        }

        front.reset();
    }
}
