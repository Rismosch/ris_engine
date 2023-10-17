pub mod job_settings;
pub mod serializer;

use job_settings::JobSettings;
use ris_util::error::RisResult;

use crate::info::app_info::AppInfo;

pub struct Settings{
    pub job: JobSettings,
}

impl Settings {
    pub fn load_or_new(app_info: &AppInfo) -> Self {
        match serializer::deserialize(app_info) {
            Some(settings) => settings,
            None => {
                panic!();
            }
        }
    }

    pub fn save(&self, app_info: &AppInfo) -> RisResult<()> {
        serializer::serialize(self, app_info)
    }
}
