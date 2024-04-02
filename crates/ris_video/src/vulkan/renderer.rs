use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

use ash::vk;
use sdl2::video::Window;
use vulkano::swapchain::AcquireError;
use vulkano::swapchain::SwapchainAcquireFuture;
use vulkano::sync::future::NowFuture;

use ris_data::info::app_info::AppInfo;
use ris_asset::loader::scenes_loader::Scenes;
use ris_error::Extensions;
use ris_error::RisResult;

use crate::vulkan::util;

const REQUIRED_INSTANCE_LAYERS: &[&str] = &["VK_LAYER_KHRONOS_validation"];
const REQUIRED_DEVICE_EXTENSIONS: &[*const i8] = &[ash::extensions::khr::Swapchain::name().as_ptr()];
#[cfg(not(debug_assertions))]
const VALIDATION_ENABLED: bool = false;
#[cfg(debug_assertions)]
const VALIDATION_ENABLED: bool = true;

const PREFERRED_FORMAT: vk::Format = vk::Format::B8G8R8A8_SRGB;
const PREFERRED_COLOR_SPACE: vk::ColorSpaceKHR = vk::ColorSpaceKHR::SRGB_NONLINEAR;
const PREFERRED_PRESENT_MODE: vk::PresentModeKHR = vk::PresentModeKHR::IMMEDIATE;

unsafe extern "system" fn debug_callback(
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

struct SuitableDevice {
    // the lower the suitability, the better suited the device is to render. a dedicated gpu would
    // have a value of 0
    pub suitability: usize,
    pub graphics_queue_family: u32,
    pub present_queue_family: u32,
    pub physical_device: vk::PhysicalDevice,
    pub surface_capabilities: vk::SurfaceCapabilitiesKHR,
    pub surface_formats: Vec<vk::SurfaceFormatKHR>,
    pub surface_present_modes: Vec<vk::PresentModeKHR>,
}

pub struct Renderer {
    entry: ash::Entry,
    instance: ash::Instance,
    debug_utils: Option<(ash::extensions::ext::DebugUtils, vk::DebugUtilsMessengerEXT)>,
    surface_loader: ash::extensions::khr::Surface,
    surface: vk::SurfaceKHR,
    device: ash::Device,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
    swapchain_loader: ash::extensions::khr::Swapchain,
    swapchain: vk::SwapchainKHR,
    swapchain_format: vk::SurfaceFormatKHR,
    swapchain_extent: vk::Extent2D,
    swapchain_images: Vec<vk::Image>,
    swapchain_image_views: Vec<vk::ImageView>,
    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    graphics_pipeline: vk::Pipeline,
    framebuffers: Vec<vk::Framebuffer>,
    command_pool: vk::CommandPool,
    command_buffer: vk::CommandBuffer,

//    pub instance: Arc<Instance>,
//    pub device: Arc<Device>,
//    pub queue: Arc<Queue>,
//    pub swapchain: Arc<Swapchain>,
//    pub images: Vec<Arc<SwapchainImage>>,
//    pub render_pass: Arc<RenderPass>,
//    pub framebuffers: Vec<Arc<Framebuffer>>,
//    pub allocators: Allocators,
//    pub buffers: Buffers,
//    pub vertex_shader: Arc<ShaderModule>,
//    pub fragment_shader: Arc<ShaderModule>,
//    pub scenes: Scenes,
//    pub viewport: Viewport,
//    pub pipeline: Arc<GraphicsPipeline>,
//    pub command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>>,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        ris_log::debug!("dropping renderer...");

        unsafe {
            self.device.destroy_command_pool(self.command_pool, None);

            for &framebuffer in self.framebuffers.iter() {
                self.device.destroy_framebuffer(framebuffer, None);
            }

            self.device.destroy_pipeline(self.graphics_pipeline, None);
            self.device.destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_render_pass(self.render_pass, None);

            for &swapchain_image_view in self.swapchain_image_views.iter() {
                self.device.destroy_image_view(swapchain_image_view, None);
            }

            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);

            if let Some((debug_utils, debug_utils_messenger)) = self.debug_utils.take() {
                debug_utils.destroy_debug_utils_messenger(debug_utils_messenger, None);
            }

            self.instance.destroy_instance(None);
        }

        ris_log::info!("renderer dropped!");
    }
}

