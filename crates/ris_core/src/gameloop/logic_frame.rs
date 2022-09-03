use ris_data::gameloop::{input_data::InputData, frame_data::FrameData, logic_data::LogicData, gameloop_state::GameloopState};

pub fn run_logic(
    current: &mut LogicData,
    previous: &LogicData,
    input: &InputData,
    frame: &FrameData) -> GameloopState
{
    GameloopState::WantsToContinue
}