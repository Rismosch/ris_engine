use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, logic_data::LogicData,
    output_data::OutputData,
};

pub fn run(
    current: OutputData,
    _previous: &OutputData,
    _logic: &LogicData,
    _frame: &FrameData,
) -> (OutputData, GameloopState) {
    (current, GameloopState::WantsToContinue)
}