impl Renderer {
    pub fn initialize(app_info: &AppInfo, window: &Window, scenes: Scenes) -> RisResult<Self> {
        let entry = unsafe {ash::Entry::load()}?;
        
        // instance extensions
        let mut count = 0;
        if unsafe {
            sdl2_sys::SDL_Vulkan_GetInstanceExtensions(window.raw(), &mut count, ptr::null_mut())
        } == sdl2_sys::SDL_bool::SDL_FALSE {
            return ris_error::new_result!("{}", sdl2::get_error());
        }

        let mut instance_extensions = vec![ptr::null(); count as usize];

        if unsafe {
            sdl2_sys::SDL_Vulkan_GetInstanceExtensions(window.raw(), &mut count, instance_extensions.as_mut_ptr())
        } == sdl2_sys::SDL_bool::SDL_FALSE {
            return ris_error::new_result!("{}", sdl2::get_error());
        }

        // validation layers
        let available_layers = if !VALIDATION_ENABLED {
            ris_log::debug!("instance layers are disabled");
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
                    let name = unsafe {util::VkStr::from(&layer.layer_name)}?;
                    log_message.push_str(&format!("\n\t- {}", name));
                }
                ris_log::trace!("{}", log_message);

                let mut available_layers = Vec::new();
                let mut log_message = String::from("instance layers to be enabled:");

                for required_layer in REQUIRED_INSTANCE_LAYERS {
                    let mut layer_found = false;

                    for layer in layer_properties.iter() {
                        let name = unsafe {util::VkStr::from(&layer.layer_name)}?;
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

        let mut log_message = format!("Vulkan Instance Extensions: {}", instance_extensions.len());
        for extension in instance_extensions.iter() {
            let extension_name = unsafe{CStr::from_ptr(*extension)}.to_str()?;
            log_message.push_str(&format!("\n\t- {}", extension_name));
        }
        ris_log::trace!("{}", log_message);

        // instance
        let name = CString::new(app_info.package.name.clone())?;
        let vk_app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: name.as_ptr(),
            application_version: vk::make_api_version(0, 1, 0, 0),
            p_engine_name: name.as_ptr(),
            engine_version: vk::make_api_version(0, 1, 0, 0),
            api_version: vk::make_api_version(0, 1, 0, 92),
        };

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &vk_app_info,
            pp_enabled_layer_names: available_layers.1,
            enabled_layer_count: available_layers.0,
            pp_enabled_extension_names: instance_extensions.as_ptr(),
            enabled_extension_count: instance_extensions.len() as u32,
        };

        let instance = unsafe {
            entry.create_instance(&create_info, None)?
        };

        let debug_utils = if !VALIDATION_ENABLED {
            None
        } else {
            let debug_utils = ash::extensions::ext::DebugUtils::new(&entry, &instance);

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

            Some((debug_utils, debug_utils_messenger))
        };

        // surface
        let instance_handle = vk::Handle::as_raw(instance.handle());
        let surface_raw = window.vulkan_create_surface(instance_handle as usize).unroll()?;
        let surface: vk::SurfaceKHR = vk::Handle::from_raw(surface_raw);
        let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);

        // setup to find suitable physical devices
        let physical_devices = unsafe {
            instance.enumerate_physical_devices()?
        };

        let mut suitable_devices = Vec::new();

        let mut log_message = format!("Vulkan Required Device Extensions: {}", REQUIRED_DEVICE_EXTENSIONS.len());
        for extension in REQUIRED_DEVICE_EXTENSIONS {
            let extension_str = unsafe {CStr::from_ptr(*extension)}.to_str()?;
            log_message.push_str(&format!("\n\t- {}", extension_str));
        }
        ris_log::debug!("{}", log_message);

        // find suitable physical devices
        for (i, &physical_device) in physical_devices.iter().enumerate() {

            // gather physical device information
            let device_properties = unsafe {instance.get_physical_device_properties(physical_device)};
            let device_features = unsafe {instance.get_physical_device_features(physical_device)};
            let device_queue_families = unsafe {instance.get_physical_device_queue_family_properties(physical_device)};

            let (suitability, device_type_name) = match device_properties.device_type {
                vk::PhysicalDeviceType::DISCRETE_GPU => (0, "discrete gpu"),
                vk::PhysicalDeviceType::INTEGRATED_GPU => (1, "integrated gpu"),
                vk::PhysicalDeviceType::VIRTUAL_GPU => (2, "virtual gpu"),
                vk::PhysicalDeviceType::CPU => (3, "cpu"),
                vk::PhysicalDeviceType::OTHER => (4, "unkown"),
                _ => continue,
            };

            let mut log_message = format!("Vulkan Physical Device {}", i);

            let device_name = unsafe {util::VkStr::from(&device_properties.device_name)}?;
            log_message.push_str(&format!("\n\tname: {}", device_name));
            log_message.push_str(&format!("\n\tid: {}", device_properties.device_id));
            log_message.push_str(&format!("\n\ttype: {}", device_type_name));

            let api_version_variant = vk::api_version_variant(device_properties.api_version);
            let api_version_major = vk::api_version_major(device_properties.api_version);
            let api_version_minor = vk::api_version_minor(device_properties.api_version);
            let api_version_patch = vk::api_version_patch(device_properties.api_version);
            let api_version = format!(
                "{}.{}.{}.{}",
                api_version_variant,
                api_version_major,
                api_version_minor,
                api_version_patch,
            );
            log_message.push_str(&format!("\n\tapi version: {}", api_version));

            log_message.push_str(&format!("\n\tsupported queue families: {}", device_queue_families.len()));
            log_message.push_str("\n\t\tqueue | graphics, compute, transfer, sparse binding");
            for queue_family in device_queue_families.iter() {
                let supports_graphics = queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS);
                let supports_compute = queue_family.queue_flags.contains(vk::QueueFlags::COMPUTE);
                let supports_transfer = queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER);
                let supports_sparse_binding = queue_family.queue_flags.contains(vk::QueueFlags::SPARSE_BINDING);

