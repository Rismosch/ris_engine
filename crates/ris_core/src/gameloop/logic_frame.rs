use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
    logic_data::LogicData,
};
use ris_jobs::job_system;

#[derive(Default)]
pub struct LogicFrame{
    camera_horizontal_angle: f32,
    camera_vertical_angle: f32,
}

impl LogicFrame {
    pub fn run(
        &mut self,
        _current: &mut LogicData,
        _previous: &LogicData,
        input: &InputData,
        frame: &FrameData,
    ) -> GameloopState {

        if input.general.buttons.down() != 0 {
            ris_log::debug!(
                "{:#010x} worker: {}, {}ns ({}fps)",
                input.general.buttons.down(),
                job_system::thread_index(),
                frame.delta().as_nanos(),
                frame.fps(),
            );
        }

        GameloopState::WantsToContinue
    }
}

