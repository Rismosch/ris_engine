use std::sync::Arc;

use vulkano::device::Device;
use vulkano::render_pass::RenderPass;
use vulkano::swapchain::Swapchain;

pub fn create_render_pass(
    device: &Arc<Device>,
    swapchain: &Arc<Swapchain>,
) -> Result<Arc<RenderPass>, String> {
    Ok(vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.image_format(),
                samples: 1,
            },
        },
        pass: {
            color: [color],
            depth_stencil: {},
        },
    )
    .map_err(|e| format!("failed to create render pass: {}", e))?)
}
