use std::ffi::CStr;
use std::os::raw::c_void;
use std::ptr;

use ash::vk;

use ris_error::RisResult;
use ris_log::log_level::LogLevel;

use super::util;

const BACKTRACE_LOG_LEVEL: LogLevel = LogLevel::None;

#[cfg(not(debug_assertions))]
const VALIDATION_ENABLED: bool = false;
#[cfg(debug_assertions)]
const VALIDATION_ENABLED: bool = true;
const VALIDATION_LAYERS: &[&str] = &["VK_LAYER_KHRONOS_validation"];

pub fn add_validation_layer(
    entry: &ash::Entry,
    instance_extensions: &mut Vec<*const i8>,
) -> RisResult<(u32, *const *const i8)> {
    let available_layers = if !VALIDATION_ENABLED {
        ris_log::debug!("validation layer are disabled");
        (0, ptr::null())
    } else {
        // add debug util extension
        instance_extensions.push(ash::extensions::ext::DebugUtils::name().as_ptr());

        // find and collect available layers
        let layer_properties = entry.enumerate_instance_layer_properties()?;
        if layer_properties.is_empty() {
            ris_log::warning!("no available instance layers");
            (0, ptr::null())
        } else {
            let mut log_message = String::from("available instance layers:");
            for layer in layer_properties.iter() {
                let name = unsafe { util::VkStr::from(&layer.layer_name) }?;
                log_message.push_str(&format!("\n\t- {}", name));
            }
            ris_log::trace!("{}", log_message);

            let mut available_layers = Vec::new();
            let mut log_message = String::from("instance layers to be enabled:");

            for required_layer in VALIDATION_LAYERS {
                let mut layer_found = false;

                for layer in layer_properties.iter() {
                    let name = unsafe { util::VkStr::from(&layer.layer_name) }?;
                    if (*required_layer) == name.as_str() {
                        available_layers.push(layer.layer_name.as_ptr());
                        layer_found = true;
                        break;
                    }
                }

                if !layer_found {
                    ris_log::warning!("layer \"{}\" is not available", required_layer);
                } else {
                    log_message.push_str(&format!("\n\t- {}", required_layer));
                }
            }

            ris_log::debug!("{}", log_message);

            (0, available_layers.as_ptr())
        }
    };

    Ok(available_layers)
}

pub fn setup_debugging(
    entry: &ash::Entry,
    instance: &ash::Instance,
) -> RisResult<Option<(ash::extensions::ext::DebugUtils, vk::DebugUtilsMessengerEXT)>> {
    if !VALIDATION_ENABLED {
        Ok(None)
    } else {
        let debug_utils = ash::extensions::ext::DebugUtils::new(entry, instance);

        let debug_utils_messenger_create_info = vk::DebugUtilsMessengerCreateInfoEXT {
            s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
            p_next: ptr::null(),
            flags: vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
            message_severity:
                //vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
                vk::DebugUtilsMessageSeverityFlagsEXT::INFO |
                vk::DebugUtilsMessageSeverityFlagsEXT::WARNING |
                vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            message_type:
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL |
                vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE |
                vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
            pfn_user_callback: Some(debug_callback),
            p_user_data: ptr::null_mut(),
        };

        let debug_utils_messenger = unsafe {
            debug_utils.create_debug_utils_messenger(&debug_utils_messenger_create_info, None)?
        };

        Ok(Some((debug_utils, debug_utils_messenger)))
    }
}

/// # Safety
///
/// dereferences `p_callback_data`.
pub unsafe extern "system" fn debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> vk::Bool32 {
    let priority = match message_severity {
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
        }
    };

    let log_backtrace = ris_log::log::can_log(BACKTRACE_LOG_LEVEL, priority);

    let backtrace_string = if log_backtrace {
        let backtrace = std::backtrace::Backtrace::force_capture();
        format!("\nbackrace:\n{}", backtrace)
    } else {
        String::new()
    };

    ris_log::log!(
        priority,
        "VULKAN {} | {}{}",
        type_flag,
        message,
        backtrace_string,
    );

    vk::FALSE
}
