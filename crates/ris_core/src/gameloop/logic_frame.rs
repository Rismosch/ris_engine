use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
    logic_data::LogicData,
};

pub fn run(
    _current: &mut LogicData,
    _previous: &LogicData,
    _input: &InputData,
    _frame: &FrameData,
) -> GameloopState {
    // ris_log::debug!("{} {} {}", job_system::thread_index(), frame.number(), frame.fps());

    GameloopState::WantsToContinue
}
