use ris_data::info::app_info::AppInfo;
use ris_jobs::job_system;
use ris_log::log_message::LogMessage;

use crate::{god_job, god_object::GodObject};

pub struct Engine {
    god_object: Option<GodObject>,
}

impl Engine {
    pub fn new(app_info: AppInfo) -> Result<Engine, String> {
        let god_object = GodObject::new(app_info)?;

        let formatted_app_info = format!("{}", god_object.app_info);
        ris_log::log::forward_to_appenders(LogMessage::Plain(formatted_app_info));

        Ok(Engine {
            god_object: Some(god_object),
        })
    }

    pub fn run(&mut self) -> Result<i32, String> {
        let god_object = match self.god_object.take() {
            Some(god_object) => god_object,
            None => return Err(String::from("god_object was already taken")),
        };

        let threads = god_object.app_info.cpu.cpu_count as usize;
        let job_system_guard = job_system::init(1024, threads);

        let result = god_job::run(god_object);

        drop(job_system_guard);

        result
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        ris_log::info!("engine was dropped");
    }
}
