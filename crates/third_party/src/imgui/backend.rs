use sdl2::video::Window;
use sdl2_sys::SDL_Event;

use super::sys::backends::imgui_impl_sdl2;
use super::imgui::ImGuiContext;

pub struct ImGuiBackend {
    pub context: ImGuiContext,
}

impl Drop for ImGuiBackend {
    fn drop(&mut self) {
        unsafe {imgui_impl_sdl2::ImGui_ImplSDL2_Shutdown()};
    }
}

impl ImGuiBackend {
    /// Safety: `window` must outlive `ImGuiBackends`
    pub unsafe fn init(
        context: ImGuiContext,
        window: &Window,
    ) -> Self {
        let window_ptr = window.raw() as *mut imgui_impl_sdl2::SDL_Window;
        imgui_impl_sdl2::ImGui_ImplSDL2_InitForVulkan(window_ptr);

        Self {context}
    }

    pub unsafe fn process_event(&mut self, event: &SDL_Event) -> bool {
        let ptr = event as *const SDL_Event as *const imgui_impl_sdl2::SDL_Event;
        imgui_impl_sdl2::ImGui_ImplSDL2_ProcessEvent(ptr)
    }

    pub fn new_frame(&mut self) {
        unsafe {imgui_impl_sdl2::ImGui_ImplSDL2_NewFrame()};
    }
}

