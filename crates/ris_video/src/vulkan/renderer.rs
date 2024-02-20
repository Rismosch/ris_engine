use std::ffi::CString;
use std::ptr;
use std::sync::Arc;

use ash::vk;
use sdl2::video::Window;
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::device::Device;
use vulkano::device::DeviceCreateInfo;
use vulkano::device::DeviceExtensions;
use vulkano::device::Queue;
use vulkano::device::QueueCreateInfo;
use vulkano::image::SwapchainImage;
use vulkano::instance::Instance;
use vulkano::instance::InstanceCreateInfo;
use vulkano::instance::InstanceExtensions;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::Framebuffer;
use vulkano::render_pass::RenderPass;
use vulkano::shader::ShaderModule;
use vulkano::swapchain;
use vulkano::swapchain::AcquireError;
use vulkano::swapchain::Surface;
use vulkano::swapchain::SurfaceApi;
use vulkano::swapchain::Swapchain;
use vulkano::swapchain::SwapchainAcquireFuture;
use vulkano::swapchain::SwapchainCreateInfo;
use vulkano::swapchain::SwapchainCreationError;
use vulkano::sync;
use vulkano::sync::future::NowFuture;
use vulkano::sync::GpuFuture;
use vulkano::Handle;
use vulkano::VulkanLibrary;
use vulkano::VulkanObject;

use ris_data::info::app_info::AppInfo;
use ris_asset::loader::scenes_loader::Scenes;
use ris_error::Extensions;
use ris_error::RisResult;

use crate::vulkan::allocators::Allocators;
use crate::vulkan::buffers::Buffers;

pub struct Renderer {
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

impl Renderer {
    pub fn initialize(app_info: &AppInfo, window: &Window, scenes: Scenes) -> RisResult<Self> {
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

        // instance
        let entry = unsafe {ash::Entry::load()?};
        let app_name = CString::new(app_info.package.name.clone())?;
        let engine_name = CString::new("vulkan (ash) engine")?;
        let vk_app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: app_name.as_ptr(),
            application_version: vk::make_api_version(0, 0, 1, 0),
            p_engine_name: engine_name.as_ptr(),
            engine_version: vk::make_api_version(0, 0, 1, 0),
            api_version: vk::make_api_version(0, 1, 0, 92),
        };

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &vk_app_info,
            pp_enabled_layer_names: ptr::null(),
            enabled_layer_count: 0,
            pp_enabled_extension_names: extensions.as_ptr(),
            enabled_extension_count: extensions.len() as u32,
        };

        let instance = unsafe {
            entry.create_instance(&create_info, None)?
        };

        ris_error::new_result!("reached end 10");
        // // allocators
        // let allocators = super::allocators::Allocators::new(device.clone());

        // // frame buffers
        // let framebuffers = super::swapchain::create_framebuffers(
        //     &allocators,
        //     dimensions,
        //     &images,
        //     render_pass.clone(),
        // )?;

        // // pipeline
        // let vs = vs_future.wait(None)??;
        // let fs = fs_future.wait(None)??;

        // let pipeline = super::pipeline::create_pipeline(
        //     device.clone(),
        //     vs.clone(),
        //     fs.clone(),
        //     render_pass.clone(),
        //     &viewport,
        // )?;

        // // buffers
        // let buffers = super::buffers::Buffers::new(&allocators, images.len(), pipeline.clone())?;

        // // command buffers
        // let command_buffers = super::command_buffers::create_command_buffers(
        //     &allocators,
        //     queue.clone(),
        //     pipeline.clone(),
        //     &framebuffers,
        //     &buffers,
        // )?;

        // // return
        // Ok(Self {
        //     instance,
        //     device,
        //     queue,
        //     swapchain,
        //     images,
        //     render_pass,
        //     framebuffers,
        //     allocators,
        //     buffers,
        //     vertex_shader: vs,
        //     fragment_shader: fs,
        //     scenes,
        //     viewport,
        //     pipeline,
        //     command_buffers,
        // })
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
