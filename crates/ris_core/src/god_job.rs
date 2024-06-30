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
        ris_debug::profiler::new_frame()?;
        let frame = frame_calculator.bump_and_create_frame();

        // reset events
        let previous_state = god_object.state.clone();
        god_object.state.reset_events();

        ris_debug::record!("reset events")?;

        // game loop frame
        let save_settings_future = job_system::submit(move || {
            let settings_serializer = god_object.settings_serializer;
            let state = previous_state;

            let settings = &state.settings;

            let result = if settings.save_requested() {
                settings_serializer.serialize(settings)
            } else {
                Ok(())
            };

            (settings_serializer, result)
        });

        ris_debug::record!("submit save settings future")?;

        let logic_result = god_object.logic_frame.run(frame, &mut god_object.state);
        ris_debug::record!("logic frame")?;
        let output_result =
            god_object
                .output_frame
                .run(frame, &mut god_object.state, &god_object.god_asset);
        ris_debug::record!("output frame")?;

        // wait for jobs
        let (new_settings_serializer, save_settings_result) = save_settings_future.wait(None)?;

        ris_debug::record!("wait for save settings future")?;

        // update buffers
        god_object.settings_serializer = new_settings_serializer;

        ris_debug::record!("update buffers")?;

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

        ris_debug::record!("restart job system")?;

        // handle errors
        output_result?;
        save_settings_result?;
        let gameloop_state = logic_result?;

        ris_debug::record!("handle errors")?;

        match gameloop_state {
            GameloopState::WantsToContinue => continue,
            GameloopState::WantsToQuit => return Ok(WantsTo::Quit),
            GameloopState::WantsToRestart => return Ok(WantsTo::Restart),
        }
    }
}
