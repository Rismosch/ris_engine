pub mod job_settings;
pub mod serializer;

use job_settings::JobSettings;

use crate::info::app_info::AppInfo;

#[derive(Clone)]
pub struct Settings {
    pub job: JobSettings,
}

impl Settings {
    pub fn new(app_info: &AppInfo) -> Self {
        // job settings
        let workers = app_info.cpu.cpu_count;
        let job = JobSettings {
            workers,
        };

        // settings
        Self {
            job,
        }
    }
}

