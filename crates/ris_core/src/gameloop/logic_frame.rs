use std::cell::{RefMut, Ref};

use ris_data::gameloop::{input_data::InputData, frame_data::FrameData, logic_data::LogicData, gameloop_state::GameloopState};

pub fn run(
    current: &mut LogicData,
    previous: &LogicData,
    frame: &FrameData) -> GameloopState
{
    GameloopState::WantsToContinue
}