use ris_data::info::package_info::PackageInfo;
use ris_jobs::job_system;
use ris_log::log_message::LogMessage;

use crate::{god_job, god_object::GodObject};

pub struct Engine {
    god_object: GodObject,
}

impl Engine {
    pub fn new(package_info: PackageInfo) -> Result<Engine, String> {
        let god_object = GodObject::new(package_info)?;

        let app_info = format!("{}", god_object.app_info);
        ris_log::log::forward_to_appenders(LogMessage::Plain(app_info));

        Ok(Engine { god_object })
    }

    pub fn run(&mut self) -> Result<(), String> {
        let cpu_count = self.god_object.app_info.cpu.cpu_count as usize;
        let job_system_guard = job_system::init(1024, cpu_count);

        let result = god_job::run(&mut self.god_object);

        drop(job_system_guard);

        result
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        ris_log::info!("engine was dropped");
    }
}
