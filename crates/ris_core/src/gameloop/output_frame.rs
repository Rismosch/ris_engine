use ris_data::gameloop::{
    frame_data::FrameData,
    gameloop_state::GameloopState,
    input_data::InputData,
    logic_data::LogicData,
    output_data::OutputData,
};
use ris_video::video::{Video, DrawState};

pub struct OutputFrame {
    video: Video,
    recreate_swapchain: bool,
}


impl OutputFrame {
    pub fn new(video: Video) -> Result<Self, String> {
        Ok(Self {
            video,
            recreate_swapchain: false,
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
        if let Err(error) = self.video.recreate_swapchain(
            input.window_size_changed.is_some(),
            self.recreate_swapchain,
        ) {
            return GameloopState::Error(error);
        }

        match self.video.draw() {
            DrawState::Ok => (),
            DrawState::RecreateSwapchain => self.recreate_swapchain = true,
            DrawState::Err(e) => return GameloopState::Error(e),
        }

        GameloopState::WantsToContinue
    }
}
