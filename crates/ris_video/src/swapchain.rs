use std::sync::Arc;

use sdl2::video::Window;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::Device;
use vulkano::image::view::ImageView;
use vulkano::image::ImageUsage;
use vulkano::image::SwapchainImage;
use vulkano::render_pass::Framebuffer;
use vulkano::render_pass::FramebufferCreateInfo;
use vulkano::render_pass::RenderPass;
use vulkano::swapchain::Surface;
use vulkano::swapchain::Swapchain;
use vulkano::swapchain::SwapchainCreateInfo;

pub fn create_swapchain(
    physical_device: &Arc<PhysicalDevice>,
    window: &Window,
    device: &Arc<Device>,
    surface: &Arc<Surface>,
) -> Result<(Arc<Swapchain>, Vec<Arc<SwapchainImage>>), String> {
    let capabilities = physical_device
        .surface_capabilities(surface, Default::default())
        .map_err(|e| format!("failed to get surface capabilities: {}", e))?;
    let dimensions = window.vulkan_drawable_size();
    let composite_alpha = capabilities
        .supported_composite_alpha
        .into_iter()
        .next()
        .ok_or("failed to get supported composite alpha")?;
    let image_format = Some(
        physical_device
            .surface_formats(surface, Default::default())
            .map_err(|e| format!("failed to get surface formats: {}", e))?[0]
            .0,
    );
    Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo {
            min_image_count: capabilities.min_image_count,
            image_format,
            image_extent: [dimensions.0, dimensions.1],
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            composite_alpha,
            present_mode: vulkano::swapchain::PresentMode::Immediate,
            ..Default::default()
        },
    )
    .map_err(|e| format!("failed to create swapchain: {}", e))
}

pub fn create_framebuffers(
    images: &[Arc<SwapchainImage>],
    render_pass: &Arc<RenderPass>,
) -> Result<Vec<Arc<Framebuffer>>, String> {
    let mut framebuffers = Vec::new();

    for image in images {
        let view = ImageView::new_default(image.clone())
            .map_err(|e| format!("failed to create image view: {}", e))?;
        let framebuffer = Framebuffer::new(
            render_pass.clone(),
            FramebufferCreateInfo {
                attachments: vec![view],
                ..Default::default()
            },
        )
        .map_err(|e| format!("failed to create frame buffer: {}", e))?;

        framebuffers.push(framebuffer);
    }

    Ok(framebuffers)
}
