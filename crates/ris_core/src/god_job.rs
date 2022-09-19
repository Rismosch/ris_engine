use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
    logic_data::LogicData, output_data::OutputData,
};
use ris_jobs::{job_system, job_cell::JobCell};

use crate::{
    gameloop::{logic_frame, output_frame},
    god_object::GodObject,
    restart_code::RESTART_CODE,
};

pub fn run(mut god_object: GodObject) -> Result<i32, String> {
    let mut frame = JobCell::<FrameData>::default();

    let mut current_input = InputData::default();
    let mut previous_input = JobCell::<InputData>::default();

    let mut current_logic = LogicData::default();
    let mut previous_logic = JobCell::<LogicData>::default();

    let mut current_output = OutputData::default();
    let mut previous_output = JobCell::<OutputData>::default();

    loop {
        // update frame
        frame.bump();

        // swap buffers
        current_input = previous_input.replace(current_input);
        current_logic = previous_logic.replace(current_logic);
        current_output = previous_output.replace(current_output);

        // create references
        let ref_frame = frame.ref_cell();
        let ref_input = previous_input.ref_cell();
        let ref_logic = previous_logic.ref_cell();
        let ref_output = previous_output.ref_cell();

        let frame_for_input = ref_frame.borrow();
        let frame_for_logic = ref_frame.borrow();
        let frame_for_output = ref_frame.borrow();

        let previous_input_for_input = ref_input.borrow();
        let previous_input_for_logic = ref_input.borrow();

        let previous_logic_for_logic = ref_logic.borrow();
        let previous_logic_for_output = ref_logic.borrow();

        let previous_output_for_output = ref_output.borrow();

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

        frame = ref_frame.return_cell();
        previous_input = ref_input.return_cell();
        previous_logic = ref_logic.return_cell();
        previous_output = ref_output.return_cell();

        // determine, whether to continue, return error or exit
        if matches!(input_state, GameloopState::WantsToContinue)
            && matches!(logic_state, GameloopState::WantsToContinue)
            && matches!(output_state, GameloopState::WantsToContinue)
        {
            continue;
        }

        if matches!(input_state, GameloopState::WantsToRestart)
            || matches!(logic_state, GameloopState::WantsToRestart)
            || matches!(output_state, GameloopState::WantsToRestart)
        {
            return Ok(RESTART_CODE);
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

        return Ok(0);
    }
}
