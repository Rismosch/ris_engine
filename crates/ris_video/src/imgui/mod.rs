pub mod imgui_backend;
pub mod imgui_mesh;
pub mod imgui_renderer;

pub struct RisImgui {
    pub backend: imgui_backend::ImguiBackend,
    pub renderer: imgui_renderer::ImguiRenderer,
}
