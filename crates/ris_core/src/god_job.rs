use std::sync::Arc;

use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::god_state::GodState;
use ris_error::RisResult;
use ris_jobs::job_system;

use crate::god_object::GodObject;

pub enum WantsTo {
    Quit,
    Restart,
}

pub fn run(mut god_object: GodObject) -> RisResult<WantsTo> {
    let mut frame_calculator = god_object.frame_calculator;
    let mut current_logic = god_object.logic_data;
    let mut current_output = god_object.output_data;

    let god_state = god_object.state;

    loop {
        // update frame
        let frame = frame_calculator.bump_and_create_frame();

        // update god state
        copy_current_to_previous(&god_state);

        // create copies
        let previous_logic_for_logic = current_logic.clone();
        let previous_logic_for_output = current_logic.clone();

        let previous_output_for_output = current_output.clone();

        let state_for_logic = god_state.clone();
        let state_for_save_settings = god_state.clone();

        // game loop frame
        let output_future = job_system::submit(move || {
            let mut output_frame = god_object.output_frame;
            let mut current_output = current_output;
            let result = output_frame.run(
                &mut current_output,
                &previous_output_for_output,
                &previous_logic_for_output,
                frame,
            );

            (output_frame, current_output, result)
        });

        let save_settings_future = job_system::submit(move || {
            let settings_serializer = god_object.settings_serializer;

            let previous_state = job_system::lock_read(&state_for_save_settings.previous);
            let settings = &previous_state.settings;

            let result = if settings.save_requested() {
                settings_serializer.serialize(settings)
            } else {
                Ok(())
            };

            (settings_serializer, result)
        });

        let logic_result = god_object.logic_frame.run(
            &mut current_logic,
            &previous_logic_for_logic,
            frame,
            state_for_logic,
        );

        // wait for jobs
        let (new_output_frame, new_output_data, output_result) = output_future.wait();
        let (new_settings_serializer, save_settings_result) = save_settings_future.wait();

        // update buffers
        current_output = new_output_data;
        god_object.output_frame = new_output_frame;
        god_object.settings_serializer = new_settings_serializer;

        // restart job system
        let current_state = job_system::lock_write(&god_state.current);
        if current_state.settings.job().changed() {
            ris_log::debug!("job workers changed. restarting job system...");
            drop(god_object.job_system_guard);

            let cpu_count = god_object.app_info.cpu.cpu_count;
            let workers =
                job_system::determine_thread_count(&god_object.app_info, &current_state.settings);

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

fn copy_current_to_previous(god_state: &Arc<GodState>) {
    let mut current = job_system::lock_write(&god_state.current);
    let mut previous = job_system::lock_write(&god_state.previous);

    if current.settings.changed() {
        previous.settings = current.settings.clone();
    }

    current.reset();
}
