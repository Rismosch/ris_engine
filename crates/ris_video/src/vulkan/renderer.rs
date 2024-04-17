use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

use ash::vk;
use sdl2::video::Window;

use ris_asset::AssetId;
use ris_asset::loader::scenes_loader::Scenes;
use ris_data::info::app_info::AppInfo;
use ris_error::Extensions;
use ris_error::RisResult;
use ris_math::color::Rgb;
use ris_math::matrix::Mat4;
use ris_math::vector::Vec3;

use crate::vulkan::util;

#[repr(C)]
pub struct Vertex {
    pos: Vec3,
    color: Rgb,
}
impl Vertex {
    pub fn get_binding_descriptions() -> [vk::VertexInputBindingDescription; 1] {
        [vk::VertexInputBindingDescription{
            binding: 0,
            stride: std::mem::size_of::<Self>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }]
    }

    pub fn get_attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        [
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: std::mem::offset_of!(Self, pos) as u32,
            },
            vk::VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: std::mem::offset_of!(Self, color) as u32,
            },
        ]
    }
}

pub const VERTICES: [Vertex; 4 * 6] = [
    // pos x
    Vertex{
        pos: Vec3(0.5, 0.5, 0.5),
        color: Rgb{r:1.0, g:0.0, b:0.0},
    },
    Vertex{
        pos: Vec3(0.5, 0.5, -0.5),
        color: Rgb{r:1.0, g:0.0, b:0.0},
    },
    Vertex{
        pos: Vec3(0.5, -0.5, -0.5),
        color: Rgb{r:1.0, g:0.0, b:0.0},
    },
    Vertex{
        pos: Vec3(0.5, -0.5, 0.5),
        color: Rgb{r:1.0, g:0.0, b:0.0},
    },
    // pos y
    Vertex{
        pos: Vec3(0.5, 0.5, 0.5),
        color: Rgb{r:0.0, g:1.0, b:0.0},
    },
    Vertex{
        pos: Vec3(-0.5, 0.5, 0.5),
        color: Rgb{r:0.0, g:1.0, b:0.0},
    },
    Vertex{
        pos: Vec3(-0.5, 0.5, -0.5),
        color: Rgb{r:0.0, g:1.0, b:0.0},
    },
    Vertex{
        pos: Vec3(0.5, 0.5, -0.5),
        color: Rgb{r:0.0, g:1.0, b:0.0},
    },
    // pos z
    Vertex{
        pos: Vec3(0.5, 0.5, 0.5),
        color: Rgb{r:0.0, g:0.0, b:1.0},
    },
    Vertex{
        pos: Vec3(0.5, -0.5, 0.5),
        color: Rgb{r:0.0, g:0.0, b:1.0},
    },
    Vertex{
        pos: Vec3(-0.5, -0.5, 0.5),
        color: Rgb{r:0.0, g:0.0, b:1.0},
    },
    Vertex{
        pos: Vec3(-0.5, 0.5, 0.5),
        color: Rgb{r:0.0, g:0.0, b:1.0},
    },
    // neg x
    Vertex{
        pos: Vec3(-0.5, -0.5, -0.5),
        color: Rgb{r:0.0, g:1.0, b:1.0},
    },
    Vertex{
        pos: Vec3(-0.5, 0.5, -0.5),
        color: Rgb{r:0.0, g:1.0, b:1.0},
    },
    Vertex{
        pos: Vec3(-0.5, 0.5, 0.5),
        color: Rgb{r:0.0, g:1.0, b:1.0},
    },
    Vertex{
        pos: Vec3(-0.5, -0.5, 0.5),
        color: Rgb{r:0.0, g:1.0, b:1.0},
    },
    // neg y
    Vertex{
        pos: Vec3(-0.5, -0.5, 0.5),
        color: Rgb{r:1.0, g:0.0, b:1.0},
    },
    Vertex{
        pos: Vec3(0.5, -0.5, 0.5),
        color: Rgb{r:1.0, g:0.0, b:1.0},
    },
    Vertex{
        pos: Vec3(0.5, -0.5, -0.5),
        color: Rgb{r:1.0, g:0.0, b:1.0},
    },
    Vertex{
        pos: Vec3(-0.5, -0.5, -0.5),
        color: Rgb{r:1.0, g:0.0, b:1.0},
    },
    // neg z
    Vertex{
        pos: Vec3(-0.5, -0.5, -0.5),
        color: Rgb{r:1.0, g:1.0, b:0.0},
    },
    Vertex{
        pos: Vec3(0.5, -0.5, -0.5),
        color: Rgb{r:1.0, g:1.0, b:0.0},
    },
    Vertex{
        pos: Vec3(0.5, 0.5, -0.5),
        color: Rgb{r:1.0, g:1.0, b:0.0},
    },
    Vertex{
        pos: Vec3(-0.5, 0.5, -0.5),
        color: Rgb{r:1.0, g:1.0, b:0.0},
    },
];

