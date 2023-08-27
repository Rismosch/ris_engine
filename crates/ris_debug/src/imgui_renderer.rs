use std::sync::Arc;
use std::time::Instant;

use imgui::BackendFlags;
use imgui::ConfigFlags;
use imgui::Context;
use imgui::Io;
use imgui::MouseCursor;
use imgui::Textures;
use imgui::TextureId;
use imgui::Ui;
use sdl2::event::Event;
use sdl2::keyboard::Mod;
use sdl2::keyboard::Scancode;
use sdl2::mouse::Cursor;
use sdl2::mouse::SystemCursor;
use sdl2::video::Window;
use vulkano::buffer::BufferContents;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::CommandBufferUsage;
use vulkano::image::ImmutableImage;
use vulkano::image::ImageDimensions;
use vulkano::image::view::ImageView;
use vulkano::image::view::ImageViewAbstract;
use vulkano::image::view::ImageViewCreateInfo;
use vulkano::image::traits::ImageAccess;
use vulkano::render_pass::Framebuffer;
use vulkano::render_pass::FramebufferCreateInfo;
use vulkano::render_pass::Subpass;
use vulkano::render_pass::RenderPass;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::Pipeline;
use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::pipeline::graphics::viewport::ViewportState;
use vulkano::pipeline::graphics::color_blend::ColorBlendState;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::sampler::Sampler;
use vulkano::sampler::SamplerCreateInfo;

use ris_video::video::Video;

#[derive(BufferContents, Vertex, Default)]
#[repr(C)]
struct ImguiVertex {
    #[format(R32G32_SFLOAT)]
    pub pos: [f32; 2],
    #[format(R32G32_SFLOAT)]
    pub uv: [f32; 2],
    #[format(R32_UINT)]
    pub col: u32,
}

pub type Texture = (Arc<dyn ImageViewAbstract + Send + Sync>, Arc<Sampler>);

pub struct ImguiRenderer {
    render_pass: Arc<RenderPass>,
    pipeline: Arc<GraphicsPipeline>,
    font_textures: Texture,
    textures: Textures<Texture>,
}

impl ImguiRenderer {
    pub fn new(context: &mut Context, video: &Video) -> Result<Self, String> {
        let mut format = vulkano::format::Format::R8G8B8A8_SRGB;
        
        let vertex_shader = crate::imgui_shaders::vertex_shader(video.device())?;
        let fragment_shader = crate::imgui_shaders::fragment_shader(video.device())?;

        let render_pass = vulkano::single_pass_renderpass!(
                video.device().clone(),
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
            )
            .map_err(|e| format!("failed to create imgui render pass: {}", e))?;
        
        let pipeline = GraphicsPipeline::start()
            .vertex_input_state(ImguiVertex::per_vertex())
            .vertex_shader(
                vertex_shader
                    .clone()
                    .entry_point("main")
                    .ok_or("failed to locate imgui vertex entry point")?,
                (),
            )
            .input_assembly_state(InputAssemblyState::new())
            .viewport_state(ViewportState::viewport_dynamic_scissor_dynamic(1))
            .fragment_shader(
                fragment_shader
                    .clone()
                    .entry_point("main")
                    .ok_or("failed to locate imgui fragment entry point")?,
                (),
            )
            .color_blend_state(ColorBlendState::default().blend_alpha())
            .render_pass(
                Subpass::from(render_pass.clone(), 0).ok_or("failed to create imgui subpass")?,
            )
            .build(video.device().clone()).map_err(|e| format!("failed to build imgui render pipeline: {}", e))?;

        let textures = Textures::new();

        let texture = context.fonts().build_rgba32_texture();

        let mut builder = AutoCommandBufferBuilder::primary(
            &video.allocators().command_buffer,
            video.queue().queue_family_index(),
            CommandBufferUsage::MultipleSubmit,
        )
        .map_err(|e| format!("failed to create auto command buffer builder: {}", e))?;

        let image = ImmutableImage::from_iter(
            &video.allocators().memory,
            texture.data.iter().cloned(),
            ImageDimensions::Dim2d{
                width: texture.width,
                height: texture.height,
                array_layers: 1,
            },
            vulkano::image::MipmapsCount::One,
            format,
            &mut builder
        )
        .map_err(|e| format!("failed to create imgui image: {}", e))?;

        let sampler = Sampler::new(
            video.device().clone(),
            SamplerCreateInfo::simple_repeat_linear()
        )
        .map_err(|e| format!("failed to create imgui sampler: {}", e))?;

        let image_view = ImageView::new(
            image.clone(),
            ImageViewCreateInfo{
                format: Some(format),
                subresource_range: image.clone().subresource_range(),
                ..Default::default()
            }
        )
        .map_err(|e| format!("failed to create imgui image view: {}", e))?;

        context.fonts().tex_id = TextureId::from(usize::MAX);

        //let vrt_buffer_pool = CpuBufferPool::new(device.clone(), BufferUsage::vertex_buffer_transfer_destination());
        //let idx_buffer_pool = CpuBufferPool::new(device.clone(), BufferUsage::index_buffer_transfer_destination());
        
        Ok(Self{
            render_pass,
            pipeline,
            font_textures: (image_view, sampler),
            textures,
        })
    }

    pub fn render(&mut self, context: &mut Context, video: &Video) -> Result<(), String> {
        let draw_data = context.render();

        let fb_width = draw_data.display_size[0] * draw_data.framebuffer_scale[0];
        let fb_height = draw_data.display_size[1] * draw_data.framebuffer_scale[1];
        if fb_width < 0.0 || fb_height < 0.0 {
            return Ok(());
        }
        let left = draw_data.display_pos[0];
        let right = left + draw_data.display_size[0];
        let top = draw_data.display_pos[1];
        let bottom = top + draw_data.display_size[1];

        //let pc = shader::vs::ty::VertPC {
        //    matrix : [
        //        [(2.0 / (right - left)), 0.0, 0.0, 0.0],
        //        [0.0, (2.0 / (bottom - top)), 0.0, 0.0],
        //        [0.0, 0.0, -1.0, 0.0],
        //        [
        //            (right + left) / (left - right),
        //            (top + bottom) / (top - bottom),
        //            0.0,
        //            1.0,
        //        ],
        //    ]
        //};
        
        let target = self.font_textures.0.clone();
        
        let dims = match target.image().dimensions() {
            ImageDimensions::Dim2d { width, height, ..} => {[width, height]},
            d => {return Err(format!("bad image dimensions: {:?}", d))},
        };

        //let mut dynamic_state = DynamicState::default();
        //dynamic_state.viewports = Some(vec![
        //    Viewport {
        //        origin: [0.0, 0.0],
        //        dimensions: [dims[0] as f32, dims[1] as f32],
        //        depth_range: 0.0 .. 1.0,
        //    }
        //]);
        //dynamic_state.scissors = Some(vec![
        //    Scissor::default()
        //]);

        let clip_off = draw_data.display_pos;
        let clip_scale = draw_data.framebuffer_scale;

        let layout = self.pipeline.layout();

        let framebuffer = Framebuffer::new(
            self.render_pass.clone(),
            FramebufferCreateInfo
            {
                attachments: vec![target],
                ..Default::default()
            }
        )
        .map_err(|e| format!("failed to create imgui framebuffer: {}", e))?;

        Ok(())
    }
}
