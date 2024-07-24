use std::ptr;

use ash::vk;

use ris_asset::RisGodAsset;
use ris_data::god_state::GodState;
use ris_error::RisResult;
use ris_math::color::Rgb;
use ris_math::matrix::Mat4;
use ris_math::vector::Vec3;
use ris_math::vector::Vec2;

use crate::vulkan::core::VulkanCore;
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

#[repr(C)]
pub struct Vertex {
    pub pos: Vec3,
    pub color: Rgb,
    pub uv: Vec2,
}

//impl Vertex {
//    pub fn get_binding_descriptions() -> [vk::VertexInputBindingDescription; 1] {
//        [vk::VertexInputBindingDescription {
//            binding: 0,
//            stride: std::mem::size_of::<Self>() as u32,
//            input_rate: vk::VertexInputRate::VERTEX,
//        }]
//    }
//
//    pub fn get_attribute_descriptions() -> [vk::VertexInputAttributeDescription; 3] {
//        [
//            vk::VertexInputAttributeDescription {
//                location: 0,
//                binding: 0,
//                format: vk::Format::R32G32B32_SFLOAT,
//                offset: std::mem::offset_of!(Self, pos) as u32,
//            },
//            vk::VertexInputAttributeDescription {
//                location: 1,
//                binding: 0,
//                format: vk::Format::R32G32B32_SFLOAT,
//                offset: std::mem::offset_of!(Self, color) as u32,
//            },
//            vk::VertexInputAttributeDescription {
//                location: 2,
//                binding: 0,
//                format: vk::Format::R32G32_SFLOAT,
//                offset: std::mem::offset_of!(Self, uv) as u32,
//            },
//        ]
//    }
//}

const VERTICES: [Vertex; 4 * 6] = [
    // pos x
    Vertex {
        pos: Vec3(0.5, -0.5, 0.5),
        color: Rgb(1.0, 0.0, 0.0),
        uv: Vec2(0.0, 0.0),
    },
    Vertex {
        pos: Vec3(0.5, 0.5, 0.5),
        color: Rgb(1.0, 0.0, 0.0),
        uv: Vec2(1.0, 0.0),
    },
    Vertex {
        pos: Vec3(0.5, 0.5, -0.5),
        color: Rgb(1.0, 0.0, 0.0),
        uv: Vec2(1.0, 1.0),
    },
    Vertex {
        pos: Vec3(0.5, -0.5, -0.5),
        color: Rgb(1.0, 0.0, 0.0),
        uv: Vec2(0.0, 1.0),
    },
    // pos y
    Vertex {
        pos: Vec3(0.5, 0.5, 0.5),
        color: Rgb(0.0, 1.0, 0.0),
        uv: Vec2(0.0, 0.0),
    },
    Vertex {
        pos: Vec3(-0.5, 0.5, 0.5),
        color: Rgb(0.0, 1.0, 0.0),
        uv: Vec2(1.0, 0.0),
    },
    Vertex {
        pos: Vec3(-0.5, 0.5, -0.5),
        color: Rgb(0.0, 1.0, 0.0),
        uv: Vec2(1.0, 1.0),
    },
    Vertex {
        pos: Vec3(0.5, 0.5, -0.5),
        color: Rgb(0.0, 1.0, 0.0),
        uv: Vec2(0.0, 1.0),
    },
    // pos z
    Vertex {
        pos: Vec3(-0.5, 0.5, 0.5),
        color: Rgb(0.0, 0.0, 1.0),
        uv: Vec2(0.0, 0.0),
    },
    Vertex {
        pos: Vec3(0.5, 0.5, 0.5),
        color: Rgb(0.0, 0.0, 1.0),
        uv: Vec2(1.0, 0.0),
    },
    Vertex {
        pos: Vec3(0.5, -0.5, 0.5),
        color: Rgb(0.0, 0.0, 1.0),
        uv: Vec2(1.0, 1.0),
    },
    Vertex {
        pos: Vec3(-0.5, -0.5, 0.5),
        color: Rgb(0.0, 0.0, 1.0),
        uv: Vec2(0.0, 1.0),
    },
    // neg x
    Vertex {
        pos: Vec3(-0.5, 0.5, 0.5),
        color: Rgb(0.0, 1.0, 1.0),
        uv: Vec2(0.0, 0.0),
    },
    Vertex {
        pos: Vec3(-0.5, -0.5, 0.5),
        color: Rgb(0.0, 1.0, 1.0),
        uv: Vec2(1.0, 0.0),
    },
    Vertex {
        pos: Vec3(-0.5, -0.5, -0.5),
        color: Rgb(0.0, 1.0, 1.0),
        uv: Vec2(1.0, 1.0),
    },
    Vertex {
        pos: Vec3(-0.5, 0.5, -0.5),
        color: Rgb(0.0, 1.0, 1.0),
        uv: Vec2(0.0, 1.0),
    },
    // neg y
    Vertex {
        pos: Vec3(-0.5, -0.5, 0.5),
        color: Rgb(1.0, 0.0, 1.0),
        uv: Vec2(0.0, 0.0),
    },
    Vertex {
        pos: Vec3(0.5, -0.5, 0.5),
        color: Rgb(1.0, 0.0, 1.0),
        uv: Vec2(1.0, 0.0),
    },
    Vertex {
        pos: Vec3(0.5, -0.5, -0.5),
        color: Rgb(1.0, 0.0, 1.0),
        uv: Vec2(1.0, 1.0),
    },
    Vertex {
        pos: Vec3(-0.5, -0.5, -0.5),
        color: Rgb(1.0, 0.0, 1.0),
        uv: Vec2(0.0, 1.0),
    },
    // neg z
    Vertex {
        pos: Vec3(-0.5, -0.5, -0.5),
        color: Rgb(1.0, 1.0, 0.0),
        uv: Vec2(0.0, 0.0),
    },
    Vertex {
        pos: Vec3(0.5, -0.5, -0.5),
        color: Rgb(1.0, 1.0, 0.0),
        uv: Vec2(1.0, 0.0),
    },
    Vertex {
        pos: Vec3(0.5, 0.5, -0.5),
        color: Rgb(1.0, 1.0, 0.0),
        uv: Vec2(1.0, 1.0),
    },
    Vertex {
        pos: Vec3(-0.5, 0.5, -0.5),
        color: Rgb(1.0, 1.0, 0.0),
        uv: Vec2(0.0, 1.0),
    },
];

