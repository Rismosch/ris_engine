pub mod backend;
pub mod gpu_objects;
pub mod pipeline;
pub mod render_pass;
pub mod renderer;

pub struct RisImgui {
    pub backend: backend::ImguiBackend,
    pub renderer: renderer::ImguiRenderer,
}
