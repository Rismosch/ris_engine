use ris_jobs::job_system;

use crate::{
    gameloop::{run_one_frame, GameloopState},
    god_object::GodObject,
};

pub fn run(god_object: &mut GodObject) -> Result<(), String> {
    loop {
        let gameloop_state = run_one_frame(god_object);

        job_system::submit(move || {
            let thread_index = job_system::thread_index();
            ris_log::debug!("{}", thread_index);
        });

        match gameloop_state {
            GameloopState::Running => continue,
            GameloopState::WantsToQuit => break,
            GameloopState::Error(error) => return Err(error),
        }
    }

    Ok(())
}
