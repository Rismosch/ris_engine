use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, logic_data::LogicData,
    output_data::OutputData,
};

pub fn run(
    current: OutputData,
    previous: &OutputData,
    logic: &LogicData,
    frame: &FrameData,
) -> (OutputData, GameloopState) {
    (current, GameloopState::WantsToContinue)
}
