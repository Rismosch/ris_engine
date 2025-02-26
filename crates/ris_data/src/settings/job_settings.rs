use crate::info::app_info::AppInfo;
use crate::ris_yaml::RisYaml;
use crate::ris_yaml::RisYamlEntry;

use crate::settings::serializer::SettingsSerializer;
use super::serializer::SerializeError;

const KEY_WORKERS: &str = "job.workers";
const KEY_AFFINITY: &str = "job.affinity";
const KEY_USE_PARKING: &str = "job.use_parking";

#[derive(Default, Clone)]
pub struct JobSettings {
    changed: bool,

    workers: usize,
    affinity: bool,
    use_parking: bool,
}

impl JobSettings {
    pub fn new(app_info: &AppInfo) -> Self {
        Self {
            changed: false,
            workers: app_info.cpu.cpu_count,
            affinity: true,
            use_parking: true,
        }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn reset(&mut self) {
        self.changed = false;
    }

    pub fn workers(&self) -> usize {
        self.workers
    }

    pub fn set_workers(&mut self, value: usize) {
        self.changed = true;
        self.workers = value;
    }

    pub fn affinity(&self) -> bool {
        self.affinity
    }

    pub fn set_affinity(&mut self, value: bool) {
        self.changed = true;
        self.affinity = value;
    }

    pub fn use_parking(&self) -> bool {
        self.use_parking
    }

    pub fn set_use_parking(&mut self, value: bool) {
        self.changed = true;
        self.use_parking = value;
    }

    pub fn serialize(&self, yaml: &mut RisYaml) {
        yaml.add_entry(None, Some("jobs"));
        yaml.add_entry(
            Some((KEY_WORKERS, &self.workers.to_string())),
            None,
        );
        yaml.add_entry(
            Some((KEY_AFFINITY, &self.affinity.to_string())),
            None,
        );
        yaml.add_entry(
            Some((KEY_USE_PARKING, &self.use_parking.to_string())),
            None,
        );
        yaml.add_entry(None, None);
    }

    pub fn deserialize(&mut self, entry: &RisYamlEntry) -> Result<(), SerializeError>{
        let Some((key, value)) = &entry.key_value else {
            return Err(SerializeError::EntryWasEmpty);
        };

        match key.as_str() {
            KEY_WORKERS => {
                let parsed = SettingsSerializer::parse(value)?;
                self.set_workers(parsed);
            },
            KEY_AFFINITY => {
                let parsed = SettingsSerializer::parse(value)?;
                self.set_affinity(parsed);
            },
            KEY_USE_PARKING => {
                let parsed = SettingsSerializer::parse(value)?;
                self.set_use_parking(parsed);
            },
            _ => return Err(SerializeError::UnkownKey)
        }

        Ok(())
    }
}
