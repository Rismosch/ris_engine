use std::{cell::{Ref, RefMut}, thread, time::Duration};

use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
    logic_data::LogicData,
};
use ris_jobs::job_system;

pub fn run(
    current: &mut LogicData,
    previous: &LogicData,
    input: &InputData,
    frame: &FrameData,
) -> GameloopState {
    
    // ris_log::debug!("{} {} {}", job_system::thread_index(), frame.number(), frame.fps());

    GameloopState::WantsToContinue
}
