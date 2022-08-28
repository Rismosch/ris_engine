use ris_jobs::job_system;

use crate::{god_object::GodObject, gameloop::{run_one_frame, GameloopState}};

pub fn run(god_object: &mut GodObject) -> Result<(), String> {
    loop {
        let gameloop_state = run_one_frame(god_object);

        for i in 0..10 {
            job_system::submit(move || {
                // thread::sleep(Duration::from_millis(100));
                let thread_index = job_system::thread_index();
                // ris_log::debug!("{} {}", thread_index, i);
            });
        }

        // ris_log::debug!("fps: {}", god_object.frame_buffer.fps());

        match gameloop_state {
            GameloopState::Running => continue,
            GameloopState::WantsToQuit => break,
            GameloopState::Error(error) => return Err(error),
        }
    }

    Ok(())
}