pub const INDICES: [u32; 6 * 6] = [
    0, 1, 2, 2, 3, 0,
    4, 5, 6, 6, 7, 4,
    8, 9, 10, 10, 11, 8,
    12, 13, 14, 14, 15, 12,
    16, 17, 18, 18, 19, 16,
    20, 21, 22, 22, 23, 20,
];


#[repr(C)]
pub struct UniformBufferObject {
    pub model: Mat4,
    pub view: Mat4,
    pub proj: Mat4,
}

const REQUIRED_INSTANCE_LAYERS: &[&str] = &["VK_LAYER_KHRONOS_validation"];
const REQUIRED_DEVICE_EXTENSIONS: &[*const i8] = &[ash::extensions::khr::Swapchain::name().as_ptr()];
#[cfg(not(debug_assertions))]
const VALIDATION_ENABLED: bool = false;
#[cfg(debug_assertions)]
const VALIDATION_ENABLED: bool = true;

const PREFERRED_FORMAT: vk::Format = vk::Format::B8G8R8A8_SRGB;
const PREFERRED_COLOR_SPACE: vk::ColorSpaceKHR = vk::ColorSpaceKHR::SRGB_NONLINEAR;
const PREFERRED_PRESENT_MODE: vk::PresentModeKHR = vk::PresentModeKHR::IMMEDIATE;

const MAX_FRAMES_IN_FLIGHT: u32 = 2;

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

pub struct SurfaceDetails {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

pub struct SuitableDevice {
    // the lower the suitability, the better suited the device is to render. a dedicated gpu would
    // have a value of 0
    pub suitability: usize,
    pub graphics_queue_family: u32,
    pub present_queue_family: u32,
    pub physical_device: vk::PhysicalDevice,
}

pub struct FrameInFlight {
    pub command_buffer: vk::CommandBuffer,
    pub uniform_buffer: vk::Buffer,
    pub uniform_buffer_memory: vk::DeviceMemory,
    pub uniform_buffer_mapped: *mut UniformBufferObject,
    pub descriptor_set: vk::DescriptorSet,
    pub image_available_semaphore: vk::Semaphore,
    pub render_finished_semaphore: vk::Semaphore,
    pub in_flight_fence: vk::Fence,
}

pub struct SwapchainObjects {
    pub swapchain_loader: ash::extensions::khr::Swapchain,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_format: vk::SurfaceFormatKHR,
    pub swapchain_extent: vk::Extent2D,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_image_views: Vec<vk::ImageView>,
    pub render_pass: vk::RenderPass,
    pub pipeline_layout: vk::PipelineLayout,
    pub graphics_pipeline: vk::Pipeline,
    pub framebuffers: Vec<vk::Framebuffer>,
}

pub struct TransientCommand<'a> {
    device: &'a ash::Device,
    queue: &'a vk::Queue,
    command_pool: &'a vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,
}

