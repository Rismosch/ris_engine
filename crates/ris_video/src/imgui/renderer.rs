use std::sync::Arc;

use imgui::internal::RawWrapper;
use imgui::Context;
use imgui::DrawCmd;
use imgui::DrawCmdParams;
use imgui::DrawData;
use imgui::DrawVert;
use imgui::FontAtlas;
use imgui::TextureId;
use imgui::Textures;
use vulkano::buffer::Buffer;
use vulkano::buffer::BufferCreateInfo;
use vulkano::buffer::BufferUsage;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::CommandBufferUsage;
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::command_buffer::PrimaryCommandBufferAbstract;
use vulkano::command_buffer::RenderPassBeginInfo;
use vulkano::command_buffer::SubpassContents;
use vulkano::descriptor_set::persistent::PersistentDescriptorSet;
use vulkano::descriptor_set::WriteDescriptorSet;
use vulkano::device::Device;
use vulkano::device::Queue;
use vulkano::format::Format;
use vulkano::image::traits::ImageAccess;
use vulkano::image::view::ImageView;
use vulkano::image::view::ImageViewCreateInfo;
use vulkano::image::ImageDimensions;
use vulkano::image::ImmutableImage;
use vulkano::image::MipmapsCount;
use vulkano::memory::allocator::AllocationCreateInfo;
use vulkano::memory::allocator::MemoryUsage;
use vulkano::pipeline::graphics::viewport::Scissor;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::Pipeline;
use vulkano::render_pass::Framebuffer;
use vulkano::render_pass::FramebufferCreateInfo;
use vulkano::render_pass::RenderPass;
use vulkano::sampler::Sampler;
use vulkano::sampler::SamplerCreateInfo;
use vulkano::sync::future::GpuFuture;

use ris_asset::loader::scenes_loader::Scenes;
use ris_error::RisResult;
use ris_math::matrix4x4::Matrix4x4;

