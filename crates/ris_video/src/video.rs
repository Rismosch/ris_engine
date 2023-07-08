use sdl2::{video::Window, Sdl};
use vulkano::{Handle, VulkanLibrary, VulkanObject};
use vulkano::device::{
    Device,
    DeviceExtensions,
    DeviceCreateInfo,
    physical::PhysicalDeviceType,
    QueueCreateInfo,
    QueueFlags
};
use vulkano::instance::{Instance, InstanceCreateInfo, InstanceExtensions};
use vulkano::image::{ImageUsage, SwapchainImage, view::ImageView};
use vulkano::swapchain::{Surface, SurfaceApi, Swapchain, SwapchainCreateInfo};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo};

pub struct Video {
    _window: Window,
}

impl Video {
    pub fn new(sdl_context: &Sdl) -> Result<Video, String> {
        // window
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("ris_engine", 640, 480)
            .position_centered()
            .vulkan()
            .build()
            .map_err(|e| e.to_string())?;

        // instance
        let library = VulkanLibrary::new().map_err(|_| "no local vulkano library/dll")?;
        let instance_extensions = InstanceExtensions::from_iter(
            window.vulkan_instance_extensions().map_err(|_| "failed to get vulkan instance extensions")?
        );
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: instance_extensions,
                ..Default::default()
            },
        ).map_err(|_| "failed to create instance")?;

        // surface
        let surface_handle = window
            .vulkan_create_surface(instance.handle().as_raw() as _)
            .map_err(|_| "failed to create vulkan surface handle")?;
        let surface = unsafe {
            Surface::from_handle(
                instance.clone(),
                <_ as Handle>::from_raw(surface_handle),
                SurfaceApi::Win32,
                None,
            )
        };
        let surface = std::sync::Arc::new(surface);

        // physical device
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };
        let (physical_device, queue_family_index) = instance
            .enumerate_physical_devices()
            .map_err(|_| "failed to enumerate devices")?
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.contains(QueueFlags::GRAPHICS)
                            && p.surface_support(i as u32, &surface).unwrap_or(false)
                    })
                    .map(|q| (p, q as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                _ => 4,
            })
            .ok_or("no devices available")?;

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
        .map_err(|_| "failed to create device")?;

        // swapchain
        let capabilities = physical_device
            .surface_capabilities(&surface, Default::default())
            .map_err(|_| "failed to get surface capabilities")?;
        let dimensions = window.vulkan_drawable_size();
        let composite_alpha = capabilities
            .supported_composite_alpha
            .into_iter()
            .next()
            .ok_or("could not get supported composite alpha")?;
        let image_format = Some(
            physical_device
                .surface_formats(&surface, Default::default())
                .map_err(|_| "failed to get surface formats")?[0]
                .0,
        );
        let (swapchain, images) = Swapchain::new(
            device.clone(),
            surface.clone(),
            SwapchainCreateInfo {
                min_image_count: capabilities.min_image_count + 1,
                image_format,
                image_extent: [dimensions.0, dimensions.1],
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha,
                ..Default::default()
            },
        )
        .map_err(|_| "failed to create swapchain")?;

        // render pass
        let render_pass = vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.image_format(),
                    samples: 1,
                },
            },
            pass : {
                color: [color],
                depth_stencil: {},
            },
        ).map_err(|_| "failed to create render pass")?;

        // framebuffers
        let mut framebuffers = Vec::new();
        for image in images {
            let view = ImageView::new_default(image.clone()).map_err(|_| "failed to create image view")?;
            let framebuffer = Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            ).map_err(|_| "failed to create frame buffer")?;

            framebuffers.push(framebuffer);
        }
        let framebuffers = framebuffers;

        // vertex buffer
        

        
        // shaders
        

        
        // viewport
        


        // pipeline
        


        // command buffers



        let video = Video { _window: window };
        Ok(video)
    }
}
