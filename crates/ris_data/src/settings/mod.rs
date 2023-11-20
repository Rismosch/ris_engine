pub mod job_settings;
pub mod key;
pub mod ris_yaml;
pub mod serializer;

use job_settings::JobSettings;

#[derive(Default, Clone)]
pub struct Settings {
    pub job: JobSettings,
}