pub struct Renderer {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub debug_utils: Option<(ash::extensions::ext::DebugUtils, vk::DebugUtilsMessengerEXT)>,
    pub surface_loader: ash::extensions::khr::Surface,
    pub surface: vk::SurfaceKHR,
    pub suitable_device: SuitableDevice,
    pub device: ash::Device,
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
    pub descriptor_set_layout: vk::DescriptorSetLayout,
    pub descriptor_pool: vk::DescriptorPool,
    pub swapchain_objects: SwapchainObjects,
    pub vertex_buffer: vk::Buffer,
    pub vertex_buffer_memory: vk::DeviceMemory,
    pub index_buffer: vk::Buffer,
    pub index_buffer_memory: vk::DeviceMemory,
    pub command_pool: vk::CommandPool,
    pub transient_command_pool: vk::CommandPool,
    pub frames_in_flight: Vec<FrameInFlight>,
}

impl SurfaceDetails {
    pub fn query(
        surface_loader: &ash::extensions::khr::Surface,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> RisResult<Self> {
        let capabilities = unsafe {
            surface_loader.get_physical_device_surface_capabilities(physical_device, surface)
        }?;
        let formats = unsafe {
            surface_loader.get_physical_device_surface_formats(physical_device, surface)
        }?;
        let present_modes = unsafe {
            surface_loader.get_physical_device_surface_present_modes(physical_device, surface)
        }?;

        Ok(Self{
            capabilities,
            formats,
            present_modes,
        })
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        ris_log::debug!("dropping renderer...");

        unsafe {
            self.device.device_wait_idle();

            for frame_in_flight in self.frames_in_flight.iter() {
                self.device.destroy_fence(frame_in_flight.in_flight_fence, None);
                self.device.destroy_semaphore(frame_in_flight.render_finished_semaphore, None);
                self.device.destroy_semaphore(frame_in_flight.image_available_semaphore, None);

                self.device.destroy_buffer(frame_in_flight.uniform_buffer, None);
                self.device.free_memory(frame_in_flight.uniform_buffer_memory, None);
            }

            self.device.destroy_command_pool(self.transient_command_pool, None);
            self.device.destroy_command_pool(self.command_pool, None);

            self.swapchain_objects.cleanup(&self.device);

            self.device.destroy_descriptor_pool(self.descriptor_pool, None);
            self.device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);

            self.device.destroy_buffer(self.vertex_buffer, None);
            self.device.free_memory(self.vertex_buffer_memory, None);
            self.device.destroy_buffer(self.index_buffer, None);
            self.device.free_memory(self.index_buffer_memory, None);

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
            let SurfaceDetails{
                formats,
                present_modes,
                ..
            } = SurfaceDetails::query(&surface_loader, physical_device, surface)?;

            log_message.push_str(&format!("\n\tsurface formats: {}", formats.len()));
            for format in formats.iter() {
                log_message.push_str(&format!("\n\t\t- {:?}, {:?}", format.format, format.color_space));
            }
            log_message.push_str(&format!("\n\tsurface present modes: {}", present_modes.len()));
            for present_mode in present_modes.iter() {
                log_message.push_str(&format!("\n\t\t- {:?}", present_mode));
            }

            ris_log::info!("{}", log_message);

            if !supports_required_extensions || formats.is_empty() || present_modes.is_empty() {
                continue; // device not supported. skip
            }

            // find queue family
            let mut queue_supports = Vec::with_capacity(device_queue_families.len());
            
            for (i, queue_family) in device_queue_families.iter().enumerate() {
                if queue_family.queue_count == 0 {
                    continue;
                }

                let graphics_queue_index = queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS).then_some(i);
                let present_queue_index = unsafe {surface_loader.get_physical_device_surface_support(physical_device, i as u32, surface)}?.then_some(i);

                queue_supports.push((
                    i,
                    graphics_queue_index,
                    present_queue_index,
                ));
            }

            // a preferred queue supports all flags
            let preferred_queue = queue_supports.iter().find(|x| {
                x.1.is_some() && x.2.is_some()
            });

            let (graphics, present) = match preferred_queue {
                Some(&(i, ..)) => {
                    (i, i)
                },
                None => {
                    // no single queue exists, which supports all flags. attempt to find multiple
                    // queues that together support all flags
                    let mut graphics = None;
                    let mut present = None;

                    for (i, graphics_queue_index, present_queue_index) in queue_supports {
                        if graphics_queue_index.is_some() {
                            graphics = Some(i);
                        }

                        if present_queue_index.is_some() {
                            present = Some(i);
                        }
                    }

                    if let (Some(graphics), Some(present)) = (graphics, present) {
                        (graphics, present)
                    } else {
                        continue; // device does not have all necessary queues. skip
                    }
                },
            };

            let suitable_device = SuitableDevice{
                suitability,
                graphics_queue_family: graphics as u32,
                present_queue_family: present as u32,
                physical_device,
            };
            suitable_devices.push(suitable_device);

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

        ris_log::debug!("chosen queue families: {:?}", unique_queue_families);

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

        // descriptor set layout
        let ubo_layout_bindings = [vk::DescriptorSetLayoutBinding{
            binding: 0,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::VERTEX,
            p_immutable_samplers: ptr::null(),
        }];

        let descriptor_set_layout_create_info = vk::DescriptorSetLayoutCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DescriptorSetLayoutCreateFlags::empty(),
            binding_count: ubo_layout_bindings.len() as u32,
            p_bindings: ubo_layout_bindings.as_ptr(),
        };

