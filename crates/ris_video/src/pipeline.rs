use std::sync::Arc;

use vulkano::device::Device;
use vulkano::pipeline::graphics::depth_stencil::CompareOp;
use vulkano::pipeline::graphics::depth_stencil::DepthState;
use vulkano::pipeline::graphics::depth_stencil::DepthStencilState;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::rasterization::CullMode;
use vulkano::pipeline::graphics::rasterization::FrontFace;
use vulkano::pipeline::graphics::rasterization::RasterizationState;
use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::pipeline::graphics::viewport::ViewportState;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::StateMode;
use vulkano::render_pass::RenderPass;
use vulkano::render_pass::Subpass;
use vulkano::shader::ShaderModule;

use crate::gpu_objects::Vertex3d;

pub fn create_pipeline(
    device: &Arc<Device>,
    vertex_shader: &Arc<ShaderModule>,
    fragment_shader: &Arc<ShaderModule>,
    render_pass: &Arc<RenderPass>,
    viewport: &Viewport,
) -> Result<Arc<GraphicsPipeline>, String> {
    GraphicsPipeline::start()
        .vertex_input_state(Vertex3d::per_vertex())
        .vertex_shader(
            vertex_shader
                .clone()
                .entry_point("main")
                .ok_or("failed to locate vertex entry point")?,
            (),
        )
        .input_assembly_state(InputAssemblyState::new())
        .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([
            viewport.clone()
        ]))
        .fragment_shader(
            fragment_shader
                .clone()
                .entry_point("main")
                .ok_or("failed to locate fragment entry point")?,
            (),
        )
        .rasterization_state(RasterizationState {
            front_face: StateMode::Fixed(FrontFace::CounterClockwise),
            cull_mode: StateMode::Fixed(CullMode::Back),
            ..Default::default()
        })
        .depth_stencil_state(DepthStencilState {
            depth: Some(DepthState {
                enable_dynamic: false,
                compare_op: StateMode::Fixed(CompareOp::Greater),
                write_enable: StateMode::Fixed(true),
            }),
            ..Default::default()
        })
        .render_pass(
            Subpass::from(render_pass.clone(), 0).ok_or("failed to create render subpass")?,
        )
        .build(device.clone())
        .map_err(|e| format!("failed to build graphics pipeline: {}", e))
}
