use std::sync::Arc;

use vulkano::device::Device;
use vulkano::pipeline::graphics::color_blend::AttachmentBlend;
use vulkano::pipeline::graphics::color_blend::ColorBlendAttachmentState;
use vulkano::pipeline::graphics::color_blend::ColorBlendState;
use vulkano::pipeline::graphics::color_blend::ColorComponents;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::viewport::ViewportState;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::StateMode;
use vulkano::render_pass::RenderPass;
use vulkano::render_pass::Subpass;
use vulkano::shader::ShaderModule;

use ris_error::Extensions;
use ris_error::RisResult;

use crate::imgui::gpu_objects::ImguiVertex;

pub fn create_pipeline(
    device: Arc<Device>,
    vs: Arc<ShaderModule>,
    fs: Arc<ShaderModule>,
    render_pass: Arc<RenderPass>,
) -> RisResult<Arc<GraphicsPipeline>> {
    let pipeline = GraphicsPipeline::start()
        .vertex_input_state(ImguiVertex::input_state())
        .vertex_shader(vs.clone().entry_point("main").unroll()?, ())
        .input_assembly_state(InputAssemblyState::new())
        .viewport_state(ViewportState::Dynamic {
            count: 1,
            viewport_count_dynamic: false,
            scissor_count_dynamic: false,
        })
        .fragment_shader(fs.clone().entry_point("main").unroll()?, ())
        .color_blend_state(ColorBlendState {
            attachments: vec![ColorBlendAttachmentState {
                blend: Some(AttachmentBlend::alpha()),
                color_write_mask: ColorComponents::all(),
                color_write_enable: StateMode::Fixed(true),
            }],
            ..Default::default()
        })
        .render_pass(Subpass::from(render_pass.clone(), 0).unroll()?)
        .build(device.clone())?;

    Ok(pipeline)
}
