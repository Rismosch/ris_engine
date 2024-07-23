use std::ffi::CString;
use std::ptr;

use ash::vk;

use ris_asset::AssetId;
use ris_debug::gizmo::GizmoShapeVertex;
use ris_debug::gizmo::GizmoTextVertex;
use ris_error::Extensions;
use ris_error::RisResult;
use ris_math::camera::Camera;
use ris_math::matrix::Mat4;

use crate::frames::IFrame;
use crate::frames::Frames;
use crate::gizmo::gizmo_mesh::ShapeMesh;
use crate::vulkan::base::VulkanBase;
use crate::vulkan::buffer::Buffer;
use crate::vulkan::swapchain::BaseSwapchain;
use crate::vulkan::swapchain::Swapchain;
use crate::vulkan::swapchain::SwapchainEntry;
use crate::vulkan::transient_command::TransientCommand;
use crate::vulkan::transient_command::TransientCommandSync;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct UniformBufferObject {
    pub view: Mat4,
    pub proj: Mat4,
}

struct GizmoShapeFrame {
    mesh: Option<ShapeMesh>,
    framebuffer: Option<vk::Framebuffer>,
    descriptor_buffer: Buffer,
    descriptor_mapped: *mut UniformBufferObject,
    descriptor_set: vk::DescriptorSet,
}

impl IFrame for GizmoShapeFrame {
    unsafe fn free(&mut self, device: &ash::Device) {
        if let Some(mut mesh) = self.mesh.take() {
            mesh.free(device);
        }

        if let Some(mut framebuffer) = self.framebuffer.take() {
            unsafe { device.destroy_framebuffer(framebuffer, None) };
        }

        self.descriptor_buffer.free(device);
    }
}


//pub struct Descriptor {
//    pub buffer: Buffer,
//    pub mapped: *mut UniformBufferObject,
//    pub set: vk::DescriptorSet,
//}

//impl IFrame for Descriptor {
//    unsafe fn free(&mut self, device: &ash::Device) {
//        self.buffer.free(device);
//    }
//}

pub struct GizmoRenderer {
    shape_renderer: ShapeRenderer,
    text_renderer: TextRenderer,
}

struct ShapeRenderer {
    pipeline: vk::Pipeline,
    pipeline_layout: vk::PipelineLayout,
    render_pass: vk::RenderPass,
    descriptor_set_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
    //descriptors: Frames<Descriptor>,
    //frames: Option<Frames<ShapeMesh>>,
    frames: Vec<GizmoShapeFrame>,
}

struct TextRenderer {

}

impl GizmoRenderer {
    pub fn free(&mut self, device: &ash::Device) {
        self.shape_renderer.free(device);
        self.text_renderer.free(device);
    }

    pub fn init(base: &VulkanBase) -> RisResult<Self> {
        Ok(Self{
            shape_renderer: ShapeRenderer::init(base)?,
            text_renderer: TextRenderer::init(base)?,
        })
    }

    pub fn draw_shapes(
        &mut self,
        base: &VulkanBase,
        entry: &SwapchainEntry,
        vertices: &[GizmoShapeVertex],
        window_drawable_size: (u32, u32),
        camera: &Camera,
    ) -> RisResult<()> {
        self.shape_renderer.draw(
            base,
            entry,
            vertices,
            window_drawable_size,
            camera,
        )
    }

    pub fn draw_text(
        &mut self,
        base: &VulkanBase,
        target: vk::ImageView,
        vertices: &[GizmoTextVertex],
        texture: &[u8],
        window_drawable_size: (u32, u32),
        camera: &Camera,
        sync: TransientCommandSync,
    ) -> RisResult<()> {
        self.text_renderer.draw(
            base,
            target,
            vertices,
            texture,
            window_drawable_size,
            camera,
            sync,
        )
    }
}

