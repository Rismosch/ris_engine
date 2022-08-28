use ris_data::info::package_info::PackageInfo;
use ris_jobs::job_system::JobSystem;
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
        let mut job_system = JobSystem::new(1024, 12);

        let result = god_job::run(&mut self.god_object);

        job_system.wait_till_done();

        result
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        ris_log::info!("engine was dropped");
    }
}
