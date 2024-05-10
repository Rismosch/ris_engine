pub mod backend;
pub mod imgui_frames;
pub mod imgui_mesh;
pub mod imgui_renderer;

pub struct RisImgui {
    pub backend: backend::ImguiBackend,
    pub renderer: imgui_renderer::ImguiRenderer,
}
