use ris_data::gameloop::frame_data::FrameData;
use ris_data::gameloop::logic_data::LogicData;
use ris_data::gameloop::output_data::OutputData;
use ris_util::error::RisResult;
use ris_video::video::Video;

pub struct OutputFrame {
    video: Video,
}

impl OutputFrame {
    pub fn new(video: Video) -> Self {
        Self { video }
    }

    pub fn run(
        &mut self,
        _current: &mut OutputData,
        _previous: &OutputData,
        logic: &LogicData,
        _frame: &FrameData,
    ) -> RisResult<()> {
        if logic.reload_shaders {
            self.video.recreate_viewport(true);
        } else if logic.window_size_changed.is_some() {
            self.video.recreate_viewport(false);
        }

        self.video.update(&logic.scene)?;

        Ok(())
    }
}