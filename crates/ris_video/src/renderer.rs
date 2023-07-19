use std::sync::Arc;

use sdl2::Sdl;
use sdl2::video::Window;
use vulkano::command_buffer::CommandBufferExecFuture;
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::device::Device;
use vulkano::device::DeviceCreateInfo;
use vulkano::device::DeviceExtensions;
use vulkano::device::Queue;
use vulkano::device::QueueCreateInfo;
use vulkano::Handle;
use vulkano::image::SwapchainImage;
use vulkano::instance::Instance;
use vulkano::instance::InstanceCreateInfo;
use vulkano::instance::InstanceExtensions;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::Framebuffer;
use vulkano::render_pass::RenderPass;
use vulkano::shader::ShaderModule;
use vulkano::swapchain::AcquireError;
use vulkano::swapchain::PresentFuture;
use vulkano::swapchain::Surface;
use vulkano::swapchain::SurfaceApi;
use vulkano::swapchain::Swapchain;
use vulkano::swapchain::SwapchainAcquireFuture;
use vulkano::sync::future::FenceSignalFuture;
use vulkano::sync::future::JoinFuture;
use vulkano::sync::future::NowFuture;
use vulkano::sync::FlushError;
use vulkano::sync::GpuFuture;
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
        let video_subsystem = sdl_context.video().map_err(|e| format!("failed to get video subsystem: {}", e))?;
        let window = video_subsystem
            .window("ris_engine", 640, 480)
            //.resizable()
            .position_centered()
            .vulkan()
            .build()
            .map_err(|e| format!("failed to build window: {}", e))?;

        // instance
        let library = VulkanLibrary::new().map_err(|e| format!("no local vulkano library: {}", e))?;
        let instance_extensions = InstanceExtensions::from_iter(
            window
            .vulkan_instance_extensions()
            .map_err(|e| format!("failed to get vulkan instance extensions: {}", e))?
        );
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: instance_extensions,
                ..Default::default()
            },
        ).map_err(|e| format!("failed to create instance: {}", e))?;

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
        let device_extensions = DeviceExtensions{
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
            DeviceCreateInfo{
                queue_create_infos: vec![QueueCreateInfo{
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
        let (swapchain, images) = crate::swapchain::create_swapchain(
            &physical_device,
            &window,
            &device,
            &surface,
        )?;

        // render pass
        let render_pass = crate::render_pass::create_render_pass(
            &device,
            &swapchain,
        )?;

        // frame buffers
        let framebuffers = crate::swapchain::create_framebuffers(
            &images,
            &render_pass,
        )?;

        // shaders
        let (vertex_shader, fragment_shader) = crate::shaders::compile_shaders(&device)?;

        // viewport
        let (w, h) = window.vulkan_drawable_size();
        let viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [w as f32, h as f32],
            depth_range: 0.0..1.0,
        };

        // pipeline
        let pipeline = crate::pipeline::create_pipeline(
            &device,
            &vertex_shader,
            &fragment_shader,
            &render_pass,
            viewport,
        )?;

        // allocators
        let allocators = crate::allocators::Allocators::new(&device);

        // buffers
        let buffers = crate::buffers::Buffers::new(
            &allocators,
            images.len(),
            &pipeline,
        )?;

        // command buffers
        let command_buffers = crate::command_buffers::create_command_buffers(
            &allocators,
            &queue,
            &pipeline,
            &framebuffers,
            &buffers,
        )?;

        // return
        Ok(Self{
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
            command_buffers
        })
    }

    pub fn recreate_swapchain(&mut self) {
    }

    pub fn recreate_viewport(&mut self) {
    }

    pub fn get_image_count(&self) -> usize {

    }

    pub fn acquire_swapchain_image(&self) -> Result<(u32, bool, SwapchainAcquireFuture), AcquireError> {

    }

    pub fn synchronize(&self) -> NowFuture {

    }

    pub fn flush_next_future(
        &self,
        previous_future: Box<dyn GpuFuture>,
        swqapchain_acquire_future: SwapchainAcquireFuture,
        image_i: u32,
    ) -> Result<Fence, FlushError> {

    }

    pub fn update_uniform(&self, index: u32) {
    }
}
