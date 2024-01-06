use std::sync::Arc;

use sdl2::video::Window;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::Device;
use vulkano::image::view::ImageView;
use vulkano::image::AttachmentImage;
use vulkano::image::ImageUsage;
use vulkano::image::ImageViewAbstract;
use vulkano::image::SwapchainImage;
use vulkano::render_pass::Framebuffer;
use vulkano::render_pass::FramebufferCreateInfo;
use vulkano::render_pass::RenderPass;
use vulkano::swapchain::Surface;
use vulkano::swapchain::Swapchain;
use vulkano::swapchain::SwapchainCreateInfo;

use ris_error::RisResult;

pub fn create_swapchain(
    physical_device: &Arc<PhysicalDevice>,
    window: &Window,
    device: &Arc<Device>,
    surface: &Arc<Surface>,
) -> RisResult<(Arc<Swapchain>, Vec<Arc<SwapchainImage>>)> {
    let capabilities = ris_error::unroll!(
        physical_device.surface_capabilities(surface, Default::default()),
        "failed to get surface capabilities"
    )?;
    let dimensions = window.vulkan_drawable_size();
    let composite_alpha = ris_error::unroll_option!(
        capabilities.supported_composite_alpha.into_iter().next(),
        "failed to get supported composite alpha"
    )?;
    let image_format = Some(
        ris_error::unroll!(
            physical_device.surface_formats(surface, Default::default()),
            "failed to get surface formats"
        )?[0]
            .0,
    );
    ris_error::unroll!(
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
        ),
        "failed to create swapchain"
    )
}

pub fn create_framebuffers(
    allocators: &crate::allocators::Allocators,
    dimensions: [u32; 2],
    images: &[Arc<SwapchainImage>],
    render_pass: &Arc<RenderPass>,
) -> RisResult<Vec<Arc<Framebuffer>>> {
    let depth_buffer = ris_error::unroll!(
        AttachmentImage::transient(&allocators.memory, dimensions, super::DEPTH_FORMAT),
        "failed to create frame buffer"
    )?;

    let mut framebuffers = Vec::new();
    for image in images {
        let image_view = ris_error::unroll!(
            ImageView::new_default(image.clone()),
            "failed to create image view"
        )?;

        let depth_view = ris_error::unroll!(
            ImageView::new_default(depth_buffer.clone()),
            "failed to create depth view"
        )?;

        let attachments: Vec<Arc<dyn ImageViewAbstract>> = vec![image_view, depth_view];

        let framebuffer = ris_error::unroll!(
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments,
                    ..Default::default()
                },
            ),
            "failed to create frame buffer"
        )?;

        framebuffers.push(framebuffer);
    }

    Ok(framebuffers)
}
