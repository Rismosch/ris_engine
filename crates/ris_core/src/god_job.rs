use std::{thread, time::Duration};

use ris_jobs::job_system;

use crate::{
    gameloop::{run_one_frame, GameloopState},
    god_object::GodObject,
};

pub fn run(god_object: &mut GodObject) -> Result<(), String> {
    loop {
        let gameloop_state = run_one_frame(god_object);

        let future = job_system::submit(|| {
            let thread_index = job_system::thread_index();
            ris_log::debug!("{} hoi", thread_index);

            thread::sleep(Duration::from_millis(100));

            format!("{} poi", thread_index)
        });

        let result = job_system::wait(future);
        ris_log::debug!("{}", result);

        match gameloop_state {
            GameloopState::Running => continue,
            GameloopState::WantsToQuit => break,
            GameloopState::Error(error) => return Err(error),
        }
    }

    Ok(())
}
