use std::ffi::CStr;
use std::os::raw::c_void;

use ash::vk;

use ris_error::RisResult;

pub struct VkStr {
    value: String,
}

impl std::fmt::Display for VkStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl VkStr {
    /// # Safety
    ///
    /// See `std::ffi::Cstr::from_ptr()`
    pub unsafe fn from(value: &[i8]) -> RisResult<Self> {
        let cstr = unsafe {
            let ptr = value.as_ptr();
            CStr::from_ptr(ptr)
        };

        let result = cstr
            .to_str()?
            .to_owned();

        Ok(Self{
            value: result,
        })
    }

    pub unsafe fn as_ptr(&self) -> *const i8{
        self.value.as_ptr() as *const i8
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

pub unsafe extern "system" fn debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> vk::Bool32 {
    use ris_log::log_level::LogLevel;

    let log_level = match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => LogLevel::Trace,
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => LogLevel::Info,
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => LogLevel::Warning,
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => LogLevel::Error,
        _ => LogLevel::Debug,
    };

    let type_flag = match message_type {
        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "GENERAL",
        vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "PERFORMANCE",
        vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "VALIDATION",
        _ => "unknown",
    };

    let message_cstr = CStr::from_ptr((*p_callback_data).p_message);
    let message = match message_cstr.to_str() {
        Ok(message) => String::from(message),
        Err(e) => {
            ris_log::error!("the vulkan debug callback was called with invalid UTF-8 data. attempting to log cstr... error: {}", e);
            format!("{:?}", message_cstr)
        },
    };

    ris_log::log!(log_level, "VULKAN {} | {}", type_flag, message);

    vk::FALSE
}
