use std::sync::Arc;

use vulkano::device::Device;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState
use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::pipeline::graphics::viewport::ViewportState;
use vulkano::render_pass::RenderPass;
use vulkano::render_pass::Subpass;
use vulkano::shader::ShaderModule;

use crate::gpu_objects::RisVertex;

pub fn create_pipeline(
    device: &Arc<Device>,
    vertex_shader: &Arc<ShaderModule>,
    fragment_shader: &Arc<ShaderModule>,
    render_pass: &Arc<RenderPass>,
    viewport: Viewport,
) -> Result<Arc<GraphicsPipeline>, String> {
    Ok(
        GraphicsPipeline::start()
        .vertex_input_state(RisVertex::per_vertex())
        .vertex_shader(
            vertex_shader.clone()
            .entry_point("main")
            .ok_or("failed to locate vertex entry point")?,
            (),
        )
        .input_assembly_state(InputAssemblyState::new())
        .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([viewport]))
        .fragment_shader(
            fragment_shader.clone()
            .entry_point("main")
            .ok_or("failed to locate fragmetn entry point")?,
            (),
        )
        .render_pass(
            Subpass::from(render_pass.clone(), 0)
                .ok_or("failed to create render subpass")?
        )
        .build(device.clone())
        .map_err(|e| format!("failed to build graphics pipeline"))?
    )
}
