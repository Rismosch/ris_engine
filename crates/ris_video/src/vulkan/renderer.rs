use std::sync::Arc;

use sdl2::video::Window;
use sdl2::Sdl;
use vulkano::command_buffer::CommandBufferExecFuture;
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
use vulkano::swapchain::PresentFuture;
use vulkano::swapchain::Surface;
use vulkano::swapchain::SurfaceApi;
use vulkano::swapchain::Swapchain;
use vulkano::swapchain::SwapchainAcquireFuture;
use vulkano::swapchain::SwapchainCreateInfo;
use vulkano::swapchain::SwapchainCreationError;
use vulkano::swapchain::SwapchainPresentInfo;
use vulkano::sync;
use vulkano::sync::future::FenceSignalFuture;
use vulkano::sync::future::JoinFuture;
use vulkano::sync::future::NowFuture;
use vulkano::sync::FlushError;
use vulkano::sync::GpuFuture;
use vulkano::Handle;
use vulkano::VulkanLibrary;
use vulkano::VulkanObject;

use ris_asset::loader::scenes_loader::Scenes;
use ris_error::RisResult;

use crate::vulkan::allocators::Allocators;
use crate::vulkan::buffers::Buffers;

pub type Fence = FenceSignalFuture<
    PresentFuture<CommandBufferExecFuture<JoinFuture<Box<dyn GpuFuture>, SwapchainAcquireFuture>>>,
>;

pub struct Renderer {
    pub instance: Arc<Instance>,
    pub window: Window,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub swapchain: Arc<Swapchain>,
    pub images: Vec<Arc<SwapchainImage>>,
    pub render_pass: Arc<RenderPass>,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    pub allocators: Allocators,
    pub buffers: Buffers,
    pub vertex_shader: Arc<ShaderModule>,
    pub fragment_shader: Arc<ShaderModule>,
    pub scenes: Scenes,
    pub viewport: Viewport,
    pub pipeline: Arc<GraphicsPipeline>,
    pub command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>>,
}

impl Renderer {
    pub fn initialize(sdl_context: &Sdl, scenes: Scenes) -> RisResult<Self> {
        // window
        let video_subsystem = sdl_context
            .video()
            .map_err(|e| ris_error::new!("failed to get video subsystem: {}", e))?;
        let window = ris_error::unroll!(
            video_subsystem
                .window("ris_engine", 640, 480)
                //.resizable()
                .position_centered()
                .vulkan()
                .build(),
            "failed to build window"
        )?;

        // instance
        let library = ris_error::unroll!(VulkanLibrary::new(), "no local vulkano library")?;
        let instance_extensions = InstanceExtensions::from_iter(
            window
                .vulkan_instance_extensions()
                .map_err(|e| ris_error::new!("failed to get vulkan instance extensions: {}", e))?,
        );

        let instance = ris_error::unroll!(
            Instance::new(
                library,
                InstanceCreateInfo {
                    enabled_extensions: instance_extensions,
                    ..Default::default()
                },
            ),
            "failed to create instance"
        )?;

        // surface
        let surface_handle = window
            .vulkan_create_surface(instance.handle().as_raw() as _)
            .map_err(|e| ris_error::new!("failed to create instance: {}", e))?;
        let surface = unsafe {
            Surface::from_handle(
                instance.clone(),
                <_ as Handle>::from_raw(surface_handle),
                SurfaceApi::Win32,
                None,
            )
        };
        let surface = Arc::new(surface);

        // physical device
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };
        let (physical_device, queue_family_index) = super::physical_device::select_physical_device(
            instance.clone(),
            surface.clone(),
            &device_extensions,
        )?;

        // device
        let (device, mut queues) = ris_error::unroll!(
            Device::new(
                physical_device.clone(),
                DeviceCreateInfo {
                    queue_create_infos: vec![QueueCreateInfo {
                        queue_family_index,
                        ..Default::default()
                    }],
                    enabled_extensions: device_extensions,
                    ..Default::default()
                },
            ),
            "failed to create device"
        )?;
        let queue = ris_error::unroll_option!(queues.next(), "no queues available")?;
        
        // shaders
        let vs_future = super::shader::load_async(
            device.clone(),
            scenes.default_vs.clone(),
        );
        let fs_future = super::shader::load_async(
            device.clone(),
            scenes.default_fs.clone(),
        );

        // swapchain
        let (swapchain, images) =
            super::swapchain::create_swapchain(
                physical_device.clone(),
                &window,
                device.clone(),
                surface.clone(),
            )?;

        // render pass
        let render_pass = super::render_pass::create_render_pass(device, swapchain)?;

