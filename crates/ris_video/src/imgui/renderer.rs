use std::sync::Arc;

use imgui::Context;
use imgui::FontAtlas;
use imgui::TextureId;
use imgui::Textures;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::device::Device;
use vulkano::device::Queue;
use vulkano::format::Format;
use vulkano::image::ImageDimensions;
use vulkano::image::ImageViewAbstract;
use vulkano::image::ImmutableImage;
use vulkano::image::MipmapsCount;
use vulkano::image::view::ImageView;
use vulkano::image::view::ImageViewCreateInfo;
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
use vulkano::sampler::Sampler;
use vulkano::sampler::SamplerCreateInfo;

use ris_asset::loader::scenes_loader::Scenes;
use ris_error::RisResult;

use crate::vulkan::allocators::Allocators;
use crate::vulkan::renderer::Renderer;
use crate::vulkan::shader;

#[derive(Default, Debug, Clone)]
#[repr(C)]
struct ImguiVertex {
    pub pos: [f32; 2],
    pub uv : [f32; 2],
    pub col: u32, // [u8; 4]
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

pub type Texture = (Arc<ImageView<ImmutableImage>>, Arc<Sampler>);

pub struct ImguiRenderer {
    render_pass: Arc<RenderPass>,
    pipeline: Arc<GraphicsPipeline>,
    font_texture: Texture,
    textures: Textures<Texture>,
    allocators: Allocators,
}

impl ImguiRenderer {
    #[cfg(debug_assertions)]
    pub fn init(
        scenes: Scenes,
        renderer: &Renderer,
        context: &mut Context,
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

        let allocators = Allocators::new(device.clone());

        let textures = Textures::new();
        let font_texture = Self::upload_font_texture(
            context.fonts(),
            device.clone(),
            queue.clone(),
            &allocators,
        )?;

        context.set_renderer_name(Some(String::from("ris_engine vulkan renderer")));

        ris_log::error!("remove me");

        Ok(Some(Self{
            render_pass,
            pipeline,
            font_texture,
            textures,
            allocators,
        }))
    }

    #[cfg(not(debug_assertions))]
    pub fn init(scenes: Scenes) -> RisResult<Option<Self>> {
        Ok(None)
    }

    pub fn draw(&mut self) {

    }

    pub fn textures(&mut self) -> &mut Textures<Texture> {
        &mut self.textures
    }

    pub fn upload_font_texture(
        font_atlas: &mut FontAtlas,
        device: Arc<Device>,
        command_buffer_builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        allocators: &Allocators,
    ) -> RisResult<Texture> {
        let texture = font_atlas.build_rgba32_texture();

        let image = ris_error::unroll!(
            ImmutableImage::from_iter(
                &allocators.memory,
                texture.data.iter().cloned(),
                ImageDimensions::Dim2d{
                    width: texture.width,
                    height: texture.height,
                    array_layers: 1,
                },
                MipmapsCount::One,
                Format::R8G8B8A8_SRGB,
                command_buffer_builder,
            ),
            "",
        )?;

        let image_view = ris_error::unroll!(
            ImageView::new(
                image,
                ImageViewCreateInfo{
                    ..Default::default()
                },
            ),
            "failed to create image view",
        )?;

        let sampler = ris_error::unroll!(
            Sampler::new(
                device.clone(),
                SamplerCreateInfo::simple_repeat_linear()
            ),
            "failed to create sampler",
        )?;

        font_atlas.tex_id = TextureId::from(usize::MAX);
        Ok((image_view, sampler))
    }

    fn lookup_texture(&self, texture_id: TextureId) -> RisResult<&Texture>{
        if texture_id.id() == usize::MAX {
            Ok(&self.font_texture)
        } else if let Some(texture) = self.textures.get(texture_id) {
            Ok(texture)
        } else {
            ris_error::new_result!("bad texture: {:?}", texture_id)
        }
    }
}
