use std::{thread, time::{Duration, Instant}};

use ris_data::gameloop::{gameloop_state::GameloopState, input_data::{InputData, self}, logic_data::LogicData, output_data::OutputData, frame_data::FrameData};
use ris_jobs::job_system;

use crate::{
    god_object::GodObject, gameloop::{output_frame::run_output, logic_frame::run_logic, input_frame::run_input},
};

pub fn run(god_object: &mut GodObject) -> Result<(), String> {

    let mut current_input = InputData::default();
    let mut current_logic = LogicData::default();
    let mut current_output = OutputData::default();
    let previous_input = InputData::default();
    let previous_logic = LogicData::default();
    let previous_output = OutputData::default();

    // loop {

    //     let frame_data = FrameData::new();

    //     let previous_logic_output = &previous_logic;
    //     let output_future = job_system::submit(move ||run_output(
    //         &mut current_output,
    //         &previous_output,
    //         &previous_logic_output,
    //         &frame_data
    //     ));

    //     let logic_future = job_system::submit(|| run_logic(
    //         &mut current_logic,
    //         &previous_logic,
    //         &previous_input,
    //         &frame_data
    //     ));

    //     run_input(
    //         &mut current_input,
    //         &previous_input,
    //         &frame_data
    //     );

    //     job_system::wait(output_future);
    //     let gameloop_state = job_system::wait(logic_future);

    //     match gameloop_state {
    //         GameloopState::WantsToContinue => continue,
    //         GameloopState::WantsToQuit => break,
    //         GameloopState::Error(error) => return Err(error),
    //     }
    // }

    Ok(())
}