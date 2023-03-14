use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
    logic_data::LogicData, output_data::OutputData,
};
use ris_jobs::{job_cell::JobCell, job_system};

use crate::{
    gameloop::{logic_frame, output_frame},
    god_object::GodObject,
};

pub fn run(mut god_object: GodObject) -> GameloopState {
    let mut frame = JobCell::new(FrameData::default());

    let mut current_input = InputData::default();
    let mut previous_input = JobCell::new(InputData::default());

    let mut current_logic = LogicData::default();
    let mut previous_logic = JobCell::new(LogicData::default());

    let mut current_output = OutputData::default();
    let mut previous_output = JobCell::new(OutputData::default());

    loop {
        // update frame
        frame.as_mut().bump();

        // swap buffers
        current_input = previous_input.as_mut().replace(current_input);
        current_logic = previous_logic.as_mut().replace(current_logic);
        current_output = previous_output.as_mut().replace(current_output);

        // create references
        let frame_for_input = frame.borrow();
        let frame_for_logic = frame.borrow();
        let frame_for_output = frame.borrow();

        let previous_input_for_input = previous_input.borrow();
        let previous_input_for_logic = previous_input.borrow();

        let previous_logic_for_logic = previous_logic.borrow();
        let previous_logic_for_output = previous_logic.borrow();

        let previous_output_for_output = previous_output.borrow();

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

        let (new_input_data, input_state) =
            god_object
                .input_frame
                .run(current_input, previous_input_for_input, frame_for_input);

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

        if matches!(input_state, GameloopState::WantsToRestart)
            || matches!(logic_state, GameloopState::WantsToRestart)
            || matches!(output_state, GameloopState::WantsToRestart)
        {
            return GameloopState::WantsToRestart;
        }

        if let GameloopState::Error(_) = input_state {
            return input_state;
        }

        if let GameloopState::Error(_) = logic_state {
            return logic_state;
        }

        if let GameloopState::Error(_) = output_state {
            return output_state;
        }

        return GameloopState::WantsToQuit;
    }
}
