use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;
use std::sync::Arc;

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