        let descriptor_set_layout = unsafe{device.create_descriptor_set_layout(&descriptor_set_layout_create_info, None)}?;

        // swap chain
        let swapchain_objects = SwapchainObjects::create(
            &instance,
            &surface_loader,
            &surface,
            &device,
            &suitable_device,
            &descriptor_set_layout,
            window.vulkan_drawable_size(),
        )?;

        // command buffer
        let command_pool_create_info = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
            queue_family_index: suitable_device.graphics_queue_family,
        };
        let command_pool = unsafe{device.create_command_pool(&command_pool_create_info, None)}?;

        let command_pool_create_info = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::CommandPoolCreateFlags::TRANSIENT,
            queue_family_index: suitable_device.graphics_queue_family,
        };
        let transient_command_pool = unsafe{device.create_command_pool(&command_pool_create_info, None)}?;

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_pool,
            level: vk::CommandBufferLevel::PRIMARY,
            command_buffer_count: MAX_FRAMES_IN_FLIGHT,
        };

        let command_buffers = unsafe {device.allocate_command_buffers(&command_buffer_allocate_info)}?;

        // vertex buffer
        let vertex_buffer_size = std::mem::size_of_val(&VERTICES) as vk::DeviceSize;
        let device_memory_properties = unsafe{instance.get_physical_device_memory_properties(suitable_device.physical_device)};

        let (staging_buffer, staging_buffer_memory) = create_buffer(
            &device,
            vertex_buffer_size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            &device_memory_properties,
        )?;
        
        unsafe{
            let data_ptr =device.map_memory(
                staging_buffer_memory,
                0,
                vertex_buffer_size,
                vk::MemoryMapFlags::empty(),
            )? as *mut Vertex;

            data_ptr.copy_from_nonoverlapping(VERTICES.as_ptr(), VERTICES.len());

            device.unmap_memory(staging_buffer_memory);
        };

        let (vertex_buffer, vertex_buffer_memory) = create_buffer(
            &device,
            vertex_buffer_size,
            vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            &device_memory_properties,
        )?;

        copy_buffer(
            &device,
            &graphics_queue,
            &transient_command_pool,
            staging_buffer,
            vertex_buffer,
            vertex_buffer_size,
        )?;

        unsafe {
            device.destroy_buffer(staging_buffer, None);
            device.free_memory(staging_buffer_memory, None);
        }

        // index buffer
        let index_buffer_size = std::mem::size_of_val(&INDICES) as vk::DeviceSize;
        let device_memory_properties = unsafe{instance.get_physical_device_memory_properties(suitable_device.physical_device)};

        let (staging_buffer, staging_buffer_memory) = create_buffer(
            &device,
            index_buffer_size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            &device_memory_properties,
        )?;
        
        unsafe{
            let data_ptr =device.map_memory(
                staging_buffer_memory,
                0,
                index_buffer_size,
                vk::MemoryMapFlags::empty(),
            )? as *mut u32;

            data_ptr.copy_from_nonoverlapping(INDICES.as_ptr(), INDICES.len());

            device.unmap_memory(staging_buffer_memory);
        };

        let (index_buffer, index_buffer_memory) = create_buffer(
            &device,
            index_buffer_size,
            vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            &device_memory_properties,
        )?;

        copy_buffer(
            &device,
            &graphics_queue,
            &transient_command_pool,
            staging_buffer,
            index_buffer,
            index_buffer_size,
        )?;

        unsafe {
            device.destroy_buffer(staging_buffer, None);
            device.free_memory(staging_buffer_memory, None);
        }

        // descriptor pool
        let descriptor_pool_sizes = [vk::DescriptorPoolSize {
            ty: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: command_buffers.len() as u32,
        }];

        let descriptor_pool_create_info = vk::DescriptorPoolCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DescriptorPoolCreateFlags::empty(),
            max_sets: command_buffers.len() as u32,
            pool_size_count: descriptor_pool_sizes.len() as u32,
            p_pool_sizes: descriptor_pool_sizes.as_ptr(),
        };

        let descriptor_pool = unsafe{device.create_descriptor_pool(&descriptor_pool_create_info, None)}?;

        let mut descriptor_set_layouts = Vec::with_capacity(command_buffers.len());
        for _ in 0..command_buffers.len() {
            descriptor_set_layouts.push(descriptor_set_layout);
        }

        let descriptor_set_allocate_info = vk::DescriptorSetAllocateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
            p_next: ptr::null(),
            descriptor_pool,
            descriptor_set_count: descriptor_set_layouts.len() as u32,
            p_set_layouts: descriptor_set_layouts.as_ptr(),
        };

        let descriptor_sets = unsafe{device.allocate_descriptor_sets(&descriptor_set_allocate_info)}?;

        // frames in flight
        let mut frames_in_flight = Vec::with_capacity(command_buffers.len());
        for i in 0..command_buffers.len() {
            let command_buffer = command_buffers[i];
            let descriptor_set = descriptor_sets[i];

            // uniform buffer
            let uniform_buffer_size = std::mem::size_of::<UniformBufferObject>() as vk::DeviceSize;
            ris_log::debug!("aschmo {}", uniform_buffer_size);
            let (uniform_buffer, uniform_buffer_memory) = create_buffer(
                &device,
                uniform_buffer_size,
                vk::BufferUsageFlags::UNIFORM_BUFFER,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
                &device_memory_properties,
            )?;
            let uniform_buffer_mapped = unsafe{device.map_memory(
                uniform_buffer_memory,
                0,
                uniform_buffer_size,
                vk::MemoryMapFlags::empty()
            )}? as *mut UniformBufferObject;

            // descriptor set
            let descriptor_buffer_info = [vk::DescriptorBufferInfo {
                buffer: uniform_buffer,
                offset: 0,
                range: std::mem::size_of::<UniformBufferObject>() as vk::DeviceSize,
            }];

            let write_descriptor_set = [vk::WriteDescriptorSet {
                s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
                p_next: ptr::null(),
                dst_set: descriptor_set,
                dst_binding: 0,
                dst_array_element: 0,
                descriptor_count: 1,
                descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                p_image_info: ptr::null(),
                p_buffer_info: descriptor_buffer_info.as_ptr(),
                p_texel_buffer_view: ptr::null(),
            }];

            unsafe{device.update_descriptor_sets(&write_descriptor_set, &[])};

            // synchronization objects
            let semaphore_create_info = vk::SemaphoreCreateInfo {
                s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::SemaphoreCreateFlags::empty(),
            };

            let fence_create_info = vk::FenceCreateInfo {
                s_type: vk::StructureType::FENCE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::FenceCreateFlags::SIGNALED,
            };

            let image_available_semaphore = unsafe{device.create_semaphore(&semaphore_create_info, None)}?;
            let render_finished_semaphore = unsafe{device.create_semaphore(&semaphore_create_info, None)}?;
            let in_flight_fence = unsafe{device.create_fence(&fence_create_info, None)}?;

            let frame_in_flight = FrameInFlight {
                command_buffer,
                descriptor_set,
                uniform_buffer,
                uniform_buffer_memory,
                uniform_buffer_mapped,
                image_available_semaphore,
                render_finished_semaphore,
                in_flight_fence,
            };

            frames_in_flight.push(frame_in_flight);
        }

        Ok(Self {
            entry,
            instance,
            debug_utils,
            surface_loader,
            surface,
            suitable_device,
            device,
            graphics_queue,
            present_queue,
            descriptor_set_layout,
            descriptor_pool,
            swapchain_objects,
            vertex_buffer,
            vertex_buffer_memory,
            index_buffer,
            index_buffer_memory,
            command_pool,
            transient_command_pool,
            frames_in_flight,
        })
    }

    pub fn recreate_swapchain(&mut self, window_size: (u32, u32)) -> RisResult<()> {
        ris_log::trace!("recreating swapchain...");

        unsafe {self.device.device_wait_idle()}?;

        self.swapchain_objects.cleanup(&self.device);
        self.swapchain_objects = SwapchainObjects::create(
            &self.instance,
            &self.surface_loader,
            &self.surface,
            &self.device,
            &self.suitable_device,
            &self.descriptor_set_layout,
            window_size,
        )?;

        ris_log::trace!("swapchain recreated!");

        Ok(())
    }
}

