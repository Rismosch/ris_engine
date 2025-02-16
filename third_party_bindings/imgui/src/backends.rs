use std::marker::PhantomData;

use sdl2::event::EventType;
use sdl2::video::Window;
use sdl2_sys::SDL_Event;

use crate::bindings::backends::imgui_impl_sdl2;
use crate::bindings::backends::imgui_impl_vulkan;

pub struct ImGuiBackends;

impl Drop for ImGuiBackends {
    fn drop(&mut self) {
        unsafe {
            //imgui_impl_vulkan::ImGui_ImplVulkan_Shutdown();
            imgui_impl_sdl2::ImGui_ImplSDL2_Shutdown();
        }
    }
}

impl ImGuiBackends {
    /// Safety: `window` must outlive `ImGuiBackends`
    pub unsafe fn init(window: &Window) -> Self {
        let ptr = window.raw() as *mut imgui_impl_sdl2::SDL_Window;

        unsafe {
            imgui_impl_sdl2::ImGui_ImplSDL2_InitForVulkan(ptr);
            //    imgui_impl_vulkan::ImGui_ImplVulkan_Init();
        }

        Self
    }

    pub unsafe fn process_event(&mut self, event: &SDL_Event) -> bool {
        let ptr = event as *const SDL_Event as *const imgui_impl_sdl2::SDL_Event;
        imgui_impl_sdl2::ImGui_ImplSDL2_ProcessEvent(ptr)
    }

    pub fn new_frame(&mut self) {
        unsafe {
            imgui_impl_vulkan::ImGui_NewFrame();
            imgui_impl_sdl2::ImGui_NewFrame();
        }
    }
}
