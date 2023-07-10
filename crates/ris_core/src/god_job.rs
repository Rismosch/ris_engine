use ris_data::gameloop::{
    frame_data::FrameDataCalculator, gameloop_state::GameloopState, input_data::InputData,
    logic_data::LogicData, output_data::OutputData,
};
use ris_jobs::job_system;

use crate::{gameloop::logic_frame, god_object::GodObject};

pub fn run(mut god_object: GodObject) -> GameloopState {
    let mut frame_data_calculator = FrameDataCalculator::default();

    let mut current_input = InputData::default();
    let mut current_logic = LogicData::default();
    let mut current_output = OutputData::default();

    loop {
        // update frame
        frame_data_calculator.bump();
        let current_frame = frame_data_calculator.current();

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

        // submit jobs
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
            let mut current_logic = current_logic;
            let gameloop_state = logic_frame::run(
                &mut current_logic,
                &previous_logic_for_logic,
                &previous_input_for_logic,
                &frame_for_logic,
            );

            (current_logic, gameloop_state)
        });

        let input_state = god_object.input_frame.run(
            &mut current_input,
            &previous_input_for_input,
            &frame_for_input,
        );

        // wait for jobs
        let (new_logic_data, logic_state) = logic_future.wait();
        let (new_output_frame, new_output_data, output_state) = output_future.wait();

        // update buffers
        current_logic = new_logic_data;
        current_output = new_output_data;
        god_object.output_frame = new_output_frame;

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
