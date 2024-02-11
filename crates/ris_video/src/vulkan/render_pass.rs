use std::sync::Arc;

use vulkano::device::Device;
use vulkano::render_pass::RenderPass;
use vulkano::swapchain::Swapchain;

use ris_error::RisResult;

pub fn create_render_pass(
    device: Arc<Device>,
    swapchain: Arc<Swapchain>,
) -> RisResult<Arc<RenderPass>> {
    let render_pass = vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.image_format(),
                samples: 1,
            },
            depth: {
                load: Clear,
                store: DontCare,
                format: super::DEPTH_FORMAT,
                samples: 1,
            },
        },
        pass: {
            color: [color],
            depth_stencil: {depth},
        },
    )?;

    Ok(render_pass)
}
