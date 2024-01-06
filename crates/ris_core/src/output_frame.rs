use ris_data::gameloop::frame_data::FrameData;
use ris_data::gameloop::logic_data::LogicData;
use ris_data::gameloop::output_data::OutputData;
use ris_error::RisResult;
use ris_video::imgui::RisImgui;
use ris_video::video::Video;

pub struct OutputFrame {
    video: Video,
    imgui: Option<RisImgui>,
}

impl OutputFrame {
    pub fn new(video: Video, imgui: Option<RisImgui>) -> Self {
        Self { video, imgui }
    }

    pub fn run(
        &mut self,
        _current: &mut OutputData,
        _previous: &OutputData,
        logic: &LogicData,
        _frame: &FrameData,
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
            ris_imgui.set_input(logic);
            ris_imgui.prepare_frame();
            //let ui = ris_imgui.new_frame();

            //ui.show_demo_window(&mut true);

            //ris_imgui.render();
        }

        Ok(())
    }
}
