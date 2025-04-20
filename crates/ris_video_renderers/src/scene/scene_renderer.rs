use std::io::Seek;
use std::io::Write;
use std::ptr;

use ash::vk;

use ris_asset::codecs::qoi;
use ris_asset::RisGodAsset;
use ris_asset_data::mesh::MeshPrototype;
use ris_asset_data::mesh::GpuMesh;
//use ris_data::ecs::mesh::VERTEX_ATTRIBUTE_DESCRIPTIONS;
//use ris_data::ecs::mesh::VERTEX_BINDING_DESCRIPTIONS;
use ris_data::ecs::scene::Scene;
use ris_error::Extensions;
use ris_error::RisResult;
use ris_math::camera::Camera;
use ris_math::matrix::Mat4;
use ris_math::vector::Vec3;
use ris_math::vector::Vec2;
use ris_video_data::buffer::Buffer;
use ris_video_data::core::VulkanCore;
use ris_video_data::swapchain::SwapchainEntry;
use ris_video_data::texture::Texture;
use ris_video_data::texture::TextureCreateInfo;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PushConstants {
    pub model: Mat4,
}

const _: () = {
    assert!(
        std::mem::size_of::<PushConstants>() <= 128,
        "PushConstants may not exceed 128 bytes",
    )
};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct UniformBufferObject {
    pub view: Mat4,
    pub proj: Mat4,
}


pub struct SceneFrame {
    framebuffer: Option<vk::Framebuffer>,
    descriptor_buffer: Buffer,
    descriptor_mapped: *mut UniformBufferObject,
    descriptor_set: vk::DescriptorSet,
}

impl SceneFrame {
    /// # Safety
    ///
    /// May only be called once. Memory must not be freed twice.
    pub unsafe fn free(&mut self, device: &ash::Device) {
        if let Some(framebuffer) = self.framebuffer.take() {
            device.destroy_framebuffer(framebuffer, None);
        }

        self.descriptor_buffer.free(device);
    }
}

pub struct SceneRenderer {
    descriptor_set_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
    render_pass: vk::RenderPass,
    pipeline: vk::Pipeline,
    pipeline_layout: vk::PipelineLayout,
    frames: Vec<SceneFrame>,
    texture: Texture,
    test_mesh: GpuMesh,
}

impl SceneRenderer {
    /// # Safety
    ///
    /// May only be called once. Memory must not be freed twice.
    pub unsafe fn free(&mut self, device: &ash::Device) {
        unsafe {
            for frame in self.frames.iter_mut() {
                frame.free(device);
            }

            device.destroy_descriptor_pool(self.descriptor_pool, None);
            device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);

            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.pipeline_layout, None);
            device.destroy_render_pass(self.render_pass, None);