                log_message.push_str(&format!(
                    "\n\t\t{:5} | {:8}, {:7}, {:8}, {:14}",
                    queue_family.queue_count,
                    supports_graphics,
                    supports_compute,
                    supports_transfer,
                    supports_sparse_binding,
                ));
            }

            log_message.push_str(&format!("\n\tgeometry shader support: {}", device_features.geometry_shader == vk::TRUE));

            // check device extension support
            let available_extensions = unsafe {
                instance.enumerate_device_extension_properties(physical_device)?
            };

            let mut supports_required_extensions = true;

            for required_extension in REQUIRED_DEVICE_EXTENSIONS {
                let mut extension_found = false;

                for extension in available_extensions.iter() {
                    let name = unsafe {util::VkStr::from(&extension.extension_name)}?;
                    let left = unsafe{CStr::from_ptr(*required_extension)}.to_str()?;
                    let right = name.as_str();
                    if left == right {
                        extension_found = true;
                        break;
                    }
                }

                if !extension_found {
                    supports_required_extensions = false;
                    break;
                }
            }

            log_message.push_str(&format!("\n\trequired extension support: {}", supports_required_extensions));
            //log_message.push_str(&format!("\n\tavailable extensions: {}", available_extensions.len()));
            //for extension in available_extensions {
            //    let name = unsafe{util::VkStr::from(&extension.extension_name)}?;
            //    log_message.push_str(&format!("\n\t\t- {}", name));
            //}

            // check swapchain support
            let surface_capabilities = unsafe {
                surface_loader.get_physical_device_surface_capabilities(physical_device, surface)
            }?;
            let surface_formats = unsafe {
                surface_loader.get_physical_device_surface_formats(physical_device, surface)
            }?;
            let surface_present_modes = unsafe {
                surface_loader.get_physical_device_surface_present_modes(physical_device, surface)
            }?;

            log_message.push_str(&format!("\n\tsurface formats: {}", surface_formats.len()));
            for format in surface_formats.iter() {
                log_message.push_str(&format!("\n\t\t- {:?}, {:?}", format.format, format.color_space));
            }
            log_message.push_str(&format!("\n\tsurface present modes: {}", surface_present_modes.len()));
            for present_mode in surface_present_modes.iter() {
                log_message.push_str(&format!("\n\t\t- {:?}", present_mode));
            }

            ris_log::info!("{}", log_message);

            if !supports_required_extensions || surface_formats.is_empty() || surface_present_modes.is_empty() {
                continue; // device not supported. skip
            }

            // find queue family
            // a single queue that supports both graphics and presenting is more performant than
            // two seperate queues. to prevent the edgecase, that two seperate queues are found
            // before a single one, we search for a single one first, and then fall back to search
            // seperately.
            let mut graphics_queue_index = None;
            let mut present_queue_index = None;

            for (i, queue_family) in device_queue_families.iter().enumerate() {
                if queue_family.queue_count > 0 &&
                    queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) &&
                    unsafe {surface_loader.get_physical_device_surface_support(physical_device, i as u32, surface)}?
                {
                    graphics_queue_index = Some(i);
                    present_queue_index = Some(i);
                    break;
                }
            }

            if graphics_queue_index.is_none() || present_queue_index.is_none() {
                for (i, queue_family) in device_queue_families.iter().enumerate() {
                    if queue_family.queue_count == 0 {
                        continue;
                    }

                    if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                        graphics_queue_index = Some(i);
                    }

                    if unsafe {surface_loader.get_physical_device_surface_support(physical_device, i as u32, surface)}? {
                        present_queue_index = Some(i);
                    }

                    if graphics_queue_index.is_some() && present_queue_index.is_some() {
                        break;
                    }
                }
            }

            if let (Some(graphics), Some(present)) = (graphics_queue_index, present_queue_index) {
                // device is supported and suitable. collect info and add to vec
                let suitable_device = SuitableDevice{
                    suitability,
                    graphics_queue_family: graphics as u32,
                    present_queue_family: present as u32,
                    physical_device,
                    surface_capabilities,
                    surface_formats,
                    surface_present_modes,
                };
                suitable_devices.push(suitable_device);
            };

        } // end find suitable physical devices

        // logical device
        let Some(suitable_device) = suitable_devices
            .into_iter()
            .min_by_key(|x| x.suitability) else {
            return ris_error::new_result!("no suitable hardware found to initialize vulkan renderer");
        };

        let mut unique_queue_families = std::collections::HashSet::new();
        unique_queue_families.insert(suitable_device.graphics_queue_family);
        unique_queue_families.insert(suitable_device.present_queue_family);

        let queue_priorities = [1.0_f32];
        let mut queue_create_infos = Vec::new();
        for queue_family in unique_queue_families {
            let queue_create_info = vk::DeviceQueueCreateInfo {
                s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::DeviceQueueCreateFlags::empty(),
                queue_family_index: queue_family,
                p_queue_priorities: queue_priorities.as_ptr(),
                queue_count: queue_priorities.len() as u32,
            };
            queue_create_infos.push(queue_create_info);
        }

        let physical_device_features = vk::PhysicalDeviceFeatures {
            ..Default::default()
        };

        let device_create_info = vk::DeviceCreateInfo {
            s_type: vk::StructureType::DEVICE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DeviceCreateFlags::empty(),
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            pp_enabled_layer_names: available_layers.1,
            enabled_layer_count: available_layers.0,
            pp_enabled_extension_names: REQUIRED_DEVICE_EXTENSIONS.as_ptr(),
            enabled_extension_count: REQUIRED_DEVICE_EXTENSIONS.len() as u32,
            p_enabled_features: &physical_device_features,
        };

        let device = unsafe {instance.create_device(suitable_device.physical_device, &device_create_info, None)}?;
        let graphics_queue = unsafe{device.get_device_queue(suitable_device.graphics_queue_family, 0)};
        let present_queue = unsafe{device.get_device_queue(suitable_device.present_queue_family, 0)};

        // swap chain
        let preferred_surface_format = suitable_device.surface_formats
            .iter()
            .find(|x| x.format == PREFERRED_FORMAT && x.color_space == PREFERRED_COLOR_SPACE);
        let surface_format = match preferred_surface_format {
            Some(format) => format,
            // getting the first format if the preferred format does not exist. this should not
            // cause ub, becuase we checked if the list is empty at finding the suitable device.
            None => &suitable_device.surface_formats[0],
        };

        let preferred_surface_present_mode = suitable_device.surface_present_modes
            .iter()
            .find(|&&x| x == PREFERRED_PRESENT_MODE);
        let surface_present_mode = match preferred_surface_present_mode {
            Some(present_mode) => present_mode,
            // getting the first present mode if the preferred format does not exist. this should
            // not cause ub, becuase we checked if the list is empty at finding the suitable device.
            None => unsafe{suitable_device.surface_present_modes.get_unchecked(0)},
        };

        let surface_capabilities = suitable_device.surface_capabilities;
        let swapchain_extent = if surface_capabilities.current_extent.width != u32::MAX {
            surface_capabilities.current_extent
        } else {
            let (window_width, window_height) = window.vulkan_drawable_size();
            let width = window_width.clamp(
                surface_capabilities.min_image_extent.width,
                surface_capabilities.max_image_extent.width,
            );
            let height = window_height.clamp(
                surface_capabilities.min_image_extent.height,
                surface_capabilities.max_image_extent.height,
            );

            vk::Extent2D {
                width,
                height,
            }
        };

        let preferred_swapchain_image_count = surface_capabilities.min_image_count + 1;
        let swapchain_image_count = if surface_capabilities.max_image_count == 0 {
            // SurfaceCapabilitiesKHR == 0 indicates there is no maximum
            preferred_swapchain_image_count
        } else {
            u32::min(
                preferred_swapchain_image_count,
                surface_capabilities.max_image_count,
            )
        };

        let (image_sharing_mode, queue_family_index_count, queue_family_indices) = 
            if suitable_device.graphics_queue_family != suitable_device.present_queue_family {(
                vk::SharingMode::CONCURRENT,
                2,
                vec![
                suitable_device.graphics_queue_family,
                suitable_device.present_queue_family,
                ],
            )} else {(
                vk::SharingMode::EXCLUSIVE,
                0,
                vec![],
            )};

        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: vk::SwapchainCreateFlagsKHR::empty(),
            surface,
            min_image_count: swapchain_image_count,
            image_color_space: surface_format.color_space,
            image_format: surface_format.format,
            image_extent: swapchain_extent,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode,
            p_queue_family_indices: queue_family_indices.as_ptr(),
            queue_family_index_count,
            pre_transform: surface_capabilities.current_transform,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            present_mode: *surface_present_mode,
            clipped: vk::TRUE,
            old_swapchain: vk::SwapchainKHR::null(),
            image_array_layers: 1,
        };

        let swapchain_loader = ash::extensions::khr::Swapchain::new(&instance, &device);
        let swapchain = unsafe {
            swapchain_loader.create_swapchain(&swapchain_create_info, None)
        }?;

        let swapchain_images = unsafe {
            swapchain_loader.get_swapchain_images(swapchain)
        }?;

        // image views
        let mut swapchain_image_views = Vec::new();
        for swapchain_image in swapchain_images.iter() {
            let image_view_create_info = vk::ImageViewCreateInfo {
                s_type: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::ImageViewCreateFlags::empty(),
                image: *swapchain_image,
                view_type: vk::ImageViewType::TYPE_2D,
                format: surface_format.format,
                components: vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                },
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
            };

            let image_view = unsafe {
                device.create_image_view(&image_view_create_info, None)
            }?;

            swapchain_image_views.push(image_view);
        }

        // graphics pipeline
        // render pass
        let color_attachment_descriptions = [vk::AttachmentDescription {
            flags: vk::AttachmentDescriptionFlags::empty(),
            format: surface_format.format,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
        }];

        let color_attachment_references = [vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }];

        let subpass_descriptions = [vk::SubpassDescription {
            flags: vk::SubpassDescriptionFlags::empty(),
            pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
            input_attachment_count: 0,
            p_input_attachments: ptr::null(),
            color_attachment_count: color_attachment_references.len() as u32,
            p_color_attachments: color_attachment_references.as_ptr(),
            p_resolve_attachments: ptr::null(),
            p_depth_stencil_attachment: ptr::null(),
            preserve_attachment_count: 0,
            p_preserve_attachments: ptr::null(),
        }];

        let render_pass_create_info = vk::RenderPassCreateInfo {
            s_type: vk::StructureType::RENDER_PASS_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::RenderPassCreateFlags::empty(),
            attachment_count: color_attachment_descriptions.len() as u32,
            p_attachments: color_attachment_descriptions.as_ptr(),
            subpass_count: subpass_descriptions.len() as u32,
            p_subpasses: subpass_descriptions.as_ptr(),
            dependency_count: 0,
            p_dependencies: ptr::null(),
        };

        let render_pass = unsafe{device.create_render_pass(&render_pass_create_info, None)}?;

        // graphics pipeline
        // shaders
        let mut vs_file = std::fs::File::open("C:/Users/Rismosch/Desktop/shaders/vert.spv")?;
        let mut fs_file = std::fs::File::open("C:/Users/Rismosch/Desktop/shaders/frag.spv")?;

        let vs_file_size = ris_file::seek!(vs_file, SeekFrom::End(0))?;
        let fs_file_size = ris_file::seek!(fs_file, SeekFrom::End(0))?;

        let mut vs_bytes = vec![0; vs_file_size as usize];
        let mut fs_bytes = vec![0; fs_file_size as usize];

        ris_file::seek!(vs_file, SeekFrom::Start(0))?;
        ris_file::read!(vs_file, vs_bytes)?;

        ris_file::seek!(fs_file, SeekFrom::Start(0))?;
        ris_file::read!(fs_file, fs_bytes)?;

        drop(vs_file);
        drop(fs_file);

        // spirv data is read in u8, but vulkan expects it to be u32.
        // assert that the data is properly aligned
        ris_error::assert!(vs_file_size % 4 == 0)?;
        ris_error::assert!(fs_file_size % 4 == 0)?;

        let vs_shader_module_create_info = vk::ShaderModuleCreateInfo {
            s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::ShaderModuleCreateFlags::empty(),
            code_size: vs_bytes.len(),
            p_code: vs_bytes.as_ptr() as *const u32,
        };
        let fs_shader_module_create_info = vk::ShaderModuleCreateInfo {
            s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::ShaderModuleCreateFlags::empty(),
            code_size: fs_bytes.len(),
            p_code: fs_bytes.as_ptr() as *const u32,
        };

        let vs_shader_module = unsafe{device.create_shader_module(&vs_shader_module_create_info, None)}?;
        let fs_shader_module = unsafe{device.create_shader_module(&fs_shader_module_create_info, None)}?;

        let main_function_name = CString::new("main").unwrap();

        let shader_stages = [
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::PipelineShaderStageCreateFlags::empty(),
                module: vs_shader_module,
                p_name: main_function_name.as_ptr(),
                p_specialization_info: ptr::null(),
                stage: vk::ShaderStageFlags::VERTEX,
            },
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::PipelineShaderStageCreateFlags::empty(),
                module: fs_shader_module,
                p_name: main_function_name.as_ptr(),
                p_specialization_info: ptr::null(),
                stage: vk::ShaderStageFlags::FRAGMENT,
            },
        ];

        // graphics pipeline
        // pipeline
        let pipeline_vertex_input_state_create_info = [vk::PipelineVertexInputStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineVertexInputStateCreateFlags::empty(),
            vertex_binding_description_count: 0,
            p_vertex_binding_descriptions: ptr::null(),
            vertex_attribute_description_count: 0,
            p_vertex_attribute_descriptions: ptr::null(),
        }];

        let pipeline_input_assembly_state_info = [vk::PipelineInputAssemblyStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineInputAssemblyStateCreateFlags::empty(),
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            primitive_restart_enable: vk::FALSE,
        }];

        let viewports = [vk::Viewport {
            x: 0.,
            y: 0.,
            width: swapchain_extent.width as f32,
            height: swapchain_extent.height as f32,
            min_depth: 0.,
            max_depth: 1.,
        }];

        let scissors = [vk::Rect2D {
            offset: vk::Offset2D{x: 0, y: 0},
            extent: swapchain_extent,
        }];

        let pipeline_viewport_state_create_info = [vk::PipelineViewportStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineViewportStateCreateFlags::empty(),
            viewport_count: 1,
            p_viewports: viewports.as_ptr(),
            scissor_count: 1,
            p_scissors: scissors.as_ptr(),
        }];

        let pipeline_rasterization_state_create_info = [vk::PipelineRasterizationStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineRasterizationStateCreateFlags::empty(),
            depth_clamp_enable: vk::FALSE,
            rasterizer_discard_enable: vk::FALSE,
            polygon_mode: vk::PolygonMode::FILL,
            cull_mode: vk::CullModeFlags::BACK,
            front_face: vk::FrontFace::CLOCKWISE,
            depth_bias_enable: vk::FALSE,
            depth_bias_constant_factor: 0.,
            depth_bias_clamp: 0.,
            depth_bias_slope_factor: 0.,
            line_width: 1.,
        }];

        let pipeline_multisample_state_create_info = [vk::PipelineMultisampleStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineMultisampleStateCreateFlags::empty(),
            rasterization_samples: vk::SampleCountFlags::TYPE_1,
            sample_shading_enable: vk::FALSE,
            min_sample_shading: 1.,
            p_sample_mask: ptr::null(),
            alpha_to_coverage_enable: vk::FALSE,
            alpha_to_one_enable: vk::FALSE,
        }];

        let stencil_op_state = vk::StencilOpState {
            fail_op: vk::StencilOp::KEEP,
            pass_op: vk::StencilOp::KEEP,
            depth_fail_op: vk::StencilOp::KEEP,
            compare_op: vk::CompareOp::ALWAYS,
            compare_mask: 0,
            write_mask: 0,
            reference: 0,
        };

        let pipeline_depth_stencil_state_create_info = vk::PipelineDepthStencilStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineDepthStencilStateCreateFlags::empty(),
            depth_test_enable: vk::FALSE,
            depth_write_enable: vk::FALSE,
            depth_compare_op: vk::CompareOp::LESS_OR_EQUAL,
            depth_bounds_test_enable: vk::FALSE,
            stencil_test_enable: vk::FALSE,
            front: stencil_op_state,
            back: stencil_op_state,
            min_depth_bounds: 0.,
            max_depth_bounds: 1.,
        };

        // pseudocode of how blending with vk::PipelineColorBlendAttachmentState works:
        //
        //     if (blend_enable) {
        //         final_color.rgb = (src_color_blend_factor * new_color.rgb) <color_blend_op> (dst_color_blend_factor * old_color.rgb);
        //         final_color.a = (src_alpha_blend_factor * new_color.a) <alpha_blend_op> (dst_alpha_blend_factor * old_color.a);
        //     } else {
        //         final_color = new_color;
        //     }
        //     
        //     final_color = final_color & color_write_mask;

        let pipeline_color_blend_attachment_states = [vk::PipelineColorBlendAttachmentState{
            blend_enable: vk::TRUE,
            src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ONE,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
            color_write_mask: vk::ColorComponentFlags::RGBA,
        }];

        let pipeline_color_blend_state_create_info = [vk::PipelineColorBlendStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineColorBlendStateCreateFlags::empty(),
            logic_op_enable: vk::FALSE,
            logic_op: vk::LogicOp::COPY,
            attachment_count: pipeline_color_blend_attachment_states.len() as u32,
            p_attachments: pipeline_color_blend_attachment_states.as_ptr(),
            blend_constants: [0., 0., 0., 0.,],
        }];

        let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo {
            s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: 0,
            p_set_layouts: ptr::null(),
            push_constant_range_count: 0,
            p_push_constant_ranges: ptr::null(),
        };

        let pipeline_layout = unsafe{device.create_pipeline_layout(&pipeline_layout_create_info, None)}?;
        
        // graphic pipeline
        // creation
        let graphics_pipeline_create_info = [vk::GraphicsPipelineCreateInfo {
            s_type: vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineCreateFlags::empty(),
            stage_count: shader_stages.len() as u32,
            p_stages: shader_stages.as_ptr(),
            p_vertex_input_state: pipeline_vertex_input_state_create_info.as_ptr(),
            p_input_assembly_state: pipeline_input_assembly_state_info.as_ptr(),
            p_tessellation_state: ptr::null(),
            p_viewport_state: pipeline_viewport_state_create_info.as_ptr(),
            p_rasterization_state: pipeline_rasterization_state_create_info.as_ptr(),
            p_multisample_state: pipeline_multisample_state_create_info.as_ptr(),
            p_depth_stencil_state: ptr::null(),
            p_color_blend_state: pipeline_color_blend_state_create_info.as_ptr(),
            p_dynamic_state: ptr::null(),
            layout: pipeline_layout,
            render_pass,
            subpass: 0,
            base_pipeline_handle: vk::Pipeline::null(),
            base_pipeline_index: -1,
        }];

        let graphics_pipelines = unsafe{device.create_graphics_pipelines(
            vk::PipelineCache::null(),
            &graphics_pipeline_create_info,
            None,
        )}.map_err(|e| e.1)?;
        let graphics_pipeline = graphics_pipelines.into_iter().next().unroll()?;

        unsafe {device.destroy_shader_module(vs_shader_module, None)};
        unsafe {device.destroy_shader_module(fs_shader_module, None)};

        // frame buffers
        let mut framebuffers = Vec::with_capacity(swapchain_image_views.len());
        for &swapchain_image_view in swapchain_image_views.iter() {
            let image_view = [swapchain_image_view];

            let framebuffer_create_info = vk::FramebufferCreateInfo {
                s_type: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::FramebufferCreateFlags::empty(),
                render_pass, 
                attachment_count: image_view.len() as u32,
                p_attachments: image_view.as_ptr(),
                width: swapchain_extent.width,
                height: swapchain_extent.height,
                layers: 1,
            };

            let framebuffer = unsafe{device.create_framebuffer(&framebuffer_create_info, None)}?;
            framebuffers.push(framebuffer);
        }

        // command buffer
        let command_pool_create_info = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
            queue_family_index: suitable_device.graphics_queue_family,
        };

        let command_pool = unsafe{device.create_command_pool(&command_pool_create_info, None)}?;

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_pool,
            level: vk::CommandBufferLevel::PRIMARY,
            command_buffer_count: 1,
        };

        let command_buffers = unsafe {device.allocate_command_buffers(&command_buffer_allocate_info)}?;
        let command_buffer = command_buffers.into_iter().next().unroll()?;

        // record command buffer

        Ok(Self {
            entry,
            instance,
            debug_utils,
            surface_loader,
            surface,
            device,
            graphics_queue,
            present_queue,
            swapchain_loader,
            swapchain,
            swapchain_format: *surface_format,
            swapchain_extent,
            swapchain_images,
            swapchain_image_views,
            render_pass,
            pipeline_layout,
            graphics_pipeline,
            framebuffers,
            command_pool,
            command_buffer,
        })

        //// instance
        //let library = VulkanLibrary::new()?;
        //let instance_extensions = InstanceExtensions::from_iter(
        //    window
        //        .vulkan_instance_extensions()
        //        .map_err(|e| ris_error::new!("failed to get vulkan instance extensions: {}", e))?,
        //);

        //let instance = Instance::new(
        //    library,
        //    InstanceCreateInfo {
        //        enabled_extensions: instance_extensions,
        //        ..Default::default()
        //    },
        //)?;

        //// surface
        //let surface_handle = window
        //    .vulkan_create_surface(instance.handle().as_raw() as _)
        //    .map_err(|e| ris_error::new!("failed to create instance: {}", e))?;
        //let surface = unsafe {
        //    Surface::from_handle(
        //        instance.clone(),
        //        <_ as Handle>::from_raw(surface_handle),
        //        SurfaceApi::Win32,
        //        None,
        //    )
        //};
        //let surface = Arc::new(surface);

        //// physical device
        //let device_extensions = DeviceExtensions {
        //    khr_swapchain: true,
        //    ..DeviceExtensions::empty()
        //};
        //let (physical_device, queue_family_index) = super::physical_device::select_physical_device(
        //    instance.clone(),
        //    surface.clone(),
        //    &device_extensions,
        //)?;

        //// device
        //let (device, mut queues) = Device::new(
        //    physical_device.clone(),
        //    DeviceCreateInfo {
        //        queue_create_infos: vec![QueueCreateInfo {
        //            queue_family_index,
        //            ..Default::default()
        //        }],
        //        enabled_extensions: device_extensions,
        //        ..Default::default()
        //    },
        //)?;
        //let queue = queues.next().unroll()?;

        //// shaders
        //let vs_future = super::shader::load_async(device.clone(), scenes.default_vs.clone());
        //let fs_future = super::shader::load_async(device.clone(), scenes.default_fs.clone());

        //// swapchain
        //let dimensions = window.vulkan_drawable_size();
        //let (swapchain, images) = super::swapchain::create_swapchain(
        //    physical_device.clone(),
        //    dimensions,
        //    device.clone(),
        //    surface.clone(),
        //)?;

        //// render pass
        //let render_pass =
        //    super::render_pass::create_render_pass(device.clone(), swapchain.clone())?;

        //// viewport
        //let viewport = Viewport {
        //    origin: [0.0, 0.0],
        //    dimensions: [dimensions.0 as f32, dimensions.1 as f32],
        //    depth_range: 0.0..1.0,
        //};

        //// allocators
        //let allocators = super::allocators::Allocators::new(device.clone());

        //// frame buffers
        //let framebuffers = super::swapchain::create_framebuffers(
        //    &allocators,
        //    dimensions,
        //    &images,
        //    render_pass.clone(),
        //)?;

        //// pipeline
        //let vs = vs_future.wait()?;
        //let fs = fs_future.wait()?;

        //let pipeline = super::pipeline::create_pipeline(
        //    device.clone(),
        //    vs.clone(),
        //    fs.clone(),
        //    render_pass.clone(),
        //    &viewport,
        //)?;

        //// buffers
        //let buffers = super::buffers::Buffers::new(&allocators, images.len(), pipeline.clone())?;

        //// command buffers
        //let command_buffers = super::command_buffers::create_command_buffers(
        //    &allocators,
        //    queue.clone(),
        //    pipeline.clone(),
        //    &framebuffers,
        //    &buffers,
        //)?;

        //// return
        //Ok(Self {
        //    instance,
        //    device,
        //    queue,
        //    swapchain,
        //    images,
        //    render_pass,
        //    framebuffers,
        //    allocators,
        //    buffers,
        //    vertex_shader: vs,
        //    fragment_shader: fs,
        //    scenes,
        //    viewport,
        //    pipeline,
        //    command_buffers,
        //})
    }

    pub fn record_command_buffer(&self, image_index: usize) -> RisResult<()> {
        let command_buffer_begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: ptr::null(),
            flags: vk::CommandBufferUsageFlags::empty(),
            p_inheritance_info: ptr::null(),
        };

        unsafe{self.device.begin_command_buffer(self.command_buffer, &command_buffer_begin_info)}?;

        let clear_values = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0., 0., 0., 1.0],
            },
        }];

        let render_pass_begin_info = vk::RenderPassBeginInfo {
            s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
            p_next: ptr::null(),
            render_pass: self.render_pass,
            framebuffer: self.framebuffers[image_index],
            render_area: vk::Rect2D{
                offset: vk::Offset2D {x: 0, y: 0},
                extent: self.swapchain_extent,
            },
            clear_value_count: clear_values.len() as u32,
            p_clear_values: clear_values.as_ptr(),
        };

        unsafe{self.device.cmd_begin_render_pass(self.command_buffer, &render_pass_begin_info, vk::SubpassContents::INLINE)};
        unsafe{self.device.cmd_bind_pipeline(self.command_buffer, vk::PipelineBindPoint::GRAPHICS, self.graphics_pipeline)};
        // dynamic viewport/scissor here
        unsafe{self.device.cmd_draw(self.command_buffer, 3, 1, 0, 0)};
        unsafe{self.device.cmd_end_render_pass(self.command_buffer)};
        unsafe{self.device.end_command_buffer(self.command_buffer)}?;

        Ok(())
    }

    pub fn recreate_swapchain(&mut self, dimensions: (u32, u32)) -> RisResult<()> {
        todo!();
        //ris_log::trace!("recreating swapchain...");

        //let (new_swapchain, new_images) = match self.swapchain.recreate(SwapchainCreateInfo {
        //    image_extent: [dimensions.0, dimensions.1],
        //    ..self.swapchain.create_info()
        //}) {
        //    Ok(r) => r,
        //    Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return Ok(()),
        //    Err(e) => return ris_error::new_result!("failed to recreate swapchain: {}", e),
        //};

        //self.images = new_images;

        //self.swapchain = new_swapchain;
        //self.framebuffers = super::swapchain::create_framebuffers(
        //    &self.allocators,
        //    dimensions,
        //    &self.images,
        //    self.render_pass.clone(),
        //)?;

        //ris_log::trace!("swapcain recreated!");
        //Ok(())
    }

    pub fn recreate_viewport(&mut self, dimensions: (u32, u32)) -> RisResult<()> {
        todo!();
        //ris_log::trace!("recreating viewport...");

        //self.recreate_swapchain(dimensions)?;
        //self.viewport.dimensions = [dimensions.0 as f32, dimensions.1 as f32];

        //self.pipeline = super::pipeline::create_pipeline(
        //    self.device.clone(),
        //    self.vertex_shader.clone(),
        //    self.fragment_shader.clone(),
        //    self.render_pass.clone(),
        //    &self.viewport,
        //)?;

        //self.command_buffers = super::command_buffers::create_command_buffers(
        //    &self.allocators,
        //    self.queue.clone(),
        //    self.pipeline.clone(),
        //    &self.framebuffers,
        //    &self.buffers,
        //)?;

        //ris_log::trace!("viewport recreated!");
        //Ok(())
    }

    pub fn reload_shaders(&mut self) -> RisResult<()> {
        todo!();
        //ris_log::trace!("reloading shaders...");

        // let vs_future =
        //     super::shader::load_async(self.device.clone(), self.scenes.default_vs.clone());
        // let fs_future =
        //     super::shader::load_async(self.device.clone(), self.scenes.default_fs.clone());

        // let vs = vs_future.wait(None)??;
        // let fs = fs_future.wait(None)??;

        // self.vertex_shader = vs;
        // self.fragment_shader = fs;

        // ris_log::trace!("shaders reloaded!");
        // Ok(())
    }

    pub fn get_image_count(&self) -> usize {
        todo!();
        //self.images.len()
    }

    pub fn acquire_swapchain_image(
        &self,
    ) -> Result<(u32, bool, SwapchainAcquireFuture), AcquireError> {
        todo!();
        //swapchain::acquire_next_image(self.swapchain.clone(), None)
    }

    pub fn synchronize(&self) -> NowFuture {
        todo!();
        //let mut now = sync::now(self.device.clone());
        //now.cleanup_finished();
        //now
    }

    pub fn update_uniform(
        &self,
        index: usize,
        ubo: &super::gpu_objects::UniformBufferObject,
    ) -> RisResult<()> {
        todo!();
        //let mut uniform_content = self.buffers.uniforms[index].0.write()?;

        //uniform_content.view = ubo.view;
        //uniform_content.proj = ubo.proj;
        //uniform_content.proj_view = ubo.proj_view;

        //Ok(())
    }
}
