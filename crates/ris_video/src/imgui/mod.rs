pub mod backend;
pub mod renderer;

pub struct RisImgui {
    pub backend: backend::ImguiBackend,
    pub renderer: renderer::ImguiRenderer,
}
