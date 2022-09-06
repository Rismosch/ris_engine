use std::cell::{Ref, RefMut};

use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
    logic_data::LogicData, output_data::OutputData,
};

pub fn run(
    current: &mut OutputData,
    previous: &OutputData,
    logic: &LogicData,
    frame: &FrameData,
) -> GameloopState {
    
    GameloopState::WantsToContinue
}
