use imgui::Ui;

use ris_error::RisResult;

use crate::gameloop::frame::Frame;
use crate::god_state::GodState;

pub struct InspectUpdate<'a> {
    pub ui: &'a Ui,
    pub frame: Frame,
    pub state: &'a GodState,
}

pub trait IInspectable {
    fn name() -> &'static str;
    fn inspect(&mut self, data: InspectUpdate) -> RisResult<()>;
}
