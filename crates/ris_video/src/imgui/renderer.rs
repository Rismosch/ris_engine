use std::sync::Arc;

use imgui::Context;
use imgui::FontAtlas;
use imgui::TextureId;
use imgui::Textures;
use vulkano::buffer::Buffer;
use vulkano::buffer::BufferCreateInfo;
use vulkano::buffer::BufferError;
use vulkano::buffer::BufferUsage;
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
use vulkano::memory::allocator::AllocationCreateInfo;
use vulkano::memory::allocator::MemoryUsage;
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

pub type Texture = (Arc<ImageView<ImmutableImage>>, Arc<Sampler>);

pub struct ImguiRenderer {
    render_pass: Arc<RenderPass>,
    pipeline: Arc<GraphicsPipeline>,
    font_texture: Texture,
    textures: Textures<Texture>,
}

impl ImguiRenderer {
    #[cfg(debug_assertions)]
    pub fn init(
        scenes: Scenes,
        renderer: &Renderer,
        context: &mut Context,
    ) -> RisResult<Option<Self>> {
        let device = renderer.device.clone();
        let queue = renderer.queue.clone();
        let format = renderer.swapchain.image_format();
        let viewport = &renderer.viewport;
        let framebuffers = &renderer.framebuffers;
        let swapchain = renderer.swapchain.clone();
        let allocators = &renderer.allocators;

        let vs_future = shader::load_async(device.clone(), scenes.imgui_vs.clone());
        let fs_future = shader::load_async(device.clone(), scenes.imgui_fs.clone());

        let vs = vs_future.wait()?;
        let fs = fs_future.wait()?;

        let render_pass = super::render_pass::create_render_pass(
            device.clone(),
            swapchain.clone(),
        )?;

        let pipeline = super::pipeline::create_pipeline(
            device.clone(),
            vs.clone(),
            fs.clone(),
            render_pass.clone(),
            &viewport,
        )?;

        let textures = Textures::new();
        let font_texture = Self::upload_font_texture(
            context.fonts(),
            device.clone(),
            command_buffer_builder,
            &allocators,
        )?;

        context.set_renderer_name(Some(String::from("ris_engine vulkan renderer")));

        ris_log::error!("remove me");

        Ok(Some(Self{
            render_pass,
            pipeline,
            font_texture,
            textures,
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

        let source = Buffer::from_iter(
            &allocators.memory,
            vulkano::buffer::BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                usage: MemoryUsage::Upload,
                ..Default::default()
            },
            texture.data.iter().cloned(),
        )
        .map_err(|err| match err {
            BufferError::AllocError(err) => ris_error::new!("buffer error alloc error: {}", err),
            // this is unreachable according to: doc/vulkano/image/immutable.rs.html#209
            _ => unreachable!(),
        })?;

        let image = ris_error::unroll!(
            ImmutableImage::from_buffer(
                &allocators.memory,
                source,
                ImageDimensions::Dim2d{
                    width: texture.width,
                    height: texture.height,
                    array_layers: 1,
                },
                MipmapsCount::One,
                Format::R8G8B8A8_SRGB,
                command_buffer_builder,
            ),
            "failed to create image",
        )?;

        //let image = ris_error::unroll!(
        //    ImmutableImage::from_iter(
        //        &allocators.memory,
        //        texture.data.iter().cloned(),
        //        ImageDimensions::Dim2d{
        //            width: texture.width,
        //            height: texture.height,
        //            array_layers: 1,
        //        },
        //        MipmapsCount::One,
        //        Format::R8G8B8A8_SRGB,
        //        command_buffer_builder,
        //    ),
        //    "",
        //)?;

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
