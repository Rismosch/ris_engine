use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::info::app_info::AppInfo;
use ris_jobs::job_system;
use ris_log::log_message::LogMessage;

use ris_util::ris_error::RisError;

use crate::god_job;
use crate::god_object::GodObject;

pub struct Engine {
    god_object: Option<GodObject>,
    pub wants_to_restart: bool,
}

impl Engine {
    pub fn new(app_info: AppInfo) -> Result<Engine, RisError> {
        let formatted_app_info = format!("{}", &app_info);
        ris_log::log::forward_to_appenders(LogMessage::Plain(formatted_app_info));

        let god_object = GodObject::new(app_info)?;

        Ok(Engine {
            god_object: Some(god_object),
            wants_to_restart: false,
        })
    }

    pub fn run(&mut self) -> Result<(), RisError> {
        let god_object = match self.god_object.take() {
            Some(god_object) => god_object,
            None => return ris_util::result_err!("god_object was already taken"),
        };

        let cpu_count = god_object.app_info.cpu.cpu_count as usize;
        let workers = god_object.app_info.args.workers as usize;
        let job_system_guard = unsafe { job_system::init(1024, cpu_count, workers) };

        let result = god_job::run(god_object);
        match result {
            GameloopState::Error(error) => return Err(error),
            GameloopState::WantsToRestart => self.wants_to_restart = true,
            _ => (),
        }

        drop(job_system_guard);

        Ok(())
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        ris_log::info!("engine was dropped");
    }
}