fn create_buffer(
    device: &ash::Device,
    size: vk::DeviceSize,
    usage: vk::BufferUsageFlags,
    memory_property_flags: vk::MemoryPropertyFlags,
    physical_device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
) -> RisResult<(vk::Buffer, vk::DeviceMemory)> {
    let buffer_create_info = vk::BufferCreateInfo {
        s_type: vk::StructureType::BUFFER_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::BufferCreateFlags::empty(),
        size,
        usage,
        sharing_mode: vk::SharingMode::EXCLUSIVE,
        queue_family_index_count: 0,
        p_queue_family_indices: ptr::null(),
    };

    let buffer = unsafe{device.create_buffer(&buffer_create_info, None)}?;

    let memory_requirements = unsafe{device.get_buffer_memory_requirements(buffer)};

    let mut memory_type = None;
    for (i, potential_memory_type) in physical_device_memory_properties.memory_types.iter().enumerate() {
        if (memory_requirements.memory_type_bits & (1 << i)) > 0 &&
            potential_memory_type.property_flags.contains(memory_property_flags) {
            memory_type = Some(i as u32);
            break;
        }
    }
    let memory_type = memory_type.unroll()?;

    let memory_allocate_info = vk::MemoryAllocateInfo {
        s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
        p_next: ptr::null(),
        allocation_size: memory_requirements.size,
        memory_type_index: memory_type,
    };

    let buffer_memory = unsafe{device.allocate_memory(&memory_allocate_info, None)}?;

    unsafe{device.bind_buffer_memory(buffer, buffer_memory, 0)}?;

    Ok((buffer, buffer_memory))
}

