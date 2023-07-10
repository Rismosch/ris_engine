use ris_data::gameloop::{
    frame_data::FrameData,
    gameloop_state::GameloopState,
    input_data::InputData,
    logic_data::LogicData,
    output_data::OutputData,
};
use ris_video::video::Video;

pub struct OutputFrame {
    video: Video,
}


impl OutputFrame {
    pub fn new(video: Video) -> Result<Self, String> {
        Ok(Self {
            video
        })
    }

    pub fn run(
        &mut self,
        _current: &mut OutputData,
        _previous: &OutputData,
        input: &InputData,
        _logic: &LogicData,
        _frame: &FrameData,
    ) -> GameloopState {
        if input.window_size_changed.is_some() {
            self.video.recreate_swapchain();
        }

        GameloopState::WantsToContinue
    }
}
