use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::info::app_info::AppInfo;
use ris_util::ris_error::RisResult;

use crate::god_job;
use crate::god_object::GodObject;

pub struct Engine {
    pub wants_to_restart: bool,
}

pub fn run(app_info: AppInfo) -> RisResult<Engine> {
    let god_object = GodObject::new(app_info)?;
    let result = god_job::run(god_object)?;

    match result {
        GameloopState::WantsToRestart => Ok(Engine {
            wants_to_restart: true,
        }),
        GameloopState::WantsToQuit => Ok(Engine {
            wants_to_restart: false,
        }),
        GameloopState::WantsToContinue => ris_util::result_err!(
            "god job returned but wants to continue? i don't know how this is supposed to happen"
        ),
    }
}
