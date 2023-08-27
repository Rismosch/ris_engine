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

pub type Fence = FenceSignalFuture<
    PresentFuture<CommandBufferExecFuture<JoinFuture<Box<dyn GpuFuture>, SwapchainAcquireFuture>>>,
>;

pub struct Renderer {
    _instance: Arc<Instance>,
    pub window: Window,
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    render_pass: Arc<RenderPass>,
    framebuffers: Vec<Arc<Framebuffer>>,
    allocators: crate::allocators::Allocators,
    buffers: crate::buffers::Buffers,
    vertex_shader: Arc<ShaderModule>,
    fragment_shader: Arc<ShaderModule>,
    viewport: Viewport,
    pipeline: Arc<GraphicsPipeline>,
    command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>>,
}

impl Renderer {
    pub fn initialize(sdl_context: &Sdl) -> Result<Self, String> {
        // window
        let video_subsystem = sdl_context
            .video()
            .map_err(|e| format!("failed to get video subsystem: {}", e))?;
        let window = video_subsystem
            .window("ris_engine", 640, 480)
            //.resizable()
            .position_centered()
            .vulkan()
            .build()
            .map_err(|e| format!("failed to build window: {}", e))?;

        // instance
        let library =
            VulkanLibrary::new().map_err(|e| format!("no local vulkano library: {}", e))?;
        let instance_extensions = InstanceExtensions::from_iter(
            window
                .vulkan_instance_extensions()
                .map_err(|e| format!("failed to get vulkan instance extensions: {}", e))?,
        );
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: instance_extensions,
                ..Default::default()
            },
        )
        .map_err(|e| format!("failed to create instance: {}", e))?;

        // surface
        let surface_handle = window
            .vulkan_create_surface(instance.handle().as_raw() as _)
            .map_err(|e| format!("failed to create instance: {}", e))?;
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
        let (physical_device, queue_family_index) = crate::physical_device::select_physical_device(
            &instance,
            &surface,
            &device_extensions,
        )?;

        // device
        let (device, mut queues) = Device::new(
            physical_device.clone(),
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                enabled_extensions: device_extensions,
                ..Default::default()
            },
        )
        .map_err(|e| format!("failed to create device: {}", e))?;
        let queue = queues.next().ok_or("no queues available")?;

        // swapchain
        let (swapchain, images) =
            crate::swapchain::create_swapchain(&physical_device, &window, &device, &surface)?;

        // render pass
        let render_pass = crate::render_pass::create_render_pass(&device, &swapchain)?;

        // shaders
        let (vertex_shader, fragment_shader) = crate::shaders::compile_shaders(&device)?;

        // viewport
        let (w, h) = window.vulkan_drawable_size();
        let viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [w as f32, h as f32],
            depth_range: 0.0..1.0,
        };

        // allocators
        let allocators = crate::allocators::Allocators::new(&device);

        // frame buffers
        let framebuffers =
            crate::swapchain::create_framebuffers(&allocators, [w, h], &images, &render_pass)?;

        // pipeline
        let pipeline = crate::pipeline::create_pipeline(
            &device,
            &vertex_shader,
            &fragment_shader,
            &render_pass,
            &viewport,
        )?;

        // buffers
        let buffers = crate::buffers::Buffers::new(&allocators, images.len(), &pipeline)?;

        // command buffers
        let command_buffers = crate::command_buffers::create_command_buffers(
            &allocators,
            &queue,
            &pipeline,
            &framebuffers,
            &buffers,
        )?;

        // return
        Ok(Self {
            _instance: instance,
            window,
            device,
            queue,
            swapchain,
            images,
            render_pass,
            framebuffers,
            allocators,
            buffers,
            vertex_shader,
            fragment_shader,
            viewport,
            pipeline,
            command_buffers,
        })
    }

    pub fn recreate_swapchain(&mut self) -> Result<(), String> {
        let new_dimensions = self.window.vulkan_drawable_size();
        let (new_swapchain, new_images) = match self.swapchain.recreate(SwapchainCreateInfo {
            image_extent: [new_dimensions.0, new_dimensions.1],
            ..self.swapchain.create_info()
        }) {
            Ok(r) => r,
            Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return Ok(()),
            Err(e) => return Err(format!("failed to recreate swapchain: {}", e)),
        };

        self.swapchain = new_swapchain;
        let (w, h) = self.window.vulkan_drawable_size();
        self.framebuffers = crate::swapchain::create_framebuffers(
            &self.allocators,
            [w, h],
            &new_images,
            &self.render_pass,
        )?;

        Ok(())
    }

    pub fn recreate_viewport(&mut self) -> Result<(), String> {
        self.recreate_swapchain()?;
        let (w, h) = self.window.vulkan_drawable_size();
        self.viewport.dimensions = [w as f32, h as f32];

        self.pipeline = crate::pipeline::create_pipeline(
            &self.device,
            &self.vertex_shader,
            &self.fragment_shader,
            &self.render_pass,
            &self.viewport,
        )?;

        self.command_buffers = crate::command_buffers::create_command_buffers(
            &self.allocators,
            &self.queue,
            &self.pipeline,
            &self.framebuffers,
            &self.buffers,
        )?;

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
    ) -> Result<Result<Fence, FlushError>, String> {
        Ok(previous_future
            .join(swqapchain_acquire_future)
            .then_execute(
                self.queue.clone(),
                self.command_buffers[image_i as usize].clone(),
            )
            .map_err(|e| format!("failed to execute command buffer: {}", e))?
            .then_swapchain_present(
                self.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), image_i),
            )
            .then_signal_fence_and_flush())
    }

    pub fn update_uniform(
        &self,
        index: usize,
        ubo: &crate::gpu_objects::UniformBufferObject,
    ) -> Result<(), String> {
        let mut uniform_content = self.buffers.uniforms[index]
            .0
            .write()
            .map_err(|e| format!("failed to update uniform: {}", e))?;

        uniform_content.view = ubo.view.transposed();
        uniform_content.proj = ubo.proj.transposed();
        uniform_content.view_proj = ubo.view_proj.transposed();
        uniform_content.debug_x = ubo.debug_x;
        uniform_content.debug_y = ubo.debug_y;

        Ok(())
    }
}