fn copy_buffer(
    device: &ash::Device,
    queue: &vk::Queue,
    transient_command_pool: &vk::CommandPool,
    src: vk::Buffer,
    dst: vk::Buffer,
    size: vk::DeviceSize,
) -> RisResult<()> {
    let transient_command = TransientCommand::begin(
        &device,
        &queue,
        &transient_command_pool,
    )?;

    let copy_reagions = [vk::BufferCopy {
        src_offset: 0,
        dst_offset: 0,
        size,
    }];

    unsafe {device.cmd_copy_buffer(*transient_command.buffer(), src, dst, &copy_reagions)};

    transient_command.end_and_submit()?;

    Ok(())
}

impl<'a> TransientCommand<'a> {
    fn begin(
        device: &'a ash::Device,
        queue: &'a vk::Queue,
        command_pool: &'a vk::CommandPool) -> RisResult<Self> {
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_buffer_count: 1,
            command_pool: *command_pool,
            level: vk::CommandBufferLevel::PRIMARY,
        };

        let command_buffers = unsafe {device.allocate_command_buffers(&command_buffer_allocate_info)}?;
        let command_buffer = command_buffers.first().unroll()?;

        let command_buffer_begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: ptr::null(),
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            p_inheritance_info: ptr::null(),
        };

        unsafe{device.begin_command_buffer(*command_buffer, &command_buffer_begin_info)}?;

        Ok(Self{
            device,
            queue,
            command_pool,
            command_buffers,
        })
    }

    fn buffer(&self) -> &vk::CommandBuffer {
        // cannot cause ub, because `begin(1)` would've failed if no command buffer exists
        unsafe{self.command_buffers.get_unchecked(0)}
    }

    fn end_and_submit(self) -> RisResult<()> {
        let Self {
            device,
            queue,
            command_buffers,
            ..
        } = &self;

        unsafe{device.end_command_buffer(*self.buffer())}?;

        let submit_info = [vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            p_next: ptr::null(),
            wait_semaphore_count: 0,
            p_wait_semaphores: ptr::null(),
            p_wait_dst_stage_mask: ptr::null(),
            command_buffer_count: command_buffers.len() as u32,
            p_command_buffers: command_buffers.as_ptr(),
            signal_semaphore_count: 0,
            p_signal_semaphores: ptr::null(),
        }];

        unsafe {device.queue_submit(**queue, &submit_info, vk::Fence::null())}?;

        Ok(())
    }
}

