use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
    logic_data::LogicData,
};

pub fn run(
    current: LogicData,
    _previous: &LogicData,
    _input: &InputData,
    _frame: &'static FrameData,
) -> (LogicData, GameloopState) {
    // thread::sleep(Duration::from_millis(50));

    // ris_log::debug!("{:#034b} {} {}", input.get_keyboard().buttons.down(), job_system::thread_index(), frame.fps());

    (current, GameloopState::WantsToContinue)
}
