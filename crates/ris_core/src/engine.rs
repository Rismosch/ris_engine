use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::info::app_info::AppInfo;
use ris_jobs::job_system;
use ris_log::log_message::LogMessage;

use ris_util::ris_error::RisError;

use crate::god_job;
use crate::god_object::GodObject;

pub struct Engine {
    pub wants_to_restart: bool,
}

pub fn run(app_info: AppInfo) -> Result<Engine, RisError> {
    let formatted_app_info = format!("{}", &app_info);
    ris_log::log::forward_to_appenders(LogMessage::Plain(formatted_app_info));

    let god_object = GodObject::new(app_info)?;

    let cpu_count = god_object.app_info.cpu.cpu_count as usize;
    let workers = god_object.app_info.args.workers as usize;
    let job_system_guard = unsafe { job_system::init(1024, cpu_count, workers) };

    let result = god_job::run(god_object);
    let return_value = match result {
        GameloopState::Error(error) => Err(error),
        GameloopState::WantsToRestart => Ok(Engine {
            wants_to_restart: true,
        }),
        GameloopState::WantsToQuit => Ok(Engine {
            wants_to_restart: false,
        }),
        GameloopState::WantsToContinue => ris_util::result_err!(
            "god job returned but wants to continue? i don't know how this is supposed to happen"
        ),
    };

    drop(job_system_guard);
    ris_log::info!("engine was droped");

    return_value
}