impl<'a> Drop for TransientCommand<'a> {
    fn drop(&mut self) {
        let Self {
            device,
            queue,
            command_pool,
            command_buffers,
        } = self;

        unsafe {
            ris_error::unwrap!(
                device.queue_wait_idle(**queue),
                "failed to queue wait idle",
            );

            device.free_command_buffers(**command_pool, command_buffers);
        }
    }
}

impl SwapchainObjects {
    fn cleanup(&mut self, device: &ash::Device) {
        unsafe {
            for &framebuffer in self.framebuffers.iter() {
                device.destroy_framebuffer(framebuffer, None);
            }

            device.destroy_pipeline(self.graphics_pipeline, None);
            device.destroy_pipeline_layout(self.pipeline_layout, None);
            device.destroy_render_pass(self.render_pass, None);

            for &swapchain_image_view in self.swapchain_image_views.iter() {
                device.destroy_image_view(swapchain_image_view, None);
            }

            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
        }
    }

    fn create(
        instance: &ash::Instance,
        surface_loader: &ash::extensions::khr::Surface,
        surface: &vk::SurfaceKHR,
        device: &ash::Device,
        suitable_device: &SuitableDevice,
        descriptor_set_layout: &vk::DescriptorSetLayout,
        window_size: (u32, u32),
    ) -> RisResult<Self> {
        let SurfaceDetails{
            capabilities,
            formats,
            present_modes,
        } = SurfaceDetails::query(
            &surface_loader,
            suitable_device.physical_device,
            *surface,
        )?;

        // swap chain
        let preferred_surface_format = formats
            .iter()
            .find(|x| x.format == PREFERRED_FORMAT && x.color_space == PREFERRED_COLOR_SPACE);
        let surface_format = match preferred_surface_format {
            Some(format) => format,
            // getting the first format if the preferred format does not exist. this should not
            // cause ub, becuase we checked if the list is empty at finding the suitable device.
            None => &formats[0],
        };

        let preferred_surface_present_mode = present_modes
            .iter()
            .find(|&&x| x == PREFERRED_PRESENT_MODE);
        let surface_present_mode = match preferred_surface_present_mode {
            Some(present_mode) => present_mode,
            // getting the first present mode if the preferred format does not exist. this should
            // not cause ub, because we checked if the list is empty at finding the suitable device.
            None => unsafe{present_modes.get_unchecked(0)},
        };

        let swapchain_extent = if capabilities.current_extent.width != u32::MAX {
            capabilities.current_extent
        } else {
            let (window_width, window_height) = window_size;
            let width = window_width.clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
            );
            let height = window_height.clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
            );

