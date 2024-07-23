use std::ptr;

use ash::vk;

use ris_data::god_state::GodState;
use ris_error::RisResult;
use ris_math::matrix::Mat4;
use ris_math::vector::Vec3;

use crate::vulkan::base::VulkanBase;
use crate::vulkan::swapchain::BaseSwapchain;
use crate::vulkan::swapchain::Swapchain;
use crate::vulkan::swapchain::SwapchainEntry;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct UniformBufferObject {
    pub model: Mat4,
    pub view: Mat4,
    pub proj: Mat4,
}

pub struct SceneRenderer {

}

impl SceneRenderer {
    //pub fn free() -> RisResult<()> {

    //}

    pub fn init() -> RisResult<Self> {
        panic!()
    }

    pub fn draw(
        &mut self,
        state: &GodState,
        base: &VulkanBase,
        image_index: usize,
        window_drawable_size: (u32, u32),
    ) -> RisResult<()> {
        let VulkanBase {
            instance,
            suitable_device,
            device,
            graphics_queue,
            transient_command_pool,
            swapchain:
                Swapchain {
                    base:
                        BaseSwapchain {
                            extent: swapchain_extent,
                            ..
                        },
                    entries,
                    graphics_pipeline,
                    ..
                },
            ..
        } = base;

        let SwapchainEntry {
            command_buffer,
            //framebuffer,
            ..
        } = entries[image_index];

        // update uniform buffer
        //let (w, h) = (window_drawable_size.0 as f32, window_drawable_size.1 as f32);
        //state.camera.aspect_ratio = w / h;
        //let view = state.camera.view_matrix();
        //let proj = state.camera.projection_matrix();

        //let uniform_buffer_object = UniformBufferObject {
        //    model: Mat4::init(1.0),
        //    view,
        //    proj,
        //};

        //let ubo = [uniform_buffer_object];
        //unsafe { uniform_buffer_mapped.copy_from_nonoverlapping(ubo.as_ptr(), ubo.len()) };

        //// render pass
        //let clear_values = [
        //    vk::ClearValue {
        //        color: vk::ClearColorValue {
        //            float32: [0.0, 0.0, 0.0, 0.0],
        //        },
        //    },
        //    vk::ClearValue {
        //        depth_stencil: vk::ClearDepthStencilValue {
        //            depth: 1.0,
        //            stencil: 0,
        //        },
        //    },
        //];

        //let render_pass_begin_info = vk::RenderPassBeginInfo {
        //    s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
        //    p_next: ptr::null(),
        //    render_pass: graphics_pipeline.render_pass,
        //    framebuffer,
        //    render_area: vk::Rect2D {
        //        offset: vk::Offset2D { x: 0, y: 0 },
        //        extent: *swapchain_extent,
        //    },
        //    clear_value_count: clear_values.len() as u32,
        //    p_clear_values: clear_values.as_ptr(),
        //};

        //unsafe {
        //    device.cmd_begin_render_pass(
        //        command_buffer,
        //        &render_pass_begin_info,
        //        vk::SubpassContents::INLINE,
        //    )
        //};
        //unsafe {
        //    device.cmd_bind_pipeline(
        //        command_buffer,
        //        vk::PipelineBindPoint::GRAPHICS,
        //        graphics_pipeline.pipeline,
        //    )
        //};

        //let vertex_buffers = [vertex_buffer.buffer];
        //let offsets = [0_u64];
        //unsafe { device.cmd_bind_vertex_buffers(command_buffer, 0, &vertex_buffers, &offsets) };
        //unsafe {
        //    device.cmd_bind_index_buffer(
        //        command_buffer,
        //        index_buffer.buffer,
        //        0,
        //        vk::IndexType::UINT32,
        //    )
        //};
        //let descriptor_sets = [descriptor_set];
        //unsafe {
        //    device.cmd_bind_descriptor_sets(
        //        command_buffer,
        //        vk::PipelineBindPoint::GRAPHICS,
        //        graphics_pipeline.layout,
        //        0,
        //        &descriptor_sets,
        //        &[],
        //    )
        //};

        let index_count = crate::vulkan::INDICES.len() as u32;
        unsafe { device.cmd_draw_indexed(command_buffer, index_count, 1, 0, 0, 0) };
        unsafe { device.cmd_end_render_pass(command_buffer) };

        Ok(())
    }
}
