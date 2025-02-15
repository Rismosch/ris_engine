use std::ffi::CString;
use std::marker::PhantomData;
use std::mem::size_of;

use crate::bindings::imgui as sys;

pub const IMGUI_VERSION: &str = "1.91.9 WIP";

pub const IMGUI_CONFIG_FLAGS_NONE: i32 = sys::ImGuiConfigFlags__ImGuiConfigFlags_None;
pub const IMGUI_CONFIG_FLAGS_NAV_ENABLE_KEYBOARD: i32 = sys::ImGuiConfigFlags__ImGuiConfigFlags_NavEnableKeyboard;
pub const IMGUI_CONFIG_FLAGS_NAV_ENABLE_GAMEPAD: i32 = sys::ImGuiConfigFlags__ImGuiConfigFlags_NavEnableGamepad;
pub const IMGUI_CONFIG_FLAGS_NO_MOUSE: i32 = sys::ImGuiConfigFlags__ImGuiConfigFlags_NoMouse;
pub const IMGUI_CONFIG_FLAGS_NO_MOUSE_CURSOR_CHANGE: i32 = sys::ImGuiConfigFlags__ImGuiConfigFlags_NoMouseCursorChange;
pub const IMGUI_CONFIG_FLAGS_NO_KEYBOARD: i32 = sys::ImGuiConfigFlags__ImGuiConfigFlags_NoKeyboard;
pub const IMGUI_CONFIG_FLAGS_DOCKING_ENABLE: i32 = sys::ImGuiConfigFlags__ImGuiConfigFlags_DockingEnable;
pub const IMGUI_CONFIG_FLAGS_VIEWPORTS_ENABLE: i32 = sys::ImGuiConfigFlags__ImGuiConfigFlags_ViewportsEnable;
pub const IMGUI_CONFIG_FLAGS_DPI_ENABLE_SCALE_VIEWPORTS: i32 = sys::ImGuiConfigFlags__ImGuiConfigFlags_DpiEnableScaleViewports;
pub const IMGUI_CONFIG_FLAGS_DPI_ENABLE_SCALE_FONTS: i32 = sys::ImGuiConfigFlags__ImGuiConfigFlags_DpiEnableScaleFonts;
pub const IMGUI_CONFIG_FLAGS_IS_SRGB: i32 = sys::ImGuiConfigFlags__ImGuiConfigFlags_IsSRGB;
pub const IMGUI_CONFIG_FLAGS_IS_TOUCH_SCREEN: i32 = sys::ImGuiConfigFlags__ImGuiConfigFlags_IsTouchScreen;

pub const IMGUI_COL_TEXT: i32 = sys::ImGuiCol__ImGuiCol_Text;

pub struct ImGuiContext {
    ptr: *mut sys::ImGuiContext,
}

impl Drop for ImGuiContext {
    fn drop(&mut self) {
        unsafe{sys::ImGui_DestroyContext(self.ptr)}
    }
}

pub struct ImGuiIO<'a> {
    boo: PhantomData<&'a mut sys::ImGuiIO>,
    ptr: *mut sys::ImGuiIO,
}

impl<'a> ImGuiIO<'a> {
    pub fn config_flags(&self) -> i32 {
        unsafe{(*self.ptr).ConfigFlags}
    }

    pub fn set_config_flags(&mut self, value: i32) {
        unsafe{(*self.ptr).ConfigFlags = value};
    }
}

pub fn imgui_checkversion() -> bool {
    let Ok(version) = CString::new(IMGUI_VERSION) else {
        return false;
    };

    unsafe {
        sys::ImGui_DebugCheckVersionAndDataLayout(
            version.as_ptr(),
            size_of::<sys::ImGuiIO>(),
            size_of::<sys::ImGuiStyle>(),
            size_of::<sys::ImVec2>(),
            size_of::<sys::ImVec4>(),
            size_of::<sys::ImDrawVert>(),
            size_of::<sys::ImDrawIdx>(),
        )
    }
}

pub fn create_context() -> ImGuiContext {
    let ptr = unsafe {sys::ImGui_CreateContext(std::ptr::null_mut())};
    ImGuiContext {
        ptr,
    }
}

pub fn destroy_context(context: ImGuiContext) {
    let _ = context;
}

pub fn get_io<'a>(context: &'a mut ImGuiContext) -> ImGuiIO {
    let _ = context;
    let ptr = unsafe {sys::ImGui_GetIO()};

    ImGuiIO {
        boo: PhantomData,
        ptr,
    }
}

pub fn style_colors_dark(context: &mut ImGuiContext) {
    let _ = context;
    unsafe{sys::ImGui_StyleColorsDark(std::ptr::null_mut())}
}

pub fn style_colors_light(context: &mut ImGuiContext) {
    let _ = context;
    unsafe{sys::ImGui_StyleColorsLight(std::ptr::null_mut())}
}

pub fn style_colors_classic(context: &mut ImGuiContext) {
    let _ = context;
    unsafe{sys::ImGui_StyleColorsClassic(std::ptr::null_mut())}
}