            vk::Extent2D {
                width,
                height,
            }
        };

        let preferred_swapchain_image_count = capabilities.min_image_count + 1;
        let swapchain_image_count = if capabilities.max_image_count == 0 {
            // SurfaceCapabilitiesKHR == 0 indicates there is no maximum
            preferred_swapchain_image_count
        } else {
            u32::min(
                preferred_swapchain_image_count,
                capabilities.max_image_count,
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
            surface: *surface,
            min_image_count: swapchain_image_count,
            image_color_space: surface_format.color_space,
            image_format: surface_format.format,
            image_extent: swapchain_extent,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode,
            p_queue_family_indices: queue_family_indices.as_ptr(),
            queue_family_index_count,
            pre_transform: capabilities.current_transform,
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

        let supbass_dependencies = [vk::SubpassDependency {
            src_subpass: vk::SUBPASS_EXTERNAL,
            dst_subpass: 0,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: vk::AccessFlags::empty(),
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dependency_flags: vk::DependencyFlags::empty(),
        }];

        let render_pass_create_info = vk::RenderPassCreateInfo {
            s_type: vk::StructureType::RENDER_PASS_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::RenderPassCreateFlags::empty(),
            attachment_count: color_attachment_descriptions.len() as u32,
            p_attachments: color_attachment_descriptions.as_ptr(),
            subpass_count: subpass_descriptions.len() as u32,
            p_subpasses: subpass_descriptions.as_ptr(),
            dependency_count: supbass_dependencies.len() as u32,
            p_dependencies: supbass_dependencies.as_ptr(),
        };

        let render_pass = unsafe{device.create_render_pass(&render_pass_create_info, None)}?;

        // graphics pipeline
        // shaders
        let vs_asset_id = AssetId::Directory(String::from("__imported_raw/shaders/tutorial.vert.spv"));
        let fs_asset_id = AssetId::Directory(String::from("__imported_raw/shaders/tutorial.frag.spv"));

        let vs_asset_future = ris_asset::load_async(vs_asset_id);
        let fs_asset_future = ris_asset::load_async(fs_asset_id);

        let vs_bytes = vs_asset_future.wait(None)??;
        let fs_bytes = fs_asset_future.wait(None)??;

        // asset data is read in u8, but vulkan expects it to be in u32.
        // assert that the data is properly aligned
        ris_error::assert!(vs_bytes.len() % 4 == 0)?;
        ris_error::assert!(fs_bytes.len() % 4 == 0)?;

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
        let vertex_binding_descriptions = Vertex::get_binding_descriptions();
        let vertex_attribute_descriptions = Vertex::get_attribute_descriptions();

        let pipeline_vertex_input_state_create_info = [vk::PipelineVertexInputStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineVertexInputStateCreateFlags::empty(),
            vertex_binding_description_count: vertex_binding_descriptions.len() as u32,
            p_vertex_binding_descriptions: vertex_binding_descriptions.as_ptr(),
            vertex_attribute_description_count: vertex_attribute_descriptions.len() as u32,
            p_vertex_attribute_descriptions: vertex_attribute_descriptions.as_ptr(),
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

        let descriptor_set_layouts = [*descriptor_set_layout];

        let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo {
            s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: descriptor_set_layouts.len() as u32,
            p_set_layouts: descriptor_set_layouts.as_ptr(),
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

        Ok(Self{
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
        })
    }
}
