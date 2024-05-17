use ris_data::gameloop::gameloop_state::GameloopState;
use ris_error::RisResult;
use ris_jobs::job_system;

use crate::god_object::GodObject;

pub enum WantsTo {
    Quit,
    Restart,
}

pub fn run(mut god_object: GodObject) -> RisResult<WantsTo> {
    let mut frame_calculator = god_object.frame_calculator;

    loop {
        let frame = frame_calculator.bump_and_create_frame();

        let state_for_save_settings = god_object.state.clone();

        let save_settings_future = job_system::submit(move || {
            let settings_serializer = god_object.settings_serializer;
            let state = state_for_save_settings;

            let settings = &state.settings;

            let result = if settings.save_requested() {
                settings_serializer.serialize(settings)
            } else {
                Ok(())
            };

            (settings_serializer, result)
        });

        // reset events
        god_object.state.reset_events();

        // game loop frame
        let logic_result = god_object.logic_frame.run(frame, &mut god_object.state);
        let output_result =
            god_object
                .output_frame
                .run(frame, &mut god_object.state, &god_object.god_asset);

        // wait for jobs
        let (new_settings_serializer, save_settings_result) = save_settings_future.wait(None)?;

        // update buffers
        god_object.settings_serializer = new_settings_serializer;

        // restart job system
        let settings = &god_object.state.settings;
        if settings.job().changed() {
            ris_log::debug!("job workers changed. restarting job system...");
            drop(god_object.job_system_guard);

            let cpu_count = god_object.app_info.cpu.cpu_count;
            let workers = crate::determine_thread_count(&god_object.app_info, settings);

            let new_guard = unsafe {
                job_system::init(
                    job_system::DEFAULT_BUFFER_CAPACITY,
                    cpu_count,
                    workers,
                    true,
                )
            };

            god_object.job_system_guard = new_guard;

            ris_log::debug!("job system restarted!");
        }

        // handle errors
        output_result?;
        save_settings_result?;
        let gameloop_state = logic_result?;

        match gameloop_state {
            GameloopState::WantsToContinue => continue,
            GameloopState::WantsToQuit => return Ok(WantsTo::Quit),
            GameloopState::WantsToRestart => return Ok(WantsTo::Restart),
        }
    }
}
