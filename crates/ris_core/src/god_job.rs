use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::god_state::GodStateRef;
use ris_data::god_state::InnerGodState;
use ris_jobs::job_system;
use ris_util::error::RisResult;

use crate::god_object::GodObject;

pub enum WantsTo {
    Quit,
    Restart,
}

pub fn run(mut god_object: GodObject) -> RisResult<WantsTo> {
    let mut frame_data_calculator = god_object.frame_data_calculator;
    let mut current_logic = god_object.logic_data;
    let mut current_output = god_object.output_data;

    let mut state_double_buffer = god_object.state_double_buffer;

    loop {
        // update frame
        frame_data_calculator.bump();
        let current_frame = frame_data_calculator.current();

        // update god state
        state_double_buffer.swap_and_reset();
        let state_back = state_double_buffer.back;
        let prev_queue = state_double_buffer.prev_queue;

        let front_ptr = state_double_buffer.front.get() as *const InnerGodState;
        let state_front = unsafe { GodStateRef::from(front_ptr) };

        let state_future = job_system::submit(move || {
            let mut state_back = state_back;
            let back = state_back.get_mut();
            let prev_queue = prev_queue;

            prev_queue.start_iter();
            while let Some(command) = prev_queue.next() {
                back.execute_command(command, false);
            }

            back.command_queue.start_iter();
            while let Some(command) = back.command_queue.next() {
                back.execute_command(command, true);
            }

            (state_back, prev_queue)
        });

        // create copies
        let frame_for_logic = current_frame.clone();
        let frame_for_output = current_frame.clone();

        let previous_logic_for_logic = current_logic.clone();
        let previous_logic_for_output = current_logic.clone();

        let previous_output_for_output = current_output.clone();

        // game loop frame
        let output_future = job_system::submit(move || {
            let mut output_frame = god_object.output_frame;
            let mut current_output = current_output;
            let gameloop_state = output_frame.run(
                &mut current_output,
                &previous_output_for_output,
                &previous_logic_for_output,
                &frame_for_output,
            );

            (output_frame, current_output, gameloop_state)
        });

        let logic_state = god_object.logic_frame.run(
            &mut current_logic,
            &previous_logic_for_logic,
            &frame_for_logic,
            state_front,
        );

        // wait for jobs
        let (new_output_frame, new_output_data, output_state) = output_future.wait();
        let (new_state_back, new_prev_queue) = state_future.wait();

        // update buffers
        current_output = new_output_data;
        god_object.output_frame = new_output_frame;

        state_double_buffer.back = new_state_back;
        state_double_buffer.prev_queue = new_prev_queue;

        // save settings and restart job system
        let state_front = state_double_buffer.front.get_mut();
        if state_front.events.save_settings_requested {
            god_object
                .settings_serializer
                .serialize(&state_front.data.settings)?;
        }

        if state_front.events.job_workers_settings_changed {
            ris_log::debug!("job workers changed. restarting job system...");
            drop(god_object.job_system_guard);

            let cpu_count = god_object.app_info.cpu.cpu_count;
            let workers = job_system::determine_thread_count(
                &god_object.app_info,
                &state_front.data.settings,
            );

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
        if let Err(e) = &logic_state {
            ris_log::fatal!("gameloop logic encountered an error: {}", e);
        }

        if let Err(e) = &output_state {
            ris_log::fatal!("gameloop output encountered an error: {}", e);
        }

        // determine, whether to continue, restart or quit
        let logic_state = logic_state?;
        if logic_state == GameloopState::WantsToContinue {
            continue;
        }

        if logic_state == GameloopState::WantsToRestart {
            return Ok(WantsTo::Restart);
        } else {
            return Ok(WantsTo::Quit);
        }
    }
}