            self.texture.free(device);
        }
    }

    pub fn alloc(core: &VulkanCore, god_asset: &RisGodAsset) -> RisResult<Self> {
        let VulkanCore {
            instance,
            suitable_device,
            device,
            graphics_queue,
            transient_command_pool,
            swapchain,
            ..
        } = core;

        let physical_device_memory_properties = unsafe {
            instance.get_physical_device_memory_properties(suitable_device.physical_device)
        };
        let physical_device_properties =
            unsafe { instance.get_physical_device_properties(suitable_device.physical_device) };

        // texture
        let texture_asset_id = god_asset.texture.clone();
        let content = ris_asset::load_raw_async(texture_asset_id.clone()).wait()?;
        let (pixels, desc) = qoi::decode(&content, None)?;

        let pixels_rgba = match desc.channels {
            qoi::Channels::RGB => ris_asset::util::add_alpha_channel(&pixels)?,
            qoi::Channels::RGBA => pixels,
        };

        let texture = Texture::alloc(TextureCreateInfo {
            device,
            queue: *graphics_queue,
            transient_command_pool: *transient_command_pool,
            physical_device_memory_properties,
            physical_device_properties,
            width: desc.width,
            height: desc.height,
            format: vk::Format::R8G8B8A8_SRGB,
            filter: vk::Filter::LINEAR,
            pixels_rgba: &pixels_rgba,
        })?;

        // push constants
        let push_constant_range = [vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::VERTEX,
            offset: 0,
            size: std::mem::size_of::<PushConstants>() as u32,
        }];

        // descriptor sets
        let descriptor_set_layout_bindings = [
            vk::DescriptorSetLayoutBinding {
                binding: 0,
                descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::VERTEX,
                p_immutable_samplers: ptr::null(),
            },
            vk::DescriptorSetLayoutBinding {
                binding: 1,
                descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::FRAGMENT,
                p_immutable_samplers: ptr::null(),
            },
        ];

        let descriptor_set_layout_create_info = vk::DescriptorSetLayoutCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DescriptorSetLayoutCreateFlags::empty(),
            binding_count: descriptor_set_layout_bindings.len() as u32,
            p_bindings: descriptor_set_layout_bindings.as_ptr(),
        };

        let descriptor_set_layout = unsafe {
            device.create_descriptor_set_layout(&descriptor_set_layout_create_info, None)
        }?;

        let descriptor_pool_sizes = [
            vk::DescriptorPoolSize {
                ty: vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count: swapchain.entries.len() as u32,
            },
            vk::DescriptorPoolSize {
                ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: swapchain.entries.len() as u32,
            },
        ];

        let total_descriptor_set_count = swapchain.entries.len();
        let descriptor_pool_create_info = vk::DescriptorPoolCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DescriptorPoolCreateFlags::empty(),
            max_sets: total_descriptor_set_count as u32,
            pool_size_count: descriptor_pool_sizes.len() as u32,
            p_pool_sizes: descriptor_pool_sizes.as_ptr(),
        };

        let descriptor_pool =
            unsafe { device.create_descriptor_pool(&descriptor_pool_create_info, None) }?;

        let mut descriptor_set_layout_vec = Vec::with_capacity(total_descriptor_set_count);
        for _ in 0..total_descriptor_set_count {
            descriptor_set_layout_vec.push(descriptor_set_layout);
        }

        let descriptor_set_allocate_info = vk::DescriptorSetAllocateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
            p_next: ptr::null(),
            descriptor_pool,
            descriptor_set_count: descriptor_set_layout_vec.len() as u32,
            p_set_layouts: descriptor_set_layout_vec.as_ptr(),
        };

        let descriptor_sets =
            unsafe { device.allocate_descriptor_sets(&descriptor_set_allocate_info) }?;
        ris_error::assert!(descriptor_sets.len() == total_descriptor_set_count)?;

        // shaders
        let vs_asset_future = ris_asset::load_raw_async(god_asset.default_vert_spv.clone());
        let fs_asset_future = ris_asset::load_raw_async(god_asset.default_frag_spv.clone());

        let vs_bytes = vs_asset_future.wait()?;
        let fs_bytes = fs_asset_future.wait()?;

        let vs_module = ris_video_data::shader::create_module(device, &vs_bytes)?;
        let fs_module = ris_video_data::shader::create_module(device, &fs_bytes)?;
        let entry = ris_video_data::shader::ENTRY.as_ptr();

        let shader_stages = [
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::PipelineShaderStageCreateFlags::empty(),
                module: vs_module,
                p_name: entry,
                p_specialization_info: ptr::null(),
                stage: vk::ShaderStageFlags::VERTEX,
            },
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::PipelineShaderStageCreateFlags::empty(),
                module: fs_module,
                p_name: entry,
                p_specialization_info: ptr::null(),
                stage: vk::ShaderStageFlags::FRAGMENT,
            },
        ];

        // pipeline
        let vertex_binding_descriptions = ris_asset_data::mesh::VERTEX_BINDING_DESCRIPTIONS;
        let vertex_attribute_descriptions = ris_asset_data::mesh::VERTEX_ATTRIBUTE_DESCRIPTIONS;

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
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            primitive_restart_enable: vk::FALSE,
        }];

        let viewports = [vk::Viewport::default()];
        let scissors = [vk::Rect2D::default()];

        let viewport_state = [vk::PipelineViewportStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineViewportStateCreateFlags::empty(),
            viewport_count: viewports.len() as u32,
            p_viewports: viewports.as_ptr(),
            scissor_count: scissors.len() as u32,
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
            min_sample_shading: 0.0,
            p_sample_mask: ptr::null(),
            alpha_to_coverage_enable: vk::FALSE,
            alpha_to_one_enable: vk::FALSE,
        }];

        let stencil_op_state = vk::StencilOpState {
            fail_op: vk::StencilOp::KEEP,
            pass_op: vk::StencilOp::KEEP,
            depth_fail_op: vk::StencilOp::KEEP,
            compare_op: vk::CompareOp::ALWAYS,
            compare_mask: 0,
            write_mask: 0,
            reference: 0,
        };

        let depth_stencil_state = [vk::PipelineDepthStencilStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineDepthStencilStateCreateFlags::empty(),
            depth_test_enable: vk::TRUE,
            depth_write_enable: vk::TRUE,
            depth_compare_op: vk::CompareOp::GREATER,
            depth_bounds_test_enable: vk::FALSE,
            stencil_test_enable: vk::FALSE,
            front: stencil_op_state,
            back: stencil_op_state,
            min_depth_bounds: 0.0,
            max_depth_bounds: 1.0,
        }];

        // pseudocode of how blending with vk::PipelineColorBlendAttachmentState works:
        //
        //     if (blend_enable) {
        //         final_color.rgb = (src_color_blend_factor * new_color.rgb) <color_blend_op> (dst_color_blend_factor * old_color.rgb);
        //         final_color.a = (src_alpha_blend_factor * new_color.a) <alpha_blend_op> (dst_alpha_blend_factor * old_color.a);
        //     } else {
        //         final_color = new_color;
        //     }
        //
        //     final_color = final_color & color_write_mask;

        let color_blend_attachment_states = [vk::PipelineColorBlendAttachmentState {
            blend_enable: vk::FALSE,
            src_color_blend_factor: vk::BlendFactor::ONE,
            dst_color_blend_factor: vk::BlendFactor::ZERO,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ONE,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
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
            blend_constants: [0.0, 0.0, 0.0, 0.0],
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
        let descriptor_set_layouts = [descriptor_set_layout];

        let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo {
            s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: descriptor_set_layouts.len() as u32,
            p_set_layouts: descriptor_set_layouts.as_ptr(),
            push_constant_range_count: push_constant_range.len() as u32,
            p_push_constant_ranges: push_constant_range.as_ptr(),
        };

        let pipeline_layout =
            unsafe { device.create_pipeline_layout(&pipeline_layout_create_info, None) }?;

        // render pass
        let color_attachment = vk::AttachmentDescription {
            flags: vk::AttachmentDescriptionFlags::empty(),
            format: swapchain.format.format,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
        };

        let depth_attachment = vk::AttachmentDescription {
            flags: vk::AttachmentDescriptionFlags::empty(),
            format: ris_video_data::util::find_depth_format(
                instance,
                suitable_device.physical_device,
            )?,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::DONT_CARE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };

        let color_attachment_references = [vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }];

        let depth_attachment_reference = [vk::AttachmentReference {
            attachment: 1,
            layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        }];

        let subpass_descriptions = [vk::SubpassDescription {
            flags: vk::SubpassDescriptionFlags::empty(),
            pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
            input_attachment_count: 0,
            p_input_attachments: ptr::null(),
            color_attachment_count: color_attachment_references.len() as u32,
            p_color_attachments: color_attachment_references.as_ptr(),
            p_resolve_attachments: ptr::null(),
            p_depth_stencil_attachment: depth_attachment_reference.as_ptr(),
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

        let attachments = [color_attachment, depth_attachment];

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
        let pipeline = graphics_pipelines.into_iter().next().into_ris_error()?;

        unsafe { device.destroy_shader_module(vs_module, None) };
        unsafe { device.destroy_shader_module(fs_module, None) };

        // frames
        let frame_count = swapchain.entries.len();
        let mut frames = Vec::with_capacity(frame_count);
        for descriptor_set in descriptor_sets {
            let buffer_size = std::mem::size_of::<UniformBufferObject>() as vk::DeviceSize;
            let descriptor_buffer = Buffer::alloc(
                device,
                buffer_size,
                vk::BufferUsageFlags::UNIFORM_BUFFER,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
                physical_device_memory_properties,
            )?;

            let descriptor_mapped = unsafe {
                device.map_memory(
                    descriptor_buffer.memory,
                    0,
                    buffer_size,
                    vk::MemoryMapFlags::empty(),
                )
            }? as *mut UniformBufferObject;

            let frame = SceneFrame {
                framebuffer: None,
                descriptor_buffer,
                descriptor_mapped,
                descriptor_set,
            };
            frames.push(frame);
        }

        // test
        let device = device.clone();
        let physical_device_memory_properties = physical_device_memory_properties.clone();

        let asset_id = ris_asset_data::AssetId::Path("models/human.ris_mesh".to_string());
        let mesh_receiver = ris_asset::load_async(asset_id, move |bytes| {
            let cpu_mesh = ris_asset::assets::ris_mesh::deserialize(&bytes)?;
            let gpu_mesh = unsafe {GpuMesh::from_cpu_mesh(
                &device,
                physical_device_memory_properties,
                cpu_mesh,
            )}?;
            Ok(gpu_mesh)
        });
        let test_mesh = mesh_receiver.wait()?;

        Ok(Self {
            descriptor_set_layout,
            descriptor_pool,
            render_pass,
            pipeline,
            pipeline_layout,
            frames,
            texture,
            test_mesh,
        })
    }

    pub fn draw(
        &mut self,
        core: &VulkanCore,
        entry: &SwapchainEntry,
        window_drawable_size: (u32, u32),
        camera: &Camera,
        scene: &Scene,
    ) -> RisResult<()> {
        let VulkanCore {
            device, swapchain, ..
        } = core;

        let SwapchainEntry {
            index,
            viewport_image_view,
            depth_image_view,
            command_buffer,
            ..
        } = entry;

        let SceneFrame {
            framebuffer,
            descriptor_buffer,
            descriptor_mapped,
            descriptor_set,
        } = &mut self.frames[*index];

        // framebuffer
        if let Some(framebuffer) = framebuffer.take() {
            unsafe { device.destroy_framebuffer(framebuffer, None) };
        }

        let attachments = [*viewport_image_view, *depth_image_view];

        let frame_buffer_create_info = vk::FramebufferCreateInfo {
            s_type: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::FramebufferCreateFlags::empty(),
            render_pass: self.render_pass,
            attachment_count: attachments.len() as u32,
            p_attachments: attachments.as_ptr(),
            width: swapchain.extent.width,
            height: swapchain.extent.height,
            layers: 1,
        };

        let new_framebuffer =
            unsafe { device.create_framebuffer(&frame_buffer_create_info, None) }?;
        *framebuffer = Some(new_framebuffer);
        let framebuffer = new_framebuffer;

        // render pass
        unsafe {
            let clear_values = [
                vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 0.0],
                    },
                },
                vk::ClearValue {
                    depth_stencil: vk::ClearDepthStencilValue {
                        depth: 0.0,
                        stencil: 0,
                    },
                },
            ];

            let render_pass_begin_info = vk::RenderPassBeginInfo {
                s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
                p_next: ptr::null(),
                render_pass: self.render_pass,
                framebuffer,
                render_area: vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: swapchain.extent,
                },
                clear_value_count: clear_values.len() as u32,
                p_clear_values: clear_values.as_ptr(),
            };

            device.cmd_begin_render_pass(
                *command_buffer,
                &render_pass_begin_info,
                vk::SubpassContents::INLINE,
            );

            device.cmd_bind_pipeline(
                *command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline,
            );

            let viewports = [vk::Viewport {
                x: 0.0,
                y: 0.0,
                width: window_drawable_size.0 as f32,
                height: window_drawable_size.1 as f32,
                min_depth: 0.0,
                max_depth: 1.0,
            }];

            let scissors = [vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: vk::Extent2D {
                    width: window_drawable_size.0,
                    height: window_drawable_size.1,
                },
            }];

            device.cmd_set_viewport(*command_buffer, 0, &viewports);
            device.cmd_set_scissor(*command_buffer, 0, &scissors);

            let ubo = [UniformBufferObject {
                view: camera.view_matrix(),
                proj: camera.projection_matrix(),
            }];
            descriptor_mapped.copy_from_nonoverlapping(ubo.as_ptr(), ubo.len());

            let descriptor_buffer_info = [vk::DescriptorBufferInfo {
                buffer: descriptor_buffer.buffer,
                offset: 0,
                range: std::mem::size_of::<UniformBufferObject>() as vk::DeviceSize,
            }];

            let descriptor_image_info = [vk::DescriptorImageInfo {
                sampler: self.texture.sampler,
                image_view: self.texture.view,
                image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            }];

            let write_descriptor_sets = [
                vk::WriteDescriptorSet {
                    s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
                    p_next: ptr::null(),
                    dst_set: *descriptor_set,
                    dst_binding: 0,
                    dst_array_element: 0,
                    descriptor_count: descriptor_buffer_info.len() as u32,
                    descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                    p_image_info: ptr::null(),
                    p_buffer_info: descriptor_buffer_info.as_ptr(),
                    p_texel_buffer_view: ptr::null(),
                },
                vk::WriteDescriptorSet {
                    s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
                    p_next: ptr::null(),
                    dst_set: *descriptor_set,
                    dst_binding: 1,
                    dst_array_element: 0,
                    descriptor_count: descriptor_image_info.len() as u32,
                    descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                    p_image_info: descriptor_image_info.as_ptr(),
                    p_buffer_info: ptr::null(),
                    p_texel_buffer_view: ptr::null(),
                },
            ];

            device.update_descriptor_sets(&write_descriptor_sets, &[]);

            device.cmd_bind_descriptor_sets(
                *command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout,
                0,
                &[*descriptor_set],
                &[],
            );

            //for mesh_renderer_component in scene.mesh_renderer_components.iter() {
            //    let aref = mesh_renderer_component.borrow();
            //    if !aref.is_alive {
            //        continue;
            //    }

            //    let game_object = aref.game_object();
            //    if game_object.is_active_in_hierarchy(scene) != Ok(true) {
            //        continue;
            //    }

            //    let Ok(model) = game_object.model(scene) else {
            //        continue;
            //    };

            //    let Some(video_mesh_handle) = aref.video_mesh() else {
            //        continue;
            //    };

            //    let Ok(Some(vertices)) = video_mesh_handle.vertices(scene) else {
            //        continue;
            //    };

            //    let Ok(Some(indices)) = video_mesh_handle.indices(scene) else {
            //        continue;
            //    };

            //    let Ok(Some(index_count)) = video_mesh_handle.index_count(scene) else {
            //        continue;
            //    };

                //let push_constants = PushConstants { model };
                let push_constants = PushConstants {
                    model: Mat4::init(1.0)
                };

                let push_constants_ptr = &push_constants as *const PushConstants as *const u8;
                let size = std::mem::size_of::<PushConstants>();
                let push_constants_bytes = std::slice::from_raw_parts(push_constants_ptr, size);

                device.cmd_push_constants(
                    *command_buffer,
                    self.pipeline_layout,
                    vk::ShaderStageFlags::VERTEX,
                    0,
                    push_constants_bytes,
                );

                //device.cmd_bind_vertex_buffers(*command_buffer, 0, &[vertices.buffer], &[0]);
                device.cmd_bind_vertex_buffers(
                    *command_buffer,
                    0,
                    &self.test_mesh.vertex_buffers()?,
                    &self.test_mesh.vertex_offsets()?,
                );

                //device.cmd_bind_index_buffer(
                //    *command_buffer,
                //    indices.buffer,
                //    0,
                //    vk::IndexType::UINT16,
                //);
                device.cmd_bind_index_buffer(
                    *command_buffer,
                    self.test_mesh.index_buffer()?,
                    self.test_mesh.index_offset()?,
                    self.test_mesh.index_type(),
                );

                //let index_count_u32 = index_count as u32;
                device.cmd_draw_indexed(
                    *command_buffer,
                    self.test_mesh.index_count()?,
                    1,
                    0,
                    0,
                    0,
                );
            //}

            device.cmd_end_render_pass(*command_buffer);
        }

        Ok(())
    }
}
