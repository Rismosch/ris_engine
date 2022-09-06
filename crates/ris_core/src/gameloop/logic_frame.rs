use std::cell::{Ref, RefMut};

use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
    logic_data::LogicData,
};

pub fn run(
    current: &mut LogicData,
    previous: &LogicData,
    input: &InputData,
    frame: &FrameData,
) -> GameloopState {
    

    GameloopState::WantsToContinue
}