const INDICES: [u32; 6 * 6] = [
    0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12, 16, 17, 18,
    18, 19, 16, 20, 21, 22, 22, 23, 20,
];

pub struct SceneRenderer {

}

impl SceneRenderer {
    pub fn free(&mut self, device: &ash::Device) {
        unsafe {

        }
    }

    pub fn init(
        core: &VulkanCore,
        god_asset: &RisGodAsset,
    ) -> RisResult<Self> {
        let VulkanCore {
            instance,
            suitable_device,
            device,
            graphics_queue,
            transient_command_pool,
            swapchain:
                Swapchain {
                    base:
                        BaseSwapchain {
                            format: swapchain_format,
                            ..
                        },
                    entries,
                    ..
                },
            ..
        } = core;

        // descriptor sets

        // shaders
        let vs_asset_future = ris_asset::load_async(god_asset.default_vert_spv.clone());
        let fs_asset_future = ris_asset::load_async(god_asset.default_frag_spv.clone());

        let vs_bytes = vs_asset_future.wait(None)??;
        let fs_bytes = fs_asset_future.wait(None)??;

        let vs_module = crate::shader::create_module(device, &vs_bytes)?;
        let fs_module = crate::shader::create_module(device, &fs_bytes)?;

        let shader_stages = [
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::PipelineShaderStageCreateFlags::empty(),
                module: vs_module,
                p_name: crate::shader::ENTRY.as_ptr(),
                p_specialization_info: ptr::null(),
                stage: vk::ShaderStageFlags::VERTEX,
            },
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::PipelineShaderStageCreateFlags::empty(),
                module: fs_module,
                p_name: crate::shader::ENTRY.as_ptr(),
                p_specialization_info: ptr::null(),
                stage: vk::ShaderStageFlags::FRAGMENT,
            },
        ];

        // pipeline
        
        // pipeline layout
        
        // render pass

        // pipeline creation

        unsafe { device.destroy_shader_module(vs_module, None) };
        unsafe { device.destroy_shader_module(fs_module, None) };

        // frames

        Ok(Self{

        })
    }

    //pub fn draw(
    //    &mut self,
    //    state: &GodState,
    //    base: &VulkanBase,
    //    image_index: usize,
    //    window_drawable_size: (u32, u32),
    //) -> RisResult<()> {
    //    let VulkanBase {
    //        instance,
    //        suitable_device,
    //        device,
    //        graphics_queue,
    //        transient_command_pool,
    //        swapchain:
    //            Swapchain {
    //                base:
    //                    BaseSwapchain {
    //                        extent: swapchain_extent,
    //                        ..
    //                    },
    //                entries,
    //                ..
    //            },
    //        ..
    //    } = base;

    //    let SwapchainEntry {
    //        command_buffer,
    //        //framebuffer,
    //        ..
    //    } = entries[image_index];

    //    // update uniform buffer
    //    //let (w, h) = (window_drawable_size.0 as f32, window_drawable_size.1 as f32);
    //    //state.camera.aspect_ratio = w / h;
    //    //let view = state.camera.view_matrix();
    //    //let proj = state.camera.projection_matrix();

    //    //let uniform_buffer_object = UniformBufferObject {
    //    //    model: Mat4::init(1.0),
    //    //    view,
    //    //    proj,
    //    //};

    //    //let ubo = [uniform_buffer_object];
    //    //unsafe { uniform_buffer_mapped.copy_from_nonoverlapping(ubo.as_ptr(), ubo.len()) };

    //    //// render pass
    //    //let clear_values = [
    //    //    vk::ClearValue {
    //    //        color: vk::ClearColorValue {
    //    //            float32: [0.0, 0.0, 0.0, 0.0],
    //    //        },
    //    //    },
    //    //    vk::ClearValue {
    //    //        depth_stencil: vk::ClearDepthStencilValue {
    //    //            depth: 1.0,
    //    //            stencil: 0,
    //    //        },
    //    //    },
    //    //];

    //    //let render_pass_begin_info = vk::RenderPassBeginInfo {
    //    //    s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
    //    //    p_next: ptr::null(),
    //    //    render_pass: graphics_pipeline.render_pass,
    //    //    framebuffer,
    //    //    render_area: vk::Rect2D {
    //    //        offset: vk::Offset2D { x: 0, y: 0 },
    //    //        extent: *swapchain_extent,
    //    //    },
    //    //    clear_value_count: clear_values.len() as u32,
    //    //    p_clear_values: clear_values.as_ptr(),
    //    //};

    //    //unsafe {
    //    //    device.cmd_begin_render_pass(
    //    //        command_buffer,
    //    //        &render_pass_begin_info,
    //    //        vk::SubpassContents::INLINE,
    //    //    )
    //    //};
    //    //unsafe {
    //    //    device.cmd_bind_pipeline(
    //    //        command_buffer,
    //    //        vk::PipelineBindPoint::GRAPHICS,
    //    //        graphics_pipeline.pipeline,
    //    //    )
    //    //};

    //    //let vertex_buffers = [vertex_buffer.buffer];
    //    //let offsets = [0_u64];
    //    //unsafe { device.cmd_bind_vertex_buffers(command_buffer, 0, &vertex_buffers, &offsets) };
    //    //unsafe {
    //    //    device.cmd_bind_index_buffer(
    //    //        command_buffer,
    //    //        index_buffer.buffer,
    //    //        0,
    //    //        vk::IndexType::UINT32,
    //    //    )
    //    //};
    //    //let descriptor_sets = [descriptor_set];
    //    //unsafe {
    //    //    device.cmd_bind_descriptor_sets(
    //    //        command_buffer,
    //    //        vk::PipelineBindPoint::GRAPHICS,
    //    //        graphics_pipeline.layout,
    //    //        0,
    //    //        &descriptor_sets,
    //    //        &[],
    //    //    )
    //    //};

    //    let index_count = crate::vulkan::INDICES.len() as u32;
    //    unsafe { device.cmd_draw_indexed(command_buffer, index_count, 1, 0, 0, 0) };
    //    unsafe { device.cmd_end_render_pass(command_buffer) };

    //    Ok(())
    //}
}