impl ShapeRenderer {
    fn free(&mut self, device: &ash::Device) {
        unsafe {
            for frame in self.frames.iter_mut() {
                frame.free(device);
            }

            device.destroy_descriptor_pool(self.descriptor_pool, None);
            device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);

            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.pipeline_layout, None);
            device.destroy_render_pass(self.render_pass, None);
        }
    }

    fn init(base: &VulkanBase) -> RisResult<Self> {
        let VulkanBase {
            instance,
            suitable_device,
            device,
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
        } = base;

        // descriptor sets
        let descriptor_set_layout_bindings = [vk::DescriptorSetLayoutBinding{
            binding: 0,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::VERTEX,
            p_immutable_samplers: ptr::null(),
        }];

        let descriptor_set_layout_create_info = vk::DescriptorSetLayoutCreateInfo{
            s_type: vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DescriptorSetLayoutCreateFlags::empty(),
            binding_count: descriptor_set_layout_bindings.len() as u32,
            p_bindings: descriptor_set_layout_bindings.as_ptr(),
        };

        let descriptor_set_layout = unsafe {
            device.create_descriptor_set_layout(&descriptor_set_layout_create_info, None)
        }?;
        
        let descriptor_pool_sizes = [vk::DescriptorPoolSize {
            ty: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: entries.len() as u32,
        }];

        let descriptor_pool_create_info = vk::DescriptorPoolCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DescriptorPoolCreateFlags::empty(),
            max_sets: entries.len() as u32,
            pool_size_count: descriptor_pool_sizes.len() as u32,
            p_pool_sizes: descriptor_pool_sizes.as_ptr(),
        };

        let descriptor_pool = unsafe {device.create_descriptor_pool(&descriptor_pool_create_info, None)}?;

        let mut descriptor_set_layouts = Vec::with_capacity(entries.len());
        for _ in 0..entries.len() {
            descriptor_set_layouts.push(descriptor_set_layout);
        }

        let descriptor_set_allocate_info = vk::DescriptorSetAllocateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
            p_next: ptr::null(),
            descriptor_pool,
            descriptor_set_count: descriptor_set_layouts.len() as u32,
            p_set_layouts: descriptor_set_layouts.as_ptr(),
        };

        let descriptor_sets = unsafe {device.allocate_descriptor_sets(&descriptor_set_allocate_info)}?;

        // shaders
        let vs_asset_id = AssetId::Directory(String::from(
            "__imported_raw/shaders/gizmo_segment.vert.spv",
        ));
        let gs_asset_id = AssetId::Directory(String::from(
            "__imported_raw/shaders/gizmo_segment.geom.spv",
        ));
        let fs_asset_id = AssetId::Directory(String::from(
            "__imported_raw/shaders/gizmo_segment.frag.spv",
        ));

        let vs_future = ris_asset::load_async(vs_asset_id);
        let gs_future = ris_asset::load_async(gs_asset_id);
        let fs_future = ris_asset::load_async(fs_asset_id);

        let vs_bytes = vs_future.wait(None)??;
        let gs_bytes = gs_future.wait(None)??;
        let fs_bytes = fs_future.wait(None)??;

        let vs_module = crate::shader::create_module(device, &vs_bytes)?;
        let gs_module = crate::shader::create_module(device, &gs_bytes)?;
        let fs_module = crate::shader::create_module(device, &fs_bytes)?;

        let main_function_name = CString::new("main").unwrap();

        let shader_stages = [
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::PipelineShaderStageCreateFlags::empty(),
                module: vs_module,
                p_name: main_function_name.as_ptr(),
                p_specialization_info: ptr::null(),
                stage: vk::ShaderStageFlags::VERTEX,
            },
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::PipelineShaderStageCreateFlags::empty(),
                module: gs_module,
                p_name: main_function_name.as_ptr(),
                p_specialization_info: ptr::null(),
                stage: vk::ShaderStageFlags::GEOMETRY,
            },
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::PipelineShaderStageCreateFlags::empty(),
                module: fs_module,
                p_name: main_function_name.as_ptr(),
                p_specialization_info: ptr::null(),
                stage: vk::ShaderStageFlags::FRAGMENT,
            },
        ];

        // pipeline
        let vertex_binding_descriptions = [vk::VertexInputBindingDescription {
            binding: 0,
            stride: 24,
            input_rate: vk::VertexInputRate::VERTEX,
        }];
        let vertex_attribute_descriptions = [
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: 0,
            },
            vk::VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: 12u32,
            },
        ];

        let vertex_input_state = [vk::PipelineVertexInputStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineVertexInputStateCreateFlags::empty(),
            vertex_binding_description_count: vertex_binding_descriptions.len() as u32,
            p_vertex_binding_descriptions: vertex_binding_descriptions.as_ptr(),
            vertex_attribute_description_count: vertex_attribute_descriptions.len() as u32,
            p_vertex_attribute_descriptions: vertex_attribute_descriptions.as_ptr(),
        }];

        let input_assembly_state = [vk::PipelineInputAssemblyStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineInputAssemblyStateCreateFlags::empty(),
            topology: vk::PrimitiveTopology::LINE_LIST,
            primitive_restart_enable: vk::FALSE,
        }];

        let viewports = [Default::default()];
        let scissors = [Default::default()];

        let viewport_state = [vk::PipelineViewportStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineViewportStateCreateFlags::empty(),
            viewport_count: 1,
            p_viewports: viewports.as_ptr(),
            scissor_count: 1,
            p_scissors: scissors.as_ptr(),
        }];

        let rasterization_state = [vk::PipelineRasterizationStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineRasterizationStateCreateFlags::empty(),
            depth_clamp_enable: vk::FALSE,
            rasterizer_discard_enable: vk::FALSE,
            polygon_mode: vk::PolygonMode::FILL,
            cull_mode: vk::CullModeFlags::NONE,
            front_face: vk::FrontFace::CLOCKWISE,
            depth_bias_enable: vk::FALSE,
            depth_bias_constant_factor: 0.0,
            depth_bias_clamp: 0.0,
            depth_bias_slope_factor: 0.0,
            line_width: 1.0,
        }];

        let multisample_state = [vk::PipelineMultisampleStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineMultisampleStateCreateFlags::empty(),
            rasterization_samples: vk::SampleCountFlags::TYPE_1,
            sample_shading_enable: vk::FALSE,
            min_sample_shading: 1.,
            p_sample_mask: ptr::null(),
            alpha_to_coverage_enable: vk::FALSE,
            alpha_to_one_enable: vk::FALSE,
        }];

        let depth_stencil_state = [vk::PipelineDepthStencilStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineDepthStencilStateCreateFlags::empty(),
            depth_test_enable: vk::FALSE,
            depth_write_enable: vk::FALSE,
            depth_compare_op: vk::CompareOp::ALWAYS,
            depth_bounds_test_enable: vk::FALSE,
            stencil_test_enable: vk::FALSE,
            front: Default::default(),
            back: Default::default(),
            min_depth_bounds: 0.0,
            max_depth_bounds: 0.0,
        }];

        let color_blend_attachment_states = [vk::PipelineColorBlendAttachmentState {
            blend_enable: vk::TRUE,
            src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ONE,
            dst_alpha_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            alpha_blend_op: vk::BlendOp::ADD,
            color_write_mask: vk::ColorComponentFlags::RGBA,
        }];

        let color_blend_state = [vk::PipelineColorBlendStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineColorBlendStateCreateFlags::empty(),
            logic_op_enable: vk::FALSE,
            logic_op: vk::LogicOp::COPY,
            attachment_count: color_blend_attachment_states.len() as u32,
            p_attachments: color_blend_attachment_states.as_ptr(),
            blend_constants: [0., 0., 0., 0.],
        }];

        let dynamic_states = [vk::DynamicState::SCISSOR, vk::DynamicState::VIEWPORT];
        let dynamic_state = [vk::PipelineDynamicStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineDynamicStateCreateFlags::empty(),
            dynamic_state_count: dynamic_states.len() as u32,
            p_dynamic_states: dynamic_states.as_ptr(),
        }];

        // pipeline layout
        let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo {
            s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: descriptor_set_layouts.len() as u32,
            p_set_layouts: descriptor_set_layouts.as_ptr(),
            push_constant_range_count: 0,
            p_push_constant_ranges: ptr::null(),
        };

        let pipeline_layout =
            unsafe { device.create_pipeline_layout(&pipeline_layout_create_info, None) }?;

        // render pass
        let color_attachment = vk::AttachmentDescription {
            flags: vk::AttachmentDescriptionFlags::empty(),
            format: swapchain_format.format,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::LOAD,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::PRESENT_SRC_KHR,
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
        };

        let color_attachment_references = [vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }];

        let subpass_descriptions = [vk::SubpassDescription {
            flags: vk::SubpassDescriptionFlags::empty(),
            pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
            input_attachment_count: 0,
            p_input_attachments: ptr::null(),
            color_attachment_count: color_attachment_references.len() as u32,
            p_color_attachments: color_attachment_references.as_ptr(),
            p_resolve_attachments: ptr::null(),
            p_depth_stencil_attachment: ptr::null(),
            preserve_attachment_count: 0,
            p_preserve_attachments: ptr::null(),
        }];

        let supbass_dependencies = [vk::SubpassDependency {
            src_subpass: vk::SUBPASS_EXTERNAL,
            dst_subpass: 0,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
                | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
                | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
            src_access_mask: vk::AccessFlags::empty(),
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE
                | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
            dependency_flags: vk::DependencyFlags::empty(),
        }];

        let attachments = [color_attachment];

        let render_pass_create_info = vk::RenderPassCreateInfo {
            s_type: vk::StructureType::RENDER_PASS_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::RenderPassCreateFlags::empty(),
            attachment_count: attachments.len() as u32,
            p_attachments: attachments.as_ptr(),
            subpass_count: subpass_descriptions.len() as u32,
            p_subpasses: subpass_descriptions.as_ptr(),
            dependency_count: supbass_dependencies.len() as u32,
            p_dependencies: supbass_dependencies.as_ptr(),
        };

        let render_pass = unsafe { device.create_render_pass(&render_pass_create_info, None) }?;

        // pipeline creation
        let graphics_pipeline_create_info = [vk::GraphicsPipelineCreateInfo {
            s_type: vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineCreateFlags::empty(),
            stage_count: shader_stages.len() as u32,
            p_stages: shader_stages.as_ptr(),
            p_vertex_input_state: vertex_input_state.as_ptr(),
            p_input_assembly_state: input_assembly_state.as_ptr(),
            p_tessellation_state: ptr::null(),
            p_viewport_state: viewport_state.as_ptr(),
            p_rasterization_state: rasterization_state.as_ptr(),
            p_multisample_state: multisample_state.as_ptr(),
            p_depth_stencil_state: depth_stencil_state.as_ptr(),
            p_color_blend_state: color_blend_state.as_ptr(),
            p_dynamic_state: dynamic_state.as_ptr(),
            layout: pipeline_layout,
            render_pass,
            subpass: 0,
            base_pipeline_handle: vk::Pipeline::null(),
            base_pipeline_index: -1,
        }];

        let graphics_pipelines = unsafe {
            device.create_graphics_pipelines(
                vk::PipelineCache::null(),
                &graphics_pipeline_create_info,
                None,
            )
        }
        .map_err(|e| e.1)?;
        let pipeline = graphics_pipelines.into_iter().next().unroll()?;

        unsafe { device.destroy_shader_module(vs_module, None) };
        unsafe { device.destroy_shader_module(gs_module, None) };
        unsafe { device.destroy_shader_module(fs_module, None) };

        let physical_device_memory_properties = unsafe {
            instance.get_physical_device_memory_properties(suitable_device.physical_device)
        };

        let mut frames = Vec::with_capacity(entries.len());
        for i in 0..entries.len() {
            unsafe {
                let buffer_size = std::mem::size_of::<UniformBufferObject>() as vk::DeviceSize;
                let descriptor_buffer = Buffer::alloc(
                    device,
                    buffer_size,
                    vk::BufferUsageFlags::UNIFORM_BUFFER,
                    vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
                    physical_device_memory_properties
                )?;

                let descriptor_mapped = device.map_memory(
                    descriptor_buffer.memory,
                    0,
                    buffer_size,
                    vk::MemoryMapFlags::empty(),
                )? as *mut UniformBufferObject;

                let descriptor_set = descriptor_sets[i];

                let frame = GizmoShapeFrame{
                    mesh: None,
                    framebuffer: None,
                    descriptor_buffer,
                    descriptor_mapped,
                    descriptor_set,
                };
                frames.push(frame);
            }
        }

        Ok(Self {
            pipeline,
            pipeline_layout,
            render_pass,
            descriptor_set_layout,
            descriptor_pool,
            frames,
        })
    }

    fn draw(
        &mut self,
        base: &VulkanBase,
        entry: &SwapchainEntry,
        vertices: &[GizmoShapeVertex],
        window_drawable_size: (u32, u32),
        camera: &Camera,
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
                    ..
                },
            ..
        } = base;

        let SwapchainEntry {
            index,
            image,
            image_view,
            command_buffer,
        } = entry;

        if vertices.is_empty() {
            return Ok(());
        }

        let GizmoShapeFrame {
            mesh,
            framebuffer,
            descriptor_buffer,
            descriptor_mapped,
            descriptor_set,
        } = &mut self.frames[*index];

        // mesh
        let physical_device_memory_properties = unsafe {
            instance.get_physical_device_memory_properties(suitable_device.physical_device)
        };

        let mesh = match mesh {
            Some(mesh) => {
                mesh.update(device, physical_device_memory_properties, vertices)?;
                mesh
            },
            None => {
                let new_mesh = unsafe {ShapeMesh::alloc(device, physical_device_memory_properties, vertices)}?;
                *mesh = Some(new_mesh);
                mesh.as_mut().unroll()?
            },
        };

        // framebuffer
        if let Some(framebuffer) = framebuffer.take() {
            unsafe { device.destroy_framebuffer(framebuffer, None) };
        }

        let attachments = [*image_view];

        let frame_buffer_create_info = vk::FramebufferCreateInfo{
            s_type: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::FramebufferCreateFlags::empty(),
            render_pass: self.render_pass,
            attachment_count: attachments.len() as u32,
            p_attachments: attachments.as_ptr(),
            width: swapchain_extent.width,
            height: swapchain_extent.height,
            layers: 1,
        };

        let new_framebuffer = unsafe {device.create_framebuffer(&frame_buffer_create_info, None)}?;
        *framebuffer = Some(new_framebuffer);
        let framebuffer = new_framebuffer;

        //let transient_command = TransientCommand::begin(device, *graphics_queue, *transient_command_pool)?;

        // render pass
        let clear_values = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 0.0],
            },
        }];

        let render_pass_begin_info = vk::RenderPassBeginInfo {
            s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
            p_next: ptr::null(),
            render_pass: self.render_pass,
            framebuffer,
            render_area: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: *swapchain_extent,
            },
            clear_value_count: clear_values.len() as u32,
            p_clear_values: clear_values.as_ptr(),
        };

        unsafe {
            device.cmd_begin_render_pass(
                *command_buffer,
                &render_pass_begin_info,
                vk::SubpassContents::INLINE,
            )
        };
        
        unsafe {
            device.cmd_bind_pipeline(
                *command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline,
            )
        };

        let viewports = [vk::Viewport {
            width: window_drawable_size.0 as f32,
            height: window_drawable_size.1 as f32,
            max_depth: 1.0,
            ..Default::default()
        }];

        unsafe {
            device.cmd_set_viewport(
                *command_buffer,
                0,
                &viewports,
            )
        };

        let scissors = [vk::Rect2D{
            offset: vk::Offset2D {
                x: 0,
                y: 0,
            },
            extent: vk::Extent2D {
                width: window_drawable_size.0,
                height: window_drawable_size.1,
            }
        }];

        unsafe {
            device.cmd_set_scissor(
                *command_buffer,
                0,
                &scissors,
            );
        };

        unsafe {
            device.cmd_bind_vertex_buffers(
                *command_buffer,
                0,
                &[mesh.vertices.buffer],
                &[0],
            )
        };

        let ubo = UniformBufferObject {
            view: camera.view_matrix(),
            proj: camera.projection_matrix(),
        };

        let ubo = [ubo];
        unsafe { descriptor_mapped.copy_from_nonoverlapping(ubo.as_ptr(), ubo.len()) };

        let descriptor_buffer_info = [vk::DescriptorBufferInfo {
            buffer: descriptor_buffer.buffer,
            offset: 0,
            range: std::mem::size_of::<UniformBufferObject>() as vk::DeviceSize,
        }];
        
        let write_descriptor_sets = [vk::WriteDescriptorSet {
            s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
            p_next: ptr::null(),
            dst_set: *descriptor_set,
            dst_binding: 0,
            dst_array_element: 0,
            descriptor_count: 1,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
            p_image_info: ptr::null(),
            p_buffer_info: descriptor_buffer_info.as_ptr(),
            p_texel_buffer_view: ptr::null(),
        }];

        unsafe {
            device.update_descriptor_sets(&write_descriptor_sets, &[])
        };

        unsafe {
            device.cmd_bind_descriptor_sets(
                *command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout,
                0,
                &[*descriptor_set],
                &[],
            )
        };

        unsafe {
            device.cmd_draw(
                *command_buffer,
                vertices.len() as u32,
                1,
                0,
                0,
            )
        };

        unsafe { device.cmd_end_render_pass(*command_buffer) };
        Ok(())
    }
}

impl TextRenderer {
    fn free(&mut self, device: &ash::Device) {

    }

    fn init(base: &VulkanBase) -> RisResult<Self> {
        Ok(Self{
            
        })
    }

    fn draw(
        &mut self,
        base: &VulkanBase,
        target: vk::ImageView,
        vertices: &[GizmoTextVertex],
        texture: &[u8],
        window_drawable_size: (u32, u32),
        camera: &Camera,
        sync: TransientCommandSync,
    ) -> RisResult<()> {
        let VulkanBase {
            device,
            graphics_queue,
            ..
        } = base;

        sync.sync_now(device, *graphics_queue)?;

        Ok(())
    }
}
