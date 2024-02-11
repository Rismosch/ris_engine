use std::sync::Arc;

use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::CommandBufferUsage;
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::command_buffer::RenderPassBeginInfo;
use vulkano::command_buffer::SubpassContents;
use vulkano::device::Queue;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::Pipeline;
use vulkano::pipeline::PipelineBindPoint;
use vulkano::render_pass::Framebuffer;

use ris_error::RisResult;

use crate::vulkan::allocators::Allocators;
use crate::vulkan::buffers::Buffers;

pub fn create_command_buffers(
    allocators: &Allocators,
    queue: Arc<Queue>,
    pipeline: Arc<GraphicsPipeline>,
    framebuffers: &[Arc<Framebuffer>],
    buffers: &Buffers,
) -> RisResult<Vec<Arc<PrimaryAutoCommandBuffer>>> {
    let mut command_buffers = Vec::new();

    for (i, framebuffer) in framebuffers.iter().enumerate() {
        let pipeline_layout = pipeline.layout();

        let mut builder = AutoCommandBufferBuilder::primary(
            &allocators.command_buffer,
            queue.queue_family_index(),
            CommandBufferUsage::MultipleSubmit,
        )?;

        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some([0.5, 0., 0.5, 0.].into()), Some((0f32, 0u32).into())],
                    ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
                },
                SubpassContents::Inline,
            )
            .map_err(|e| ris_error::new!("failed to begin render pass: {}", e))?
            .bind_pipeline_graphics(pipeline.clone())
            .bind_vertex_buffers(0, buffers.vertex.clone())
            .bind_index_buffer(buffers.index.clone())
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                pipeline_layout.clone(),
                0,
                buffers.uniforms[i].1.clone(),
            )
            .draw_indexed(buffers.index.len() as u32, 1, 0, 0, 0)
            .map_err(|e| ris_error::new!("failed to draw: {}", e))?
            .end_render_pass()
            .map_err(|e| ris_error::new!("failed to end render pass: {}", e))?;

        let command_buffer = Arc::new(
            builder
                .build()
                .map_err(|e| ris_error::new!("failed to build command buffer: {}", e))?,
        );

        command_buffers.push(command_buffer);
    }

    Ok(command_buffers)
}
