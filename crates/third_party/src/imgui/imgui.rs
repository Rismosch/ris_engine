use std::ffi::CString;
use std::marker::PhantomData;
use std::mem::size_of;

use super::sys;

pub const IMGUI_VERSION: &str = "1.91.9 WIP";

pub const IMGUI_CONFIG_FLAGS_NONE: i32 = sys::imgui::ImGuiConfigFlags__ImGuiConfigFlags_None;
pub const IMGUI_CONFIG_FLAGS_NAV_ENABLE_KEYBOARD: i32 =
    sys::imgui::ImGuiConfigFlags__ImGuiConfigFlags_NavEnableKeyboard;
pub const IMGUI_CONFIG_FLAGS_NAV_ENABLE_GAMEPAD: i32 =
    sys::imgui::ImGuiConfigFlags__ImGuiConfigFlags_NavEnableGamepad;
pub const IMGUI_CONFIG_FLAGS_NO_MOUSE: i32 = sys::imgui::ImGuiConfigFlags__ImGuiConfigFlags_NoMouse;
pub const IMGUI_CONFIG_FLAGS_NO_MOUSE_CURSOR_CHANGE: i32 =
    sys::imgui::ImGuiConfigFlags__ImGuiConfigFlags_NoMouseCursorChange;
pub const IMGUI_CONFIG_FLAGS_NO_KEYBOARD: i32 = sys::imgui::ImGuiConfigFlags__ImGuiConfigFlags_NoKeyboard;
pub const IMGUI_CONFIG_FLAGS_DOCKING_ENABLE: i32 =
    sys::imgui::ImGuiConfigFlags__ImGuiConfigFlags_DockingEnable;
pub const IMGUI_CONFIG_FLAGS_VIEWPORTS_ENABLE: i32 =
    sys::imgui::ImGuiConfigFlags__ImGuiConfigFlags_ViewportsEnable;
pub const IMGUI_CONFIG_FLAGS_DPI_ENABLE_SCALE_VIEWPORTS: i32 =
    sys::imgui::ImGuiConfigFlags__ImGuiConfigFlags_DpiEnableScaleViewports;
pub const IMGUI_CONFIG_FLAGS_DPI_ENABLE_SCALE_FONTS: i32 =
    sys::imgui::ImGuiConfigFlags__ImGuiConfigFlags_DpiEnableScaleFonts;
pub const IMGUI_CONFIG_FLAGS_IS_SRGB: i32 = sys::imgui::ImGuiConfigFlags__ImGuiConfigFlags_IsSRGB;
pub const IMGUI_CONFIG_FLAGS_IS_TOUCH_SCREEN: i32 =
    sys::imgui::ImGuiConfigFlags__ImGuiConfigFlags_IsTouchScreen;

//pub const IMGUI_COL_TEXT: i32 = sys::imgui::ImGuiCol__ImGuiCol_Text;

pub fn imgui_checkversion() -> bool {
    let Ok(version) = CString::new(IMGUI_VERSION) else {
        return false;
    };

    unsafe {
        sys::imgui::ImGui_DebugCheckVersionAndDataLayout(
            version.as_ptr(),
            size_of::<sys::imgui::ImGuiIO>(),
            size_of::<sys::imgui::ImGuiStyle>(),
            size_of::<sys::imgui::ImVec2>(),
            size_of::<sys::imgui::ImVec4>(),
            size_of::<sys::imgui::ImDrawVert>(),
            size_of::<sys::imgui::ImDrawIdx>(),
        )
    }
}

pub struct ImGuiContext {
    ptr: *mut sys::imgui::ImGuiContext,
}

pub struct ImGuiIO<'a> {
    boo: PhantomData<&'a mut sys::imgui::ImGuiIO>,
    ptr: *mut sys::imgui::ImGuiIO,
}

pub struct ImFontAtlas<'a> {
    boo: PhantomData<&'a mut ImGuiIO<'a>>,
    ptr: *mut sys::imgui::ImFontAtlas,
}

pub struct ImDrawData<'a> {
    boo: PhantomData<&'a mut ImGuiContext>,
    ptr: *mut sys::imgui::ImDrawData,
}

impl Drop for ImGuiContext {
    fn drop(&mut self) {
        unsafe { sys::imgui::ImGui_DestroyContext(self.ptr) }
    }
}

impl ImGuiContext {
    pub fn create() -> Self {
        let ptr = unsafe { sys::imgui::ImGui_CreateContext(std::ptr::null_mut()) };
        Self { ptr }
    }

    pub fn get_io<'a>(&'a mut self) -> ImGuiIO {
        let _ = self;
        let ptr = unsafe { sys::imgui::ImGui_GetIO() };

        ImGuiIO {
            boo: PhantomData,
            ptr,
        }
    }

    pub fn style_colors_dark(&mut self) {
        let _ = self;
        unsafe { sys::imgui::ImGui_StyleColorsDark(std::ptr::null_mut()) }
    }

    pub fn style_colors_light(&mut self) {
        let _ = self;
        unsafe { sys::imgui::ImGui_StyleColorsLight(std::ptr::null_mut()) }
    }

    pub fn style_colors_classic(&mut self) {
        let _ = self;
        unsafe { sys::imgui::ImGui_StyleColorsClassic(std::ptr::null_mut()) }
    }

    pub fn new_frame(&mut self) {
        let _ = self;
        unsafe {sys::imgui::ImGui_NewFrame()}
    }

    pub fn render<'a>(&mut self) -> ImDrawData<'a> {
        let _ = self;
        let ptr = unsafe {
            sys::imgui::ImGui_Render();
            sys::imgui::ImGui_GetDrawData()
        };

        ImDrawData {
            boo: PhantomData,
            ptr,
        }
    }
}

impl<'a> ImGuiIO<'a> {
    pub fn config_flags(&self) -> i32 {
        unsafe { (*self.ptr).ConfigFlags }
    }

    pub fn set_config_flags(&mut self, value: i32) {
        unsafe { (*self.ptr).ConfigFlags = value };
    }

    pub fn fonts(&mut self) -> ImFontAtlas {
        let _ = self;
        let ptr = unsafe {(*self.ptr).Fonts};

        ImFontAtlas {
            boo: PhantomData,
            ptr,
        }
    }
}

impl <'a> ImFontAtlas<'a> {
    pub fn add_font_default(&mut self) {
        unsafe {(*self.ptr).AddFontDefault(std::ptr::null())};
    }

    pub fn build(&mut self) -> bool {
        unsafe { (*self.ptr).Build() }
    }
}
