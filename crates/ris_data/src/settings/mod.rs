pub mod job_settings;
pub mod key;
pub mod ris_yaml;
pub mod serializer;

use job_settings::JobSettings;

#[derive(Default, Clone)]
pub struct Settings {
    changed: bool,
    save_requested: bool,

    job: JobSettings,
}


impl Settings{
    pub fn changed(&self) -> bool {
        self.changed || self.job.changed()
    }

    pub fn reset(&mut self) {
        if self.save_requested {
            self.changed = true;
            self.save_requested = false;
        }

        self.job.reset();
    }

    pub fn save_requested(&self) -> bool {
        self.save_requested
    }

    pub fn request_save(&mut self) {
        self.changed = true;
        self.save_requested = true;
    }

    pub fn job(&self) -> &JobSettings {
        &self.job
    }

    pub fn job_mut(&mut self) -> &mut JobSettings {
        &mut self.job
    }
}

