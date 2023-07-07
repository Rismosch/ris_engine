use sdl2::{video::Window, Sdl};
use vulkano::{Handle, VulkanLibrary, VulkanObject};
use vulkano::instance::{Instance, InstanceCreateInfo, InstanceExtensions};
use vulkano::swapchain::{Surface, SurfaceApi};

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
        let enabled_extensions = InstanceExtensions::from_iter(
            window.vulkan_instance_extensions().map_err(|_| "could not get vulkan instance extensions")?
        );
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions,
                ..Default::default()
            },
        ).map_err(|_| "failed to create instance")?;

        // surface
        let surface_handle = window
            .vulkan_create_surface(instance.handle().as_raw() as _)
            .map_err(|_| "could not create vulkan surface handle")?;
        let surface = unsafe {
            Surface::from_handle(
                instance.clone(),
                <_ as Handle>::from_raw(surface_handle),
                SurfaceApi::Win32,
                None,
            )
        };

        let video = Video { _window: window };
        Ok(video)
    }
}
