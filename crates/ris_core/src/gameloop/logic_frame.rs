// use std::{thread, time::Duration};

use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
    logic_data::LogicData,
};
use ris_jobs::job_cell::Ref;
// use ris_jobs::job_system;

pub fn run(
    current: LogicData,
    _previous: Ref<LogicData>,
    _input: Ref<InputData>,
    _frame: Ref<FrameData>,
) -> (LogicData, GameloopState) {
    // thread::sleep(Duration::from_millis(50));

    // ris_log::debug!(
    //     "{:#034b} {} {}",
    //     _input.general.buttons.hold(),
    //     job_system::thread_index(),
    //     _frame.fps()
    // );

    (current, GameloopState::WantsToContinue)
}
