pub mod job_settings;
pub mod key;
pub mod ris_yaml;
pub mod serializer;

use job_settings::JobSettings;

#[derive(Default, Clone)]
pub struct Settings {
    pub save_requested: bool,

    pub job: JobSettings,
}


impl Settings{
    pub fn reset(&mut self) {
        self.save_requested = false;
        self.job.reset();
    }
}

