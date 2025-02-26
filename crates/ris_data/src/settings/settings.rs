use crate::info::app_info::AppInfo;
use crate::ris_yaml::RisYaml;
use crate::ris_yaml::RisYamlEntry;

use super::job_settings::JobSettings;
use super::serializer::SerializeError;

#[derive(Default, Clone)]
pub struct Settings {
    changed: bool,
    save_requested: bool,

    job: JobSettings,
}

impl Settings {
    pub fn new(app_info: &AppInfo) -> Self {
        Self {
            changed: false,
            save_requested: false,

            job: JobSettings::new(app_info),
        }
    }

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

    pub fn serialize(&self, yaml: &mut RisYaml) {
        self.job.serialize(yaml);
        // add more serializers here...
    }

    pub fn deserialize(&mut self, entry: &RisYamlEntry) -> Result<(), SerializeError> {
        let Err(e) = self.job.deserialize(entry) else {
            return Ok(());
        };

        if e == SerializeError::ParseFailed {
            return Err(e);
        }

        // add more deserializers here...

        Ok(())
    }
}
