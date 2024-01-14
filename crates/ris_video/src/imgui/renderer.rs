use std::sync::Arc;

use imgui::Context;
use imgui::DrawData;
use imgui::FontAtlas;
use imgui::TextureId;
use imgui::Textures;
use vulkano::buffer::Buffer;
use vulkano::buffer::BufferCreateInfo;
use vulkano::buffer::BufferError;
use vulkano::buffer::BufferUsage;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::CommandBufferExecFuture;
use vulkano::command_buffer::CommandBufferUsage;
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::command_buffer::PrimaryCommandBufferAbstract;
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
use vulkano::sync::future::GpuFuture;

use ris_asset::loader::scenes_loader::Scenes;
use ris_error::RisResult;
use ris_math::matrix4x4::Matrix4x4;

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
        renderer: &Renderer,
        scenes: &Scenes,
        context: &mut Context,
    ) -> RisResult<Self> {
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
            queue.clone(),
            &allocators,
        )?;

        context.set_renderer_name(Some(String::from("ris_engine vulkan renderer")));

        Ok(Self{
            render_pass,
            pipeline,
            font_texture,
            textures,
        })
    }

    #[cfg(not(debug_assertions))]
    pub fn init(scenes: Scenes) -> RisResult<Option<Self>> {
        Ok(None)
    }

    pub fn draw(&mut self, context: &mut Context) -> RisResult<()> {
        let data = context.render();

        let fb_width = data.display_size[0] * data.framebuffer_scale[0];
        let fb_height = data.display_size[1] * data.framebuffer_scale[1];
        if fb_width <= 0.0 || fb_height <= 0.0 {
            return Ok(())
        }

        let left = data.display_pos[0];
        let right = data.display_pos[0] + data.display_size[0];
        let top = data.display_pos[1];
        let bottom = data.display_pos[1] + data.display_size[1];

        let pc = Matrix4x4 {
            m00: 2.0 / (right - left),
            m01: 0.0,
            m02: 0.0,
            m03: 0.0,
            m10: 0.0,
            m11: 2.0 / (bottom - top),
            m12: 0.0,
            m13: 0.0,
            m20: 0.0,
            m21: 0.0,
            m22: -1.0,
            m23: 0.0,
            m30: (right + left) / (left - right),
            m31: (top + bottom) / (top - bottom),
            m32: 0.0,
            m33: 1.0,
        };

        //let dims = match ta

        Ok(())
    }

    pub fn reload_font_texture(
        &mut self,
        context: &mut Context,
        device: Arc<Device>,
        queue: Arc<Queue>,
        allocators: &Allocators,
    ) -> RisResult<()> {
        self.font_texture = Self::upload_font_texture(context.fonts(), device, queue, allocators)?;
        Ok(())
    }

    pub fn textures(&mut self) -> &mut Textures<Texture> {
        &mut self.textures
    }

    fn upload_font_texture(
        font_atlas: &mut FontAtlas,
        device: Arc<Device>,
        queue: Arc<Queue>,
        allocators: &Allocators,
    ) -> RisResult<Texture> {
        ris_log::debug!("imgui renderer uploading font texture...");

        let texture = font_atlas.build_rgba32_texture();

        let mut command_buffer_builder = ris_error::unroll!(
            AutoCommandBufferBuilder::primary(
                &allocators.command_buffer,
                queue.queue_family_index(),
                CommandBufferUsage::OneTimeSubmit,
            ),
            "failed to build command buffer",
        )?;

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
                &mut command_buffer_builder,
            ),
            "failed to create image",
        )?;

        let image_view_create_info = ImageViewCreateInfo::from_image(&image);
        let image_view = ris_error::unroll!(
            ImageView::new(
                image,
                image_view_create_info,
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

        let primary = ris_error::unroll!(
            command_buffer_builder.build(),
            "failed to build command buffer",
        )?;

        let future = ris_error::unroll!(
            primary.execute(queue),
            "failed to execute command buffer",
        )?;


        let fence = ris_error::unroll!(
            future.then_signal_fence_and_flush(),
            "failed to signal fence and flush",
        )?;

        ris_error::unroll!(
            fence.wait(None),
            "failed to wait on fence",
        )?;

        font_atlas.tex_id = TextureId::from(usize::MAX);
        ris_log::debug!("imgui renderer uploaded font texture!");
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
