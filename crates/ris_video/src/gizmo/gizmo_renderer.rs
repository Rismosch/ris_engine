use ash::vk;

use ris_error::Extensions;
use ris_error::RisResult;

pub struct GizmoRenderer;

impl GizmoRenderer {
    pub fn free(&mut self, device: &ash::Device) {
    }
    
    pub fn init() -> RisResult<Self> {
        Ok(GizmoRenderer)
    }

    pub fn draw(&mut self) -> RisResult<()> {



        Ok(())
    }
}
