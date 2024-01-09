use ris_data::gameloop::frame::Frame;
use ris_data::gameloop::logic_data::LogicData;
use ris_data::gameloop::output_data::OutputData;
use ris_error::RisResult;
use ris_video::imgui::backend::ImguiBackend;
use ris_video::video::Video;

pub struct OutputFrame {
    video: Video,
    imgui: Option<ImguiBackend>,
}

impl OutputFrame {
    pub fn new(video: Video, imgui: Option<ImguiBackend>) -> Self {
        Self { video, imgui }
    }

    pub fn run(
        &mut self,
        _current: &mut OutputData,
        _previous: &OutputData,
        logic: &LogicData,
        frame: Frame,
    ) -> RisResult<()> {
        
        // render graphics
        if logic.reload_shaders {
            self.video.recreate_viewport(true);
        } else if logic.window_size_changed.is_some() {
            self.video.recreate_viewport(false);
        }

        self.video.update(&logic.scene)?;

        // render imgui
        if let Some(ris_imgui) = &mut self.imgui {
            let ui = ris_imgui.prepare_frame(logic, frame, &self.video);

            ui.show_demo_window(&mut true);

            ris_imgui.render();
        }

        Ok(())
    }
}
