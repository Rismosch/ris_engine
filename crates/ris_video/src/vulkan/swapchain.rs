use std::sync::Arc;

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

use ris_error::Extensions;
use ris_error::RisResult;

use crate::vulkan::allocators::Allocators;

pub fn create_swapchain(
    physical_device: Arc<PhysicalDevice>,
    dimensions: (u32, u32),
    device: Arc<Device>,
    surface: Arc<Surface>,
) -> RisResult<(Arc<Swapchain>, Vec<Arc<SwapchainImage>>)> {
    let capabilities = physical_device.surface_capabilities(&surface, Default::default())?;
    let composite_alpha = capabilities
        .supported_composite_alpha
        .into_iter()
        .next()
        .unroll()?;
    let image_format = Some(physical_device.surface_formats(&surface, Default::default())?[0].0);
    let swapchain = Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo {
            min_image_count: capabilities.min_image_count,
            image_format,
            image_extent: [dimensions.0, dimensions.1],
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            composite_alpha,
            present_mode: vulkano::swapchain::PresentMode::Mailbox,
            ..Default::default()
        },
    )?;

    ris_log::trace!("swapchain image coint: {}", swapchain.0.image_count());

    Ok(swapchain)
}

pub fn create_framebuffers(
    allocators: &Allocators,
    dimensions: (u32, u32),
    images: &[Arc<SwapchainImage>],
    render_pass: Arc<RenderPass>,
) -> RisResult<Vec<Arc<Framebuffer>>> {
    let depth_buffer = AttachmentImage::transient(
        &allocators.memory,
        [dimensions.0, dimensions.1],
        super::DEPTH_FORMAT,
    )?;

    let mut framebuffers = Vec::new();
    for image in images {
        let image_view = ImageView::new_default(image.clone())?;

        let depth_view = ImageView::new_default(depth_buffer.clone())?;

        let attachments: Vec<Arc<dyn ImageViewAbstract>> = vec![image_view, depth_view];

        let framebuffer = Framebuffer::new(
            render_pass.clone(),
            FramebufferCreateInfo {
                attachments,
                ..Default::default()
            },
        )?;

        framebuffers.push(framebuffer);
    }

    Ok(framebuffers)
}
