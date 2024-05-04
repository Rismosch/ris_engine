use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;
use std::sync::Arc;
use std::collections::HashMap;

use ash::vk;

use imgui::internal::RawWrapper;
use imgui::Context;
use imgui::DrawCmd;
use imgui::DrawCmdParams;
use imgui::DrawData;
use imgui::DrawVert;
use imgui::FontAtlas;
use imgui::TextureId;
use imgui::Textures;

use ris_asset::AssetId;
use ris_asset::loader::scenes_loader::Scenes;
use ris_error::Extensions;
use ris_error::RisResult;
use ris_math::matrix::Mat4;

use crate::vulkan::buffer::Buffer;
use crate::vulkan::renderer::Renderer;
use crate::vulkan::swapchain::BaseSwapchain;
use crate::vulkan::swapchain::Swapchain;
use crate::vulkan::texture::Texture;
use crate::vulkan::transient_command::TransientCommand;

pub struct ImguiRenderer {
    render_pass: vk::RenderPass,
    descriptor_set_layout: vk::DescriptorSetLayout,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    font_texture: Texture,
}

impl ImguiRenderer {
    pub fn free(&self, device: &ash::Device) {
        unsafe {
            self.font_texture.free(device);

            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.pipeline_layout, None);
            device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);
            device.destroy_render_pass(self.render_pass, None);
        }
    }

    pub fn init(renderer: &Renderer, scenes: &Scenes, context: &mut Context) -> RisResult<Self> {
        let Renderer {
            instance,
            suitable_device,
            device,
            graphics_queue,
            transient_command_pool,
            swapchain : Swapchain {
                base: BaseSwapchain {
                    format: swapchain_format,
                    extent: swapchain_extent,
                    ..
                },
                ..
            },
            ..
        } = renderer;

        // shaders
        let vs_asset_id =
            AssetId::Directory(String::from("__imported_raw/shaders/imgui.vert.spv"));
        let fs_asset_id =
            AssetId::Directory(String::from("__imported_raw/shaders/imgui.frag.spv"));

        let vs_asset_future = ris_asset::load_async(vs_asset_id);
        let fs_asset_future = ris_asset::load_async(fs_asset_id);

        let vs_bytes = vs_asset_future.wait(None)??;
        let fs_bytes = fs_asset_future.wait(None)??;

        // asset data is read in u8, but vulkan expects it to be in u32.
        // assert that the data is properly aligned
        ris_error::assert!(vs_bytes.len() % 4 == 0)?;
        ris_error::assert!(fs_bytes.len() % 4 == 0)?;

        let vs_shader_module_create_info = vk::ShaderModuleCreateInfo {
            s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::ShaderModuleCreateFlags::empty(),
            code_size: vs_bytes.len(),
            p_code: vs_bytes.as_ptr() as *const u32,
        };
        let fs_shader_module_create_info = vk::ShaderModuleCreateInfo {
            s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::ShaderModuleCreateFlags::empty(),
            code_size: fs_bytes.len(),
            p_code: fs_bytes.as_ptr() as *const u32,
        };

        let vs_shader_module =
            unsafe { device.create_shader_module(&vs_shader_module_create_info, None) }?;
        let fs_shader_module =
            unsafe { device.create_shader_module(&fs_shader_module_create_info, None) }?;

        let main_function_name = CString::new("main").unwrap();

        let shader_stages = [
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::PipelineShaderStageCreateFlags::empty(),
                module: vs_shader_module,
                p_name: main_function_name.as_ptr(),
                p_specialization_info: ptr::null(),
                stage: vk::ShaderStageFlags::VERTEX,
            },
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::PipelineShaderStageCreateFlags::empty(),
                module: fs_shader_module,
                p_name: main_function_name.as_ptr(),
                p_specialization_info: ptr::null(),
                stage: vk::ShaderStageFlags::FRAGMENT,
            },
        ];

        // render pass
        let color_attachment = vk::AttachmentDescription {
            flags: vk::AttachmentDescriptionFlags::empty(),
            format: swapchain_format.format,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::LOAD,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
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

        // pipeline
        let vertex_binding_descriptions = [vk::VertexInputBindingDescription {
            binding: 0,
            stride: 20,
            input_rate: vk::VertexInputRate::VERTEX,
        }];
        let vertex_attribute_descriptions = [
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32G32_SFLOAT,
                offset: 0,
            },
            vk::VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: vk::Format::R32G32_SFLOAT,
                offset: 8 as u32,
            },
            vk::VertexInputAttributeDescription {
                location: 2,
                binding: 0,
                format: vk::Format::R32_UINT,
                offset: 16 as u32,
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
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            primitive_restart_enable: vk::FALSE,
        }];

        let viewports = [vk::Viewport {
            x: 0.,
            y: 0.,
            width: swapchain_extent.width as f32,
            height: swapchain_extent.height as f32,
            min_depth: 0.,
            max_depth: 1.,
        }];

        let scissors = [vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: *swapchain_extent,
        }];

        let viewport_state = [vk::PipelineViewportStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineViewportStateCreateFlags::empty(),
            viewport_count: 1,
            p_viewports: viewports.as_ptr(),
            scissor_count: 1,
            p_scissors: scissors.as_ptr(),
        }];

        let multisample_state = [vk::PipelineMultisampleStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineMultisampleStateCreateFlags::empty(),
            rasterization_samples: vk::SampleCountFlags::TYPE_1,
            sample_shading_enable: vk::FALSE,
            min_sample_shading: 0.,
            p_sample_mask: ptr::null(),
            alpha_to_coverage_enable: vk::FALSE,
            alpha_to_one_enable: vk::FALSE,
        }];

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
            blend_constants: [0., 0., 0., 0.],
        }];

        let descriptor_set_layout_bindings = [
            vk::DescriptorSetLayoutBinding {
                binding: 0,
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

        let descriptor_set_layouts = [descriptor_set_layout];

        let push_constant_ranges = [vk::PushConstantRange{
            stage_flags: vk::ShaderStageFlags::VERTEX,
            offset: 0,
            size: std::mem::size_of::<Mat4>() as u32,
        }];

        let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo {
            s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: descriptor_set_layouts.len() as u32,
            p_set_layouts: descriptor_set_layouts.as_ptr(),
            push_constant_range_count: push_constant_ranges.len() as u32,
            p_push_constant_ranges: push_constant_ranges.as_ptr(),
        };

        let pipeline_layout = unsafe { device.create_pipeline_layout(&pipeline_layout_create_info, None) }?;

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
            p_rasterization_state: ptr::null(),
            p_multisample_state: multisample_state.as_ptr(),
            p_depth_stencil_state: ptr::null(),
            p_color_blend_state: color_blend_state.as_ptr(),
            p_dynamic_state: ptr::null(),
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

        unsafe { device.destroy_shader_module(vs_shader_module, None) };
        unsafe { device.destroy_shader_module(fs_shader_module, None) };

        // textures
        let font_atlas_texture = context.fonts().build_rgba32_texture();
        let transient_command = TransientCommand::begin(
            device,
            *graphics_queue,
            *transient_command_pool,
        )?;

        let physical_device_memory_properties = unsafe {
            instance.get_physical_device_memory_properties(suitable_device.physical_device)
        };
        let physical_device_properties =
            unsafe { instance.get_physical_device_properties(suitable_device.physical_device) };


        let font_texture = Texture::alloc(
            device,
            *graphics_queue,
            *transient_command_pool,
            physical_device_memory_properties,
            physical_device_properties,
            font_atlas_texture.width,
            font_atlas_texture.height,
            font_atlas_texture.data,
        )?;

        // init
        context.set_renderer_name(Some(String::from("ris_engine vulkan renderer")));

        Ok(Self {
            render_pass,
            pipeline_layout,
            descriptor_set_layout,
            pipeline,
            font_texture,
        })
    }

    pub fn draw(
        &mut self,
        renderer: &Renderer,
        target: vk::ImageView,
        data: &DrawData,
    ) -> RisResult<()>
    {
        let Renderer {
            instance,
            suitable_device,
            device,
            graphics_queue,
            transient_command_pool,
            swapchain: Swapchain {
                base: BaseSwapchain {
                    extent: swapchain_extent,
                    ..
                },
                ..
            },
            ..
        } = renderer;

        let physical_device_memory_properties = unsafe {
            instance.get_physical_device_memory_properties(suitable_device.physical_device)
        };
        let physical_device_properties =
            unsafe { instance.get_physical_device_properties(suitable_device.physical_device)
        };

        let fb_width = data.display_size[0] * data.framebuffer_scale[0];
        let fb_height = data.display_size[1] * data.framebuffer_scale[1];
        if fb_width <= 0.0 || fb_height <= 0.0 {
            return Ok(());
        }

        let left = data.display_pos[0];
        let right = data.display_pos[0] + data.display_size[0];
        let top = data.display_pos[1];
        let bottom = data.display_pos[1] + data.display_size[1];

        let mut pc = Mat4::init(1.);
        pc.0 .0 = 2. / (right - left);
        pc.1 .1 = 2. / (bottom - top);
        pc.2 .2 = -1.0;
        pc.3 .0 = (right + left) / (left - right);
        pc.3 .1 = (top + bottom) / (top - bottom);

        //let dimensions = match target.image().dimensions() {
        //    ImageDimensions::Dim2d { width, height, .. } => [width, height],
        //    dimensions => return ris_error::new_result!("bad image dimensions: {:?}", dimensions),
        //};

        //let viewport = Viewport {
        //    origin: [0.0, 0.0],
        //    dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        //    depth_range: 0.0..1.0,
        //};

        let clip_off = data.display_pos;
        let clip_scale = data.framebuffer_scale;

        //let layout = self.pipeline.layout().clone();

        let attachments = [target];

        let framebuffer_create_info = vk::FramebufferCreateInfo {
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

        let framebuffer = unsafe { device.create_framebuffer(&framebuffer_create_info, None) }?;

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

        let transient_command = TransientCommand::begin(
            device,
            *graphics_queue,
            *transient_command_pool,
        )?;

        unsafe {
            device.cmd_begin_render_pass(
                *transient_command.buffer(),
                &render_pass_begin_info,
                vk::SubpassContents::INLINE,
            )
        };

        for draw_list in data.draw_lists() {
            let vertices = draw_list.vtx_buffer();
            let indices = draw_list.idx_buffer();

            // vertex buffer
            let vertex_buffer_size = std::mem::size_of_val(vertices) as vk::DeviceSize;

            let vertex_buffer = Buffer::alloc(
                device,
                vertex_buffer_size,
                vk::BufferUsageFlags::VERTEX_BUFFER,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
                physical_device_memory_properties,
            )?;

            vertex_buffer.write(&device, &vertices)?;

            // index buffer
            let index_buffer_size = std::mem::size_of_val(indices) as vk::DeviceSize;

            let index_buffer = Buffer::alloc(
                device,
                index_buffer_size,
                vk::BufferUsageFlags::INDEX_BUFFER,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
                physical_device_memory_properties,
            )?;

            index_buffer.write(&device, &indices)?;

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
                        //    let scissor = Scissor {
                        //        origin: [
                        //            f32::max(0.0, clip_rect[0]).floor() as u32,
                        //            f32::max(0.0, clip_rect[1]).floor() as u32,
                        //        ],
                        //        dimensions: [
                        //            (clip_rect[2] - clip_rect[0]).abs().ceil() as u32,
                        //            (clip_rect[3] - clip_rect[1]).abs().ceil() as u32,
                        //        ],
                        //    };

                        //    let texture = self.lookup_texture(texture_id)?;

                        //    let descriptor_set_layout = layout.set_layouts().first().unroll()?;

                        //    let descriptor_set = PersistentDescriptorSet::new(
                        //        &allocators.descriptor_set,
                        //        descriptor_set_layout.clone(),
                        //        [WriteDescriptorSet::image_view_sampler(
                        //            0,
                        //            texture.0.clone(),
                        //            texture.1.clone(),
                        //        )],
                        //    )?;

                        //    command_buffer_builder
                        //        .bind_pipeline_graphics(self.pipeline.clone())
                        //        .set_viewport(0, [viewport.clone()])
                        //        .set_scissor(0, [scissor])
                        //        .bind_vertex_buffers(0, vertex_buffer.clone())
                        //        .bind_index_buffer(index_buffer.clone())
                        //        .bind_descriptor_sets(
                        //            vulkano::pipeline::PipelineBindPoint::Graphics,
                        //            layout.clone(),
                        //            0,
                        //            descriptor_set.clone(),
                        //        )
                        //        .push_constants(layout.clone(), 0, pc)
                        //        .draw_indexed(
                        //            count as u32,
                        //            1,
                        //            idx_offset as u32,
                        //            vtx_offset as i32,
                        //            0,
                        //        )
                        //        .map_err(|e| ris_error::new!("failed to draw: {}", e))?;
                        }
                    }
                    DrawCmd::ResetRenderState => {
                        ris_log::debug!("imgui reset render state");
                    },
                    DrawCmd::RawCallback { callback, raw_cmd } => unsafe {
                        ris_log::debug!("imgui raw callback");
                        callback(draw_list.raw(), raw_cmd)
                    },
                }
            }

            vertex_buffer.free(device);
            index_buffer.free(device);
        }

        unsafe {device.cmd_end_render_pass(*transient_command.buffer())};
        transient_command.end_and_submit()?;
    
        unsafe {device.destroy_framebuffer(framebuffer, None)};

        Ok(())
    }
}
