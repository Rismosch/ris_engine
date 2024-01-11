use std::sync::Arc;

use vulkano::device::Device;
use vulkano::device::Queue;
use vulkano::format::Format;
use vulkano::pipeline::graphics::color_blend::AttachmentBlend;
use vulkano::pipeline::graphics::color_blend::ColorBlendAttachmentState;
use vulkano::pipeline::graphics::color_blend::ColorBlendState;
use vulkano::pipeline::graphics::color_blend::ColorComponents;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::vertex_input::VertexInputAttributeDescription;
use vulkano::pipeline::graphics::vertex_input::VertexInputBindingDescription;
use vulkano::pipeline::graphics::vertex_input::VertexInputRate;
use vulkano::pipeline::graphics::vertex_input::VertexInputState;
use vulkano::pipeline::graphics::viewport::ViewportState;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::StateMode;
use vulkano::render_pass::RenderPass;
use vulkano::render_pass::Subpass;

use ris_asset::loader::scenes_loader::Scenes;
use ris_error::RisResult;

use crate::vulkan::renderer::Renderer;
use crate::vulkan::shader;

#[derive(Default, Debug, Clone)]
#[repr(C)]
struct ImguiVertex {
    pub pos: [f32; 2],
    pub uv : [f32; 2],
    pub col: u32,
    // pub col: [u8; 4],
}

impl ImguiVertex {
    pub fn input_state() -> VertexInputState {
        let bindings = [(
            0u32,
            VertexInputBindingDescription {
                stride: 20,
                input_rate: VertexInputRate::Vertex,
            },
        )];

        let attributes = [
            (
                0,
                VertexInputAttributeDescription {
                    binding: 0,
                    format: Format::R32G32_SFLOAT,
                    offset: 0,
                },
            ),
            (
                1,
                VertexInputAttributeDescription {
                    binding: 0,
                    format: Format::R32G32_SFLOAT,
                    offset: 8,
                },
            ),
            (
                2,
                VertexInputAttributeDescription {
                    binding: 0,
                    format: Format::R32_UINT,
                    offset: 16,
                },
            ),
        ];

        VertexInputState::new()
            .bindings(bindings)
            .attributes(attributes)
    }
}

pub struct ImguiRenderer {

}

impl ImguiRenderer {
    #[cfg(debug_assertions)]
    pub fn init(
        scenes: Scenes,
        renderer: &Renderer,
    ) -> RisResult<Option<Self>> {
        let device = renderer.device();
        let queue = renderer.queue();
        let format = renderer.swapchain().image_format();
        let viewport = renderer.viewport();

        let vs_future = shader::load_async(device.clone(), scenes.imgui_vs.clone());
        let fs_future = shader::load_async(device.clone(), scenes.imgui_fs.clone());

        let vs = vs_future.wait()?;
        let fs = fs_future.wait()?;

        let render_pass = ris_error::unroll!(
            vulkano::single_pass_renderpass!(
                device.clone(),
                attachments: {
                    color: {
                        load: Load,
                        store: Store,
                        format: format,
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {}
                }
            ),
            "failed to create render pass for imgui"
        )?;

        let pipeline = ris_error::unroll!(
            GraphicsPipeline::start()
                .vertex_input_state(ImguiVertex::input_state())
                .vertex_shader(
                    ris_error::unroll_option!(
                        vs.clone().entry_point("main"),
                        "failed to locate vertex entry point",
                    )?,
                    (),
                )
                .input_assembly_state(InputAssemblyState::new())
                .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([
                    viewport.clone()
                ]))
                .fragment_shader(
                    ris_error::unroll_option!(
                        fs.clone().entry_point("main"),
                        "failed to locate fragment entry point",
                    )?,
                    (),
                )
                .color_blend_state(ColorBlendState {
                    attachments: vec![ColorBlendAttachmentState {
                        blend: Some(AttachmentBlend::alpha()),
                        color_write_mask: ColorComponents::all(),
                        color_write_enable: StateMode::Fixed(true),
                    }],
                    ..Default::default()
                })
                .render_pass(ris_error::unroll_option!(
                        Subpass::from(render_pass.clone(), 0),
                        "failed to create render subpass",
                )?)
                .build(device.clone()),
            "failed to build graphics pipeline",
        )?;

        ris_log::error!("reached end of imgui renderer init");

        Ok(Some(Self{}))
    }

    #[cfg(not(debug_assertions))]
    pub fn init(scenes: Scenes) -> RisResult<Option<Self>> {
        Ok(None)
    }
}