use crate::imgui::gpu_objects::ImguiVertex;
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
    pub fn init(renderer: &Renderer, scenes: &Scenes, context: &mut Context) -> RisResult<Self> {
        let device = renderer.device.clone();
        let queue = renderer.queue.clone();
        let swapchain = renderer.swapchain.clone();
        let allocators = &renderer.allocators;

        let vs_future = shader::load_async(device.clone(), scenes.imgui_vs.clone());
        let fs_future = shader::load_async(device.clone(), scenes.imgui_fs.clone());

        let vs = vs_future.wait()?;
        let fs = fs_future.wait()?;

        let render_pass =
            super::render_pass::create_render_pass(device.clone(), swapchain.clone())?;

        let pipeline = super::pipeline::create_pipeline(
            device.clone(),
            vs.clone(),
            fs.clone(),
            render_pass.clone(),
        )?;

        let textures = Textures::new();
        let font_texture =
            Self::upload_font_texture(context.fonts(), device.clone(), queue.clone(), allocators)?;

        context.set_renderer_name(Some(String::from("ris_engine vulkan renderer")));

        Ok(Self {
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

    pub fn draw<I>(
        &mut self,
        target: Arc<ImageView<I>>,
        command_buffer_builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        allocators: &Allocators,
        data: &DrawData,
    ) -> RisResult<()>
    where
        I: ImageAccess + std::fmt::Debug + 'static,
    {
        let fb_width = data.display_size[0] * data.framebuffer_scale[0];
        let fb_height = data.display_size[1] * data.framebuffer_scale[1];
        if fb_width <= 0.0 || fb_height <= 0.0 {
            return Ok(());
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

        let dimensions = match target.image().dimensions() {
            ImageDimensions::Dim2d { width, height, .. } => [width, height],
            dimensions => return ris_error::new_result!("bad image dimensions: {:?}", dimensions),
        };

        let viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [dimensions[0] as f32, dimensions[1] as f32],
            depth_range: 0.0..1.0,
        };

        let clip_off = data.display_pos;
        let clip_scale = data.framebuffer_scale;

        let layout = self.pipeline.layout().clone();

        let framebuffer = ris_error::unroll!(
            Framebuffer::new(
                self.render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![target],
                    ..Default::default()
                }
            ),
            "failed to create frame buffer",
        )?;

        ris_error::unroll!(
            command_buffer_builder.begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![None],
                    ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
                },
                SubpassContents::Inline,
            ),
            "failed to begin render pass",
        )?;

        for draw_list in data.draw_lists() {
            let vertices = draw_list
                .vtx_buffer()
                .iter()
                .map(|&v| unsafe { std::mem::transmute::<DrawVert, ImguiVertex>(v) });

            let indices = draw_list.idx_buffer().iter().cloned();

            let vertex_buffer = ris_error::unroll!(
                Buffer::from_iter(
                    &allocators.memory,
                    BufferCreateInfo {
                        usage: BufferUsage::VERTEX_BUFFER,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        usage: MemoryUsage::Upload,
                        ..Default::default()
                    },
                    vertices,
                ),
                "failed to create vertex buffer",
            )?;

            let index_buffer = ris_error::unroll!(
                Buffer::from_iter(
                    &allocators.memory,
                    BufferCreateInfo {
                        usage: BufferUsage::INDEX_BUFFER,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        usage: MemoryUsage::Upload,
                        ..Default::default()
                    },
                    indices,
                ),
                "failed to create index buffer",
            )?;

            for draw_cmd in draw_list.commands() {
                match draw_cmd {
                    DrawCmd::Elements {
                        count,
                        cmd_params:
                            DrawCmdParams {
                                clip_rect,
                                texture_id,
                                vtx_offset,
                                idx_offset,
                            },
                    } => {
                        let clip_rect = [
                            (clip_rect[0] - clip_off[0]) * clip_scale[0],
                            (clip_rect[1] - clip_off[1]) * clip_scale[1],
                            (clip_rect[2] - clip_off[0]) * clip_scale[0],
                            (clip_rect[3] - clip_off[1]) * clip_scale[1],
                        ];

                        if clip_rect[0] < fb_width
                            && clip_rect[1] < fb_height
                            && clip_rect[2] >= 0.0
                            && clip_rect[3] >= 0.0
                        {
                            let scissor = Scissor {
                                origin: [
                                    f32::max(0.0, clip_rect[0]).floor() as u32,
                                    f32::max(0.0, clip_rect[1]).floor() as u32,
                                ],
                                dimensions: [
                                    (clip_rect[2] - clip_rect[0]).abs().ceil() as u32,
                                    (clip_rect[3] - clip_rect[1]).abs().ceil() as u32,
                                ],
                            };

                            let texture = self.lookup_texture(texture_id)?;

                            let descriptor_set_layout = ris_error::unroll_option!(
                                layout.set_layouts().first(),
                                "failed to get descriptor set layout"
                            )?;

                            let descriptor_set = ris_error::unroll!(
                                PersistentDescriptorSet::new(
                                    &allocators.descriptor_set,
                                    descriptor_set_layout.clone(),
                                    [WriteDescriptorSet::image_view_sampler(
                                        0,
                                        texture.0.clone(),
                                        texture.1.clone(),
                                    )],
                                ),
                                "failed to create persistent descriptor set",
                            )?;

                            command_buffer_builder
                                .bind_pipeline_graphics(self.pipeline.clone())
                                .set_viewport(0, [viewport.clone()])
                                .set_scissor(0, [scissor])
                                .bind_vertex_buffers(0, vertex_buffer.clone())
                                .bind_index_buffer(index_buffer.clone())
                                .bind_descriptor_sets(
                                    vulkano::pipeline::PipelineBindPoint::Graphics,
                                    layout.clone(),
                                    0,
                                    descriptor_set.clone(),
                                )
                                .push_constants(layout.clone(), 0, pc)
                                .draw_indexed(
                                    count as u32,
                                    1,
                                    idx_offset as u32,
                                    vtx_offset as i32,
                                    0,
                                )
                                .map_err(|e| ris_error::new!("failed to draw: {}", e))?;
                        }
                    }
                    DrawCmd::ResetRenderState => (),
                    DrawCmd::RawCallback { callback, raw_cmd } => unsafe {
                        callback(draw_list.raw(), raw_cmd)
                    },
                }
            }
        }

        ris_error::unroll!(
            command_buffer_builder.end_render_pass(),
            "failed to end render pass",
        )?;

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
                ImageDimensions::Dim2d {
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
            ImageView::new(image, image_view_create_info,),
            "failed to create image view",
        )?;

        let sampler = ris_error::unroll!(
            Sampler::new(device.clone(), SamplerCreateInfo::simple_repeat_linear()),
            "failed to create sampler",
        )?;

        let primary = ris_error::unroll!(
            command_buffer_builder.build(),
            "failed to build command buffer",
        )?;

        let future =
            ris_error::unroll!(primary.execute(queue), "failed to execute command buffer",)?;

        let fence = ris_error::unroll!(
            future.then_signal_fence_and_flush(),
            "failed to signal fence and flush",
        )?;

        ris_error::unroll!(fence.wait(None), "failed to wait on fence",)?;

        font_atlas.tex_id = TextureId::from(usize::MAX);
        ris_log::debug!("imgui renderer uploaded font texture!");
        Ok((image_view, sampler))
    }

    fn lookup_texture(&self, texture_id: TextureId) -> RisResult<&Texture> {
        if texture_id.id() == usize::MAX {
            Ok(&self.font_texture)
        } else if let Some(texture) = self.textures.get(texture_id) {
            Ok(texture)
        } else {
            ris_error::new_result!("bad texture: {:?}", texture_id)
        }
    }
}