        // viewport
        let (w, h) = window.vulkan_drawable_size();
        let viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [w as f32, h as f32],
            depth_range: 0.0..1.0,
        };

        // allocators
        let allocators = super::allocators::Allocators::new(device.clone());

        // frame buffers
        let framebuffers =
            super::swapchain::create_framebuffers(&allocators, [w, h], &images, render_pass.clone())?;

        // pipeline
        let vs = vs_future.wait()?;
        let fs = fs_future.wait()?;

        let pipeline = super::pipeline::create_pipeline(
            device.clone(),
            vs.clone(),
            fs.clone(),
            render_pass.clone(),
            &viewport,
        )?;

        // buffers
        let buffers = super::buffers::Buffers::new(
            &allocators,
            images.len(),
            pipeline.clone(),
        )?;

        // command buffers
        let command_buffers = super::command_buffers::create_command_buffers(
            &allocators,
            queue.clone(),
            pipeline.clone(),
            &framebuffers,
            &buffers,
        )?;

        // return
        Ok(Self {
            instance,
            window,
            device,
            queue,
            swapchain,
            images,
            render_pass,
            framebuffers,
            allocators,
            buffers,
            vertex_shader: vs,
            fragment_shader: fs,
            scenes,
            viewport,
            pipeline,
            command_buffers,
        })
    }

    pub fn recreate_swapchain(&mut self) -> RisResult<()> {
        ris_log::trace!("recreating swapchain...");

        let new_dimensions = self.window.vulkan_drawable_size();
        let (new_swapchain, new_images) = match self.swapchain.recreate(SwapchainCreateInfo {
            image_extent: [new_dimensions.0, new_dimensions.1],
            ..self.swapchain.create_info()
        }) {
            Ok(r) => r,
            Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return Ok(()),
            Err(e) => return ris_error::new_result!("failed to recreate swapchain: {}", e),
        };

        self.swapchain = new_swapchain;
        let (w, h) = self.window.vulkan_drawable_size();
        self.framebuffers = super::swapchain::create_framebuffers(
            &self.allocators,
            [w, h],
            &new_images,
            self.render_pass.clone(),
        )?;

        ris_log::trace!("swapcain recreated!");
        Ok(())
    }

    pub fn recreate_viewport(&mut self) -> RisResult<()> {
        ris_log::trace!("recreating viewport...");

        self.recreate_swapchain()?;
        let (w, h) = self.window.vulkan_drawable_size();
        self.viewport.dimensions = [w as f32, h as f32];

        self.pipeline = super::pipeline::create_pipeline(
            self.device.clone(),
            self.vertex_shader.clone(),
            self.fragment_shader.clone(),
            self.render_pass.clone(),
            &self.viewport,
        )?;

        self.command_buffers = super::command_buffers::create_command_buffers(
            &self.allocators,
            self.queue.clone(),
            self.pipeline.clone(),
            &self.framebuffers,
            &self.buffers,
        )?;

        ris_log::trace!("viewport recreated!");
        Ok(())
    }

    pub fn reload_shaders(&mut self) -> RisResult<()> {
        ris_log::trace!("reloading shaders...");

        let vertex_future = super::shader::load_async(
            self.device.clone(),
            self.scenes.default_vs.clone(),
        );
        let fragment_future = super::shader::load_async(
            self.device.clone(),
            self.scenes.default_vs.clone(),
        );

        let vertex_shader = vertex_future.wait()?;
        let fragment_shader = fragment_future.wait()?;

        self.vertex_shader = vertex_shader;
        self.fragment_shader = fragment_shader;

        ris_log::trace!("shaders reloaded!");
        Ok(())
    }

    pub fn get_image_count(&self) -> usize {
        self.images.len()
    }

    pub fn acquire_swapchain_image(
        &self,
    ) -> Result<(u32, bool, SwapchainAcquireFuture), AcquireError> {
        swapchain::acquire_next_image(self.swapchain.clone(), None)
    }

    pub fn synchronize(&self) -> NowFuture {
        let mut now = sync::now(self.device.clone());
        now.cleanup_finished();
        now
    }

    pub fn flush_next_future(
        &self,
        previous_future: Box<dyn GpuFuture>,
        swqapchain_acquire_future: SwapchainAcquireFuture,
        image_i: u32,
    ) -> RisResult<Result<Fence, FlushError>> {
        Ok(previous_future
            .join(swqapchain_acquire_future)
            .then_execute(
                self.queue.clone(),
                self.command_buffers[image_i as usize].clone(),
            )
            .map_err(|e| ris_error::new!("failed to execute command buffer: {}", e))?
            .then_swapchain_present(
                self.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), image_i),
            )
            .then_signal_fence_and_flush())
    }

    pub fn update_uniform(
        &self,
        index: usize,
        ubo: &super::gpu_objects::UniformBufferObject,
    ) -> RisResult<()> {
        let mut uniform_content = ris_error::unroll!(
            self.buffers.uniforms[index].0.write(),
            "failed to update uniform"
        )?;

        uniform_content.view = ubo.view.transposed();
        uniform_content.proj = ubo.proj.transposed();
        uniform_content.view_proj = ubo.view_proj.transposed();

        Ok(())
    }
}