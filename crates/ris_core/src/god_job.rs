use std::collections::VecDeque;
use std::sync::Arc;

use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::god_state::execute_god_state_command;
use ris_data::god_state::GodStateEvents;
use ris_jobs::job_system;
use ris_util::error::RisResult;

use crate::god_object::GodObject;

pub enum WantsTo {
    Quit,
    Restart,
}

pub fn run(mut god_object: GodObject) -> RisResult<WantsTo> {
    let mut frame_data_calculator = god_object.frame_data_calculator;
    let mut current_input = god_object.input_data;
    let mut current_logic = god_object.logic_data;
    let mut current_output = god_object.output_data;

    let mut state_double_buffer = god_object.state_double_buffer;

    loop {
        // update frame
        frame_data_calculator.bump();
        let current_frame = frame_data_calculator.current();

        // update god state
        state_double_buffer.swap_and_reset();
        let state_front = state_double_buffer.front.clone();
        let state_back = state_double_buffer.back.clone();
        let prev_queue = state_double_buffer.prev_queue;

        let state_future = job_system::submit(move || {
            let mut back = state_back.borrow_mut();
            let prev_queue = prev_queue;

            prev_queue.start_iter();
            while let Some(command) = prev_queue.next() {
                execute_god_state_command(&mut back, command, false);
            }

            back.command_queue.start_iter();
            while let Some(command) = back.command_queue.next() {
                execute_god_state_command(&mut back, command, true);
            }

            prev_queue
        });

        // create copies
        let frame_for_input = current_frame.clone();
        let frame_for_logic = current_frame.clone();
        let frame_for_output = current_frame.clone();

        let previous_input_for_input = current_input.clone();
        let previous_input_for_logic = current_input.clone();
        let previous_input_for_output = current_input.clone();

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
                &previous_input_for_output,
                &previous_logic_for_output,
                &frame_for_output,
            );

            (output_frame, current_output, gameloop_state)
        });

        let logic_future = job_system::submit(move || {
            let mut logic_frame = god_object.logic_frame;
            let mut current_logic = current_logic;
            let gameloop_state = logic_frame.run(
                &mut current_logic,
                &previous_logic_for_logic,
                &previous_input_for_logic,
                &frame_for_logic,
                state_front.clone(),
            );

            (logic_frame, current_logic, gameloop_state)
        });

        let input_state = god_object.input_frame.run(
            &mut current_input,
            &previous_input_for_input,
            &frame_for_input,
        );

        // wait for jobs
        let (new_logic_frame, new_logic_data, logic_state) = logic_future.wait();
        let (new_output_frame, new_output_data, output_state) = output_future.wait();
        let new_prev_queue = state_future.wait();

        // update buffers
        current_logic = new_logic_data;
        current_output = new_output_data;
        god_object.output_frame = new_output_frame;
        god_object.logic_frame = new_logic_frame;

        state_double_buffer.prev_queue = new_prev_queue;

        // restart job system

        // handle errors
        if let Err(e) = &input_state {
            ris_log::fatal!("gameloop input encountered an error: {}", e);
        }

        if let Err(e) = &logic_state {
            ris_log::fatal!("gameloop logic encountered an error: {}", e);
        }

        if let Err(e) = &output_state {
            ris_log::fatal!("gameloop output encountered an error: {}", e);
        }

        let input_state = input_state?;
        let logic_state = logic_state?;
        let output_state = output_state?;

        // determine, whether to continue, restart or exit
        if matches!(input_state, GameloopState::WantsToContinue)
            && matches!(logic_state, GameloopState::WantsToContinue)
            && matches!(output_state, GameloopState::WantsToContinue)
        {
            continue;
        }

        if input_state != GameloopState::WantsToRestart
            && logic_state != GameloopState::WantsToRestart
            && output_state != GameloopState::WantsToRestart
        {
            return Ok(WantsTo::Quit);
        }
        else {
            return Ok(WantsTo::Restart);
        }
    }
}

