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
use ris_error::RisResult;

use crate::vulkan::util;

const REQUIRED_INSTANCE_LAYERS: &[&str] = &["VK_LAYER_KHRONOS_validation"];
#[cfg(not(debug_assertions))]
const VALIDATION_ENABLED: bool = false;
#[cfg(debug_assertions)]
const VALIDATION_ENABLED: bool = true;

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

    let message = CStr::from_ptr((*p_callback_data).p_message);

    ris_log::log!(log_level, "VULKAN {} | {:?}", type_flag, message);

    vk::FALSE
}

pub struct Renderer {
    _entry: ash::Entry,
    instance: ash::Instance,
    debug_utils: Option<(ash::extensions::ext::DebugUtils, vk::DebugUtilsMessengerEXT)>,
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
        let entry = unsafe {ash::Entry::load()?};
        
        // extensions
        let mut count = 0;
        if unsafe {
            sdl2_sys::SDL_Vulkan_GetInstanceExtensions(window.raw(), &mut count, ptr::null_mut())
        } == sdl2_sys::SDL_bool::SDL_FALSE {
            return ris_error::new_result!("{}", sdl2::get_error());
        }

        let mut extensions = vec![ptr::null(); count as usize];

        if unsafe {
            sdl2_sys::SDL_Vulkan_GetInstanceExtensions(window.raw(), &mut count, extensions.as_mut_ptr())
        } == sdl2_sys::SDL_bool::SDL_FALSE {
            return ris_error::new_result!("{}", sdl2::get_error());
        }

        // validation layers
        //let mut debug_utils_messenger_create_info = ptr::null();

        let available_layers = if !VALIDATION_ENABLED {
            ris_log::info!("instance layers are disabled");
            (0, ptr::null())
        } else {
            // add debug util extension
            extensions.push(ash::extensions::ext::DebugUtils::name().as_ptr());

            // find and collect available layers
            let layer_properties = entry.enumerate_instance_layer_properties()?;
            if layer_properties.is_empty() {
                ris_log::warning!("no available instance layers");
                (0, ptr::null())
            } else {
                let mut log_message = String::from("available instance layers:");
                for layer in layer_properties.iter() {
                    let name = util::vk_to_string(&layer.layer_name)?;
                    log_message.push_str(&format!("\n    - {}", name));
                }
                ris_log::trace!("{}", log_message);

                let mut available_layers = Vec::new();
                let mut log_message = String::from("instance layers to be enabled:");

                for required_layer in REQUIRED_INSTANCE_LAYERS {
                    let mut layer_found = false;

                    for layer in layer_properties.iter() {
                        let name = util::vk_to_string(&layer.layer_name)?;
                        if (*required_layer) == name {
                            available_layers.push(layer.layer_name.as_ptr());
                            layer_found = true;
                            break;
                        }
                    }

                    if !layer_found {
                        ris_log::warning!("layer \"{}\" is not available", required_layer);
                    } else {
                        log_message.push_str(&format!("\n    - {}", required_layer));
                    }
                }

                ris_log::info!("{}", log_message);

                (0, available_layers.as_ptr())
            }
        };

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
            pp_enabled_extension_names: extensions.as_ptr(),
            enabled_extension_count: extensions.len() as u32,
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
                    vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
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

        //return ris_error::new_result!("test");

        Ok(Self {
            _entry: entry,
            instance,
            debug_utils,
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
