use ris_data::gameloop::gameloop_state::GameloopState;
use ris_error::RisResult;
use ris_jobs::job_future::SettableJobFuture;
use ris_jobs::job_system;

use crate::god_object::GodObject;

pub enum WantsTo {
    Quit,
    Restart,
}

pub fn run(mut god_object: GodObject) -> RisResult<WantsTo> {
    let mut frame_calculator = god_object.frame_calculator;

    let god_state = god_object.state;

    loop {
        // update frame
        let frame = frame_calculator.bump_and_create_frame();

        // update god state
        god_state.copy_front_to_back();

        // create copies
        let state_for_logic = god_state.clone();
        let state_for_output = god_state.clone();
        let state_for_save_settings = god_state.clone();

        // game loop frame
        let (settable_logic_future, logic_future) = SettableJobFuture::new();
        
        let output_future = job_system::submit(move || {
            let mut output_frame = god_object.output_frame;
            let state = state_for_output;

            let result = output_frame.run(frame, state, logic_future);

            (output_frame, result)
        });

        let save_settings_future = job_system::submit(move || {
            let settings_serializer = god_object.settings_serializer;
            let state = state_for_save_settings;

            let settings = &state.back.settings.borrow();

            let result = if settings.save_requested() {
                settings_serializer.serialize(settings)
            } else {
                Ok(())
            };

            (settings_serializer, result)
        });

        let logic_result = god_object.logic_frame.run(frame, state_for_logic);
        settable_logic_future.set(());

        // wait for jobs
        let (new_output_frame, output_result) = output_future.wait(None)?;
        let (new_settings_serializer, save_settings_result) = save_settings_future.wait(None)?;

        // update buffers
        god_object.output_frame = new_output_frame;
        god_object.settings_serializer = new_settings_serializer;

        // restart job system
        let settings = god_state.front.settings.borrow();
        if settings.job().changed() {
            ris_log::debug!("job workers changed. restarting job system...");
            drop(god_object.job_system_guard);

            let cpu_count = god_object.app_info.cpu.cpu_count;
            let workers = crate::determine_thread_count(&god_object.app_info, &settings);

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
