use std::cell::UnsafeCell;

use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
    logic_data::LogicData, output_data::OutputData,
};
use ris_jobs::job_system;

use crate::{
    gameloop::{logic_frame, output_frame},
    god_object::GodObject,
};

pub fn run(mut god_object: GodObject) -> Result<(), String> {
    let frame = UnsafeCell::<FrameData>::default();

    let mut current_input = InputData::default();
    let previous_input = UnsafeCell::default();

    let mut current_logic = LogicData::default();
    let previous_logic = UnsafeCell::default();

    let mut current_output = OutputData::default();
    let previous_output = UnsafeCell::default();

    loop {
        // update frame
        unsafe { (*frame.get()).bump() }

        {
            // swap buffers
            current_input = std::mem::replace(unsafe { &mut *previous_input.get() }, current_input);
            current_logic = std::mem::replace(unsafe { &mut *previous_logic.get() }, current_logic);
            current_output =
                std::mem::replace(unsafe { &mut *previous_output.get() }, current_output);
        }

        {
            // create references
            let frame_for_input = unsafe { &*frame.get() };
            let frame_for_logic = unsafe { &*frame.get() };
            let frame_for_output = unsafe { &*frame.get() };

            let previous_input_for_input = unsafe { &*previous_input.get() };
            let previous_input_for_logic = unsafe { &*previous_input.get() };

            let previous_logic_for_logic = unsafe { &*previous_logic.get() };
            let previous_logic_for_output = unsafe { &*previous_logic.get() };

            let previous_output_for_output = unsafe { &*previous_output.get() };

            // submit jobs
            let output_future = job_system::submit(move || {
                output_frame::run(
                    current_output,
                    previous_output_for_output,
                    previous_logic_for_output,
                    frame_for_output,
                )
            });

            let logic_future = job_system::submit(move || {
                logic_frame::run(
                    current_logic,
                    previous_logic_for_logic,
                    previous_input_for_logic,
                    frame_for_logic,
                )
            });

            let (new_input_data, input_state) = god_object.input_frame.run(
                current_input,
                previous_input_for_input,
                frame_for_input,
            );

            // wait for jobs
            let (new_logic_data, logic_state) = logic_future.wait();
            let (new_output_data, output_state) = output_future.wait();

            // update buffers
            current_input = new_input_data;
            current_logic = new_logic_data;
            current_output = new_output_data;

            // determine, whether to continue, return error or exit
            if matches!(input_state, GameloopState::WantsToContinue)
                && matches!(logic_state, GameloopState::WantsToContinue)
                && matches!(output_state, GameloopState::WantsToContinue)
            {
                continue;
            }

            if let GameloopState::Error(error) = input_state {
                return Err(error);
            }

            if let GameloopState::Error(error) = logic_state {
                return Err(error);
            }

            if let GameloopState::Error(error) = output_state {
                return Err(error);
            }

            return Ok(());
        }
    }
}
