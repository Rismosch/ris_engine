use sdl2::video::Window;
use sdl2_sys::SDL_Event;

use crate::bindings::backends::imgui_impl_sdl2;
use crate::bindings::imgui;

pub struct ImGuiBackends;

impl Drop for ImGuiBackends {
    fn drop(&mut self) {
        unsafe {imgui_impl_sdl2::ImGui_ImplSDL2_Shutdown()};
    }
}

impl ImGuiBackends {
    /// Safety: `window` must outlive `ImGuiBackends`
    pub unsafe fn init(
        window: &Window,
    ) -> Self {
        let window_ptr = window.raw() as *mut imgui_impl_sdl2::SDL_Window;
        imgui_impl_sdl2::ImGui_ImplSDL2_InitForVulkan(window_ptr);

        Self
    }

    pub unsafe fn process_event(&mut self, event: &SDL_Event) -> bool {
        let ptr = event as *const SDL_Event as *const imgui_impl_sdl2::SDL_Event;
        imgui_impl_sdl2::ImGui_ImplSDL2_ProcessEvent(ptr)
    }

    pub fn new_frame(&mut self) {
        unsafe {
            imgui_impl_sdl2::ImGui_NewFrame();
            imgui::ImGui_NewFrame();
            imgui::ImGui_ShowDemoWindow(&mut true);
        }
    }

    pub fn render(&mut self) {
        unsafe {
            imgui::ImGui_Render();
            let main_draw_data = imgui::ImGui_GetDrawData();
        }
    }
}

