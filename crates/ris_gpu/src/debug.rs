use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_void;

use ash::vk;

use ris_error::RisResult;
use ris_log::log_level::LogLevel;

const USE_LOGGER: bool = true; // uses eprintln!() when false
const BACKTRACE_LOG_LEVEL: LogLevel = LogLevel::Error;

#[cfg(debug_assertions)]
const VALIDATION_LAYERS: &[&str] = &["VK_LAYER_KHRONOS_validation"];

pub fn get_layers(
    entry: &ash::Entry,
    instance_extensions: &mut Vec<*const i8>,
) -> RisResult<Vec<CString>> {
    #[cfg(not(debug_assertions))]
    {
        _ = entry;
        _ = instance_extensions;
        Ok(Vec::with_capacity(0))
    }

    #[cfg(debug_assertions)]
    {
        // add debug util extension
        instance_extensions.push(ash::extensions::ext::DebugUtils::name().as_ptr());

        // find and collect available layers
        let layer_properties = entry.enumerate_instance_layer_properties()?;
        if layer_properties.is_empty() {
            ris_log::warning!("no available instance layers");
            Ok(Vec::with_capacity(0))
        } else {
            let mut log_message = String::from("available instance layers:");
            for layer in layer_properties.iter() {
                let name = super::util::vk_to_std_str(&layer.layer_name)?;
                log_message.push_str(&format!("\n\t- {}", name));
            }
            ris_log::trace!("{}", log_message);

            let mut available_layers = Vec::new();
            let mut log_message = String::from("instance layers to be enabled:");

            for &required_layer in VALIDATION_LAYERS {
                let mut layer_found = false;

                for layer in layer_properties.iter() {
                    let name = super::util::vk_to_c_str(&layer.layer_name);
                    if required_layer == name.to_str()? {
                        let owned = name.to_owned();
                        available_layers.push(owned);
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
            Ok(available_layers)
        }
    }
}

pub struct Debugger {
    #[cfg(debug_assertions)]
    inner: DebuggerInner,
}

#[cfg(debug_assertions)]
struct DebuggerInner {
    utils: ash::extensions::ext::DebugUtils,
    messenger: vk::DebugUtilsMessengerEXT,
}

impl Debugger {
    /// # Safety
    ///
    /// - May only be called once. Memory must not be freed twice.
    /// - This object must not be used after it was freed
    pub unsafe fn free(&self) {
        #[cfg(debug_assertions)]
        {
            self.inner
                .utils
                .destroy_debug_utils_messenger(self.inner.messenger, None);
        }
    }

    pub fn alloc(entry: &ash::Entry, instance: &ash::Instance) -> RisResult<Self> {
        #[cfg(not(debug_assertions))]
        {
            _ = entry;
            _ = instance;
            Ok(Self {})
        }

        #[cfg(debug_assertions)]
        {
            let debug_utils = ash::extensions::ext::DebugUtils::new(entry, instance);

            let debug_utils_messenger_create_info = vk::DebugUtilsMessengerCreateInfoEXT {
                s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
                p_next: std::ptr::null(),
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
                p_user_data: std::ptr::null_mut(),
            };

            let debug_utils_messenger = unsafe {
                debug_utils
                    .create_debug_utils_messenger(&debug_utils_messenger_create_info, None)?
            };

            let inner = DebuggerInner {
                utils: debug_utils,
                messenger: debug_utils_messenger,
            };
            Ok(Self { inner })
        }
    }

    pub fn set_name<T: vk::Handle + 'static>(
        &self,
        device: &ash::Device,
        object: T,
        name: impl AsRef<str>,
    ) -> RisResult<()> {
        #[cfg(not(debug_assertions))]
        {
            _ = device;
            _ = object;
            _ = name;
            Ok(())
        }

        #[cfg(debug_assertions)]
        {
            use std::any::TypeId;

            let name = CString::new(name.as_ref())?;

            let type_id = std::any::TypeId::of::<T>();
            let object_type = if type_id == TypeId::of::<vk::Instance>() {
                vk::ObjectType::INSTANCE
            } else if type_id == TypeId::of::<vk::PhysicalDevice>() {
                vk::ObjectType::PHYSICAL_DEVICE
            } else if type_id == TypeId::of::<vk::Device>() {
                vk::ObjectType::DEVICE
            } else if type_id == TypeId::of::<vk::Queue>() {
                vk::ObjectType::QUEUE
            } else if type_id == TypeId::of::<vk::Semaphore>() {
                vk::ObjectType::SEMAPHORE
            } else if type_id == TypeId::of::<vk::CommandBuffer>() {
                vk::ObjectType::COMMAND_BUFFER
            } else if type_id == TypeId::of::<vk::Fence>() {
                vk::ObjectType::FENCE
            } else if type_id == TypeId::of::<vk::DeviceMemory>() {
                vk::ObjectType::DEVICE_MEMORY
            } else if type_id == TypeId::of::<vk::Buffer>() {
                vk::ObjectType::BUFFER
            } else if type_id == TypeId::of::<vk::Image>() {
                vk::ObjectType::IMAGE
            } else if type_id == TypeId::of::<vk::Event>() {
                vk::ObjectType::EVENT
            } else if type_id == TypeId::of::<vk::QueryPool>() {
                vk::ObjectType::QUERY_POOL
            } else if type_id == TypeId::of::<vk::BufferView>() {
                vk::ObjectType::BUFFER_VIEW
            } else if type_id == TypeId::of::<vk::ImageView>() {
                vk::ObjectType::IMAGE_VIEW
            } else if type_id == TypeId::of::<vk::ShaderModule>() {
                vk::ObjectType::SHADER_MODULE
            } else if type_id == TypeId::of::<vk::PipelineCache>() {
                vk::ObjectType::PIPELINE_CACHE
            } else if type_id == TypeId::of::<vk::PipelineLayout>() {
                vk::ObjectType::PIPELINE_LAYOUT
            } else if type_id == TypeId::of::<vk::RenderPass>() {
                vk::ObjectType::RENDER_PASS
            } else if type_id == TypeId::of::<vk::Pipeline>() {
                vk::ObjectType::PIPELINE
            } else if type_id == TypeId::of::<vk::DescriptorSetLayout>() {
                vk::ObjectType::DESCRIPTOR_SET_LAYOUT
            } else if type_id == TypeId::of::<vk::Sampler>() {
                vk::ObjectType::SAMPLER
            } else if type_id == TypeId::of::<vk::DescriptorPool>() {
                vk::ObjectType::DESCRIPTOR_POOL
            } else if type_id == TypeId::of::<vk::DescriptorSet>() {
                vk::ObjectType::DESCRIPTOR_SET
            } else if type_id == TypeId::of::<vk::Framebuffer>() {
                vk::ObjectType::FRAMEBUFFER
            } else if type_id == TypeId::of::<vk::CommandPool>() {
                vk::ObjectType::COMMAND_POOL
            } else if type_id == TypeId::of::<vk::DescriptorUpdateTemplate>() {
                vk::ObjectType::DESCRIPTOR_UPDATE_TEMPLATE
            } else if type_id == TypeId::of::<vk::SamplerYcbcrConversion>() {
                vk::ObjectType::SAMPLER_YCBCR_CONVERSION
            } else if type_id == TypeId::of::<vk::PrivateDataSlot>() {
                vk::ObjectType::PRIVATE_DATA_SLOT
            } else if type_id == TypeId::of::<vk::SurfaceKHR>() {
                vk::ObjectType::SURFACE_KHR
            } else if type_id == TypeId::of::<vk::SwapchainKHR>() {
                vk::ObjectType::SWAPCHAIN_KHR
            } else if type_id == TypeId::of::<vk::DisplayKHR>() {
                vk::ObjectType::DISPLAY_KHR
            } else if type_id == TypeId::of::<vk::DisplayModeKHR>() {
                vk::ObjectType::DISPLAY_MODE_KHR
            } else if type_id == TypeId::of::<vk::DebugReportCallbackEXT>() {
                vk::ObjectType::DEBUG_REPORT_CALLBACK_EXT
            } else if type_id == TypeId::of::<vk::VideoSessionKHR>() {
                vk::ObjectType::VIDEO_SESSION_KHR
            } else if type_id == TypeId::of::<vk::VideoSessionParametersKHR>() {
                vk::ObjectType::VIDEO_SESSION_PARAMETERS_KHR
            } else if type_id == TypeId::of::<vk::CuModuleNVX>() {
                vk::ObjectType::CU_MODULE_NVX
            } else if type_id == TypeId::of::<vk::CuFunctionNVX>() {
                vk::ObjectType::CU_FUNCTION_NVX
            } else if type_id == TypeId::of::<vk::DebugUtilsMessengerEXT>() {
                vk::ObjectType::DEBUG_UTILS_MESSENGER_EXT
            } else if type_id == TypeId::of::<vk::AccelerationStructureKHR>() {
                vk::ObjectType::ACCELERATION_STRUCTURE_KHR
            } else if type_id == TypeId::of::<vk::ValidationCacheEXT>() {
                vk::ObjectType::VALIDATION_CACHE_EXT
            } else if type_id == TypeId::of::<vk::AccelerationStructureNV>() {
                vk::ObjectType::ACCELERATION_STRUCTURE_NV
            } else if type_id == TypeId::of::<vk::PerformanceConfigurationINTEL>() {
                vk::ObjectType::PERFORMANCE_CONFIGURATION_INTEL
            } else if type_id == TypeId::of::<vk::DeferredOperationKHR>() {
                vk::ObjectType::DEFERRED_OPERATION_KHR
            } else if type_id == TypeId::of::<vk::IndirectCommandsLayoutNV>() {
                vk::ObjectType::INDIRECT_COMMANDS_LAYOUT_NV
            //} else if type_id == TypeId::of::<>() { vk::ObjectType::CUDA_MODULE_NV
            //} else if type_id == TypeId::of::<>() { vk::ObjectType::CUDA_FUNCTION_NV
            } else if type_id == TypeId::of::<vk::BufferCollectionFUCHSIA>() {
                vk::ObjectType::BUFFER_COLLECTION_FUCHSIA
            } else if type_id == TypeId::of::<vk::MicromapEXT>() {
                vk::ObjectType::MICROMAP_EXT
            //} else if type_id == TypeId::of::<>() { vk::ObjectType::TENSOR_ARM
            //} else if type_id == TypeId::of::<>() { vk::ObjectType::TENSOR_VIEW_ARM
            } else if type_id == TypeId::of::<vk::OpticalFlowSessionNV>() {
                vk::ObjectType::OPTICAL_FLOW_SESSION_NV
            } else if type_id == TypeId::of::<vk::ShaderEXT>() {
                vk::ObjectType::SHADER_EXT
            //} else if type_id == TypeId::of::<>() { vk::ObjectType::PIPELINE_BINARY_KHR
            //} else if type_id == TypeId::of::<>() { vk::ObjectType::DATA_GRAPH_PIPELINE_SESSION_ARM
            //} else if type_id == TypeId::of::<>() { vk::ObjectType::EXTERNAL_COMPUTE_QUEUE_NV
            //} else if type_id == TypeId::of::<>() { vk::ObjectType::INDIRECT_COMMANDS_LAYOUT_EXT
            //} else if type_id == TypeId::of::<>() { vk::ObjectType::INDIRECT_EXECUTION_SET_EXT
            } else if type_id == TypeId::of::<vk::DescriptorUpdateTemplateKHR>() {
                vk::ObjectType::DESCRIPTOR_UPDATE_TEMPLATE_KHR
            } else if type_id == TypeId::of::<vk::SamplerYcbcrConversionKHR>() {
                vk::ObjectType::SAMPLER_YCBCR_CONVERSION_KHR
            } else if type_id == TypeId::of::<vk::PrivateDataSlotEXT>() {
                vk::ObjectType::PRIVATE_DATA_SLOT_EXT
            } else {
                vk::ObjectType::UNKNOWN
            };

            let info = vk::DebugUtilsObjectNameInfoEXT {
                s_type: vk::StructureType::DEBUG_UTILS_OBJECT_NAME_INFO_EXT,
                p_next: std::ptr::null(),
                object_type,
                object_handle: object.as_raw(),
                p_object_name: name.as_ptr(),
            };

            unsafe {
                self.inner
                    .utils
                    .set_debug_utils_object_name(device.handle(), &info)
            }?;

            Ok(())
        }
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

    if USE_LOGGER {
        ris_log::log!(
            priority,
            "VULKAN {} | {}{}",
            type_flag,
            message,
            backtrace_string,
        )
    } else {
        eprintln!("VULKAN {} | {}{}", type_flag, message, backtrace_string,)
    }

    vk::FALSE
}
