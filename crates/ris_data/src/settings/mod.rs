pub mod job_settings;
pub mod serializer;

use job_settings::JobSettings;

pub struct Settings{
    pub job: JobSettings,
}
