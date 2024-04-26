use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

use ash::vk;

use ris_asset::AssetId;
use ris_error::Extensions;
use ris_error::RisResult;

use super::suitable_device::SuitableDevice;
use super::surface_details::SurfaceDetails;
use super::vertex::Vertex;

pub struct SwapchainObjects {
    pub swapchain_loader: ash::extensions::khr::Swapchain,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_format: vk::SurfaceFormatKHR,
    pub swapchain_extent: vk::Extent2D,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_image_views: Vec<vk::ImageView>,
    pub render_pass: vk::RenderPass,
    pub pipeline_layout: vk::PipelineLayout,
    pub graphics_pipeline: vk::Pipeline,
    pub framebuffers: Vec<vk::Framebuffer>,
}

impl SwapchainObjects {
    pub fn cleanup(&mut self, device: &ash::Device) {
        unsafe {
            for &framebuffer in self.framebuffers.iter() {
                device.destroy_framebuffer(framebuffer, None);
            }

            device.destroy_pipeline(self.graphics_pipeline, None);
            device.destroy_pipeline_layout(self.pipeline_layout, None);
            device.destroy_render_pass(self.render_pass, None);

            for &swapchain_image_view in self.swapchain_image_views.iter() {
                device.destroy_image_view(swapchain_image_view, None);
            }

            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
        }
    }

    pub fn create(
        instance: &ash::Instance,
        surface_loader: &ash::extensions::khr::Surface,
        surface: &vk::SurfaceKHR,
        device: &ash::Device,
        suitable_device: &SuitableDevice,
        descriptor_set_layout: &vk::DescriptorSetLayout,
        window_size: (u32, u32),
    ) -> RisResult<Self> {
        let SurfaceDetails{
            capabilities,
            formats,
            present_modes,
        } = SurfaceDetails::query(
            &surface_loader,
            suitable_device.physical_device,
            *surface,
        )?;

        // swap chain
        let preferred_surface_format = formats
            .iter()
            .find(|x| x.format == super::PREFERRED_FORMAT && x.color_space == super::PREFERRED_COLOR_SPACE);
        let surface_format = match preferred_surface_format {
            Some(format) => format,
            // getting the first format if the preferred format does not exist. this should not
            // cause ub, becuase we checked if the list is empty at finding the suitable device.
            None => &formats[0],
        };

        let preferred_surface_present_mode = present_modes
            .iter()
            .find(|&&x| x == super::PREFERRED_PRESENT_MODE);
        let surface_present_mode = match preferred_surface_present_mode {
            Some(present_mode) => present_mode,
            // getting the first present mode if the preferred format does not exist. this should
            // not cause ub, because we checked if the list is empty at finding the suitable device.
            None => unsafe{present_modes.get_unchecked(0)},
        };

        let swapchain_extent = if capabilities.current_extent.width != u32::MAX {
            capabilities.current_extent
        } else {
            let (window_width, window_height) = window_size;
            let width = window_width.clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
            );
            let height = window_height.clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
            );

            vk::Extent2D {
                width,
                height,
            }
        };

        let preferred_swapchain_image_count = capabilities.min_image_count + 1;
        let swapchain_image_count = if capabilities.max_image_count == 0 {
            // SurfaceCapabilitiesKHR == 0 indicates there is no maximum
            preferred_swapchain_image_count
        } else {
            u32::min(
                preferred_swapchain_image_count,
                capabilities.max_image_count,
            )
        };

        let (image_sharing_mode, queue_family_index_count, queue_family_indices) = 
            if suitable_device.graphics_queue_family != suitable_device.present_queue_family {(
                vk::SharingMode::CONCURRENT,
                2,
                vec![
                suitable_device.graphics_queue_family,
                suitable_device.present_queue_family,
                ],
            )} else {(
                vk::SharingMode::EXCLUSIVE,
                0,
                vec![],
            )};

        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: vk::SwapchainCreateFlagsKHR::empty(),
            surface: *surface,
            min_image_count: swapchain_image_count,
            image_color_space: surface_format.color_space,
            image_format: surface_format.format,
            image_extent: swapchain_extent,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode,
            p_queue_family_indices: queue_family_indices.as_ptr(),
            queue_family_index_count,
            pre_transform: capabilities.current_transform,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            present_mode: *surface_present_mode,
            clipped: vk::TRUE,
            old_swapchain: vk::SwapchainKHR::null(),
            image_array_layers: 1,
        };

        let swapchain_loader = ash::extensions::khr::Swapchain::new(&instance, &device);
        let swapchain = unsafe {
            swapchain_loader.create_swapchain(&swapchain_create_info, None)
        }?;

        let swapchain_images = unsafe {
            swapchain_loader.get_swapchain_images(swapchain)
        }?;

        // image views
        let mut swapchain_image_views = Vec::new();
        for swapchain_image in swapchain_images.iter() {
            let image_view_create_info = vk::ImageViewCreateInfo {
                s_type: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::ImageViewCreateFlags::empty(),
                image: *swapchain_image,
                view_type: vk::ImageViewType::TYPE_2D,
                format: surface_format.format,
                components: vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                },
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
            };

            let image_view = unsafe {
                device.create_image_view(&image_view_create_info, None)
            }?;

            swapchain_image_views.push(image_view);
        }

        // graphics pipeline
        // render pass
        let color_attachment_descriptions = [vk::AttachmentDescription {
            flags: vk::AttachmentDescriptionFlags::empty(),
            format: surface_format.format,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
        }];

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
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: vk::AccessFlags::empty(),
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dependency_flags: vk::DependencyFlags::empty(),
        }];

        let render_pass_create_info = vk::RenderPassCreateInfo {
            s_type: vk::StructureType::RENDER_PASS_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::RenderPassCreateFlags::empty(),
            attachment_count: color_attachment_descriptions.len() as u32,
            p_attachments: color_attachment_descriptions.as_ptr(),
            subpass_count: subpass_descriptions.len() as u32,
            p_subpasses: subpass_descriptions.as_ptr(),
            dependency_count: supbass_dependencies.len() as u32,
            p_dependencies: supbass_dependencies.as_ptr(),
        };

        let render_pass = unsafe{device.create_render_pass(&render_pass_create_info, None)}?;

        // graphics pipeline
        // shaders
        let vs_asset_id = AssetId::Directory(String::from("__imported_raw/shaders/default.vert.spv"));
        let fs_asset_id = AssetId::Directory(String::from("__imported_raw/shaders/default.frag.spv"));

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

        let vs_shader_module = unsafe{device.create_shader_module(&vs_shader_module_create_info, None)}?;
        let fs_shader_module = unsafe{device.create_shader_module(&fs_shader_module_create_info, None)}?;

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

        // graphics pipeline
        // pipeline
        let vertex_binding_descriptions = Vertex::get_binding_descriptions();
        let vertex_attribute_descriptions = Vertex::get_attribute_descriptions();

        let pipeline_vertex_input_state_create_info = [vk::PipelineVertexInputStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineVertexInputStateCreateFlags::empty(),
            vertex_binding_description_count: vertex_binding_descriptions.len() as u32,
            p_vertex_binding_descriptions: vertex_binding_descriptions.as_ptr(),
            vertex_attribute_description_count: vertex_attribute_descriptions.len() as u32,
            p_vertex_attribute_descriptions: vertex_attribute_descriptions.as_ptr(),
        }];

        let pipeline_input_assembly_state_info = [vk::PipelineInputAssemblyStateCreateInfo {
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
            offset: vk::Offset2D{x: 0, y: 0},
            extent: swapchain_extent,
        }];

        let pipeline_viewport_state_create_info = [vk::PipelineViewportStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineViewportStateCreateFlags::empty(),
            viewport_count: 1,
            p_viewports: viewports.as_ptr(),
            scissor_count: 1,
            p_scissors: scissors.as_ptr(),
        }];

        let pipeline_rasterization_state_create_info = [vk::PipelineRasterizationStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineRasterizationStateCreateFlags::empty(),
            depth_clamp_enable: vk::FALSE,
            rasterizer_discard_enable: vk::FALSE,
            polygon_mode: vk::PolygonMode::FILL,
            cull_mode: vk::CullModeFlags::BACK,
            front_face: vk::FrontFace::CLOCKWISE,
            depth_bias_enable: vk::FALSE,
            depth_bias_constant_factor: 0.,
            depth_bias_clamp: 0.,
            depth_bias_slope_factor: 0.,
            line_width: 1.,
        }];

        let pipeline_multisample_state_create_info = [vk::PipelineMultisampleStateCreateInfo {
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

        let stencil_op_state = vk::StencilOpState {
            fail_op: vk::StencilOp::KEEP,
            pass_op: vk::StencilOp::KEEP,
            depth_fail_op: vk::StencilOp::KEEP,
            compare_op: vk::CompareOp::ALWAYS,
            compare_mask: 0,
            write_mask: 0,
            reference: 0,
        };

        let pipeline_depth_stencil_state_create_info = vk::PipelineDepthStencilStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineDepthStencilStateCreateFlags::empty(),
            depth_test_enable: vk::FALSE,
            depth_write_enable: vk::FALSE,
            depth_compare_op: vk::CompareOp::LESS_OR_EQUAL,
            depth_bounds_test_enable: vk::FALSE,
            stencil_test_enable: vk::FALSE,
            front: stencil_op_state,
            back: stencil_op_state,
            min_depth_bounds: 0.,
            max_depth_bounds: 1.,
        };

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

        let pipeline_color_blend_attachment_states = [vk::PipelineColorBlendAttachmentState{
            blend_enable: vk::TRUE,
            src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ONE,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
            color_write_mask: vk::ColorComponentFlags::RGBA,
        }];

        let pipeline_color_blend_state_create_info = [vk::PipelineColorBlendStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineColorBlendStateCreateFlags::empty(),
            logic_op_enable: vk::FALSE,
            logic_op: vk::LogicOp::COPY,
            attachment_count: pipeline_color_blend_attachment_states.len() as u32,
            p_attachments: pipeline_color_blend_attachment_states.as_ptr(),
            blend_constants: [0., 0., 0., 0.,],
        }];

        let descriptor_set_layouts = [*descriptor_set_layout];

        let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo {
            s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: descriptor_set_layouts.len() as u32,
            p_set_layouts: descriptor_set_layouts.as_ptr(),
            push_constant_range_count: 0,
            p_push_constant_ranges: ptr::null(),
        };

        let pipeline_layout = unsafe{device.create_pipeline_layout(&pipeline_layout_create_info, None)}?;
        
        // graphic pipeline
        // creation
        let graphics_pipeline_create_info = [vk::GraphicsPipelineCreateInfo {
            s_type: vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineCreateFlags::empty(),
            stage_count: shader_stages.len() as u32,
            p_stages: shader_stages.as_ptr(),
            p_vertex_input_state: pipeline_vertex_input_state_create_info.as_ptr(),
            p_input_assembly_state: pipeline_input_assembly_state_info.as_ptr(),
            p_tessellation_state: ptr::null(),
            p_viewport_state: pipeline_viewport_state_create_info.as_ptr(),
            p_rasterization_state: pipeline_rasterization_state_create_info.as_ptr(),
            p_multisample_state: pipeline_multisample_state_create_info.as_ptr(),
            p_depth_stencil_state: ptr::null(),
            p_color_blend_state: pipeline_color_blend_state_create_info.as_ptr(),
            p_dynamic_state: ptr::null(),
            layout: pipeline_layout,
            render_pass,
            subpass: 0,
            base_pipeline_handle: vk::Pipeline::null(),
            base_pipeline_index: -1,
        }];

        let graphics_pipelines = unsafe{device.create_graphics_pipelines(
            vk::PipelineCache::null(),
            &graphics_pipeline_create_info,
            None,
        )}.map_err(|e| e.1)?;
        let graphics_pipeline = graphics_pipelines.into_iter().next().unroll()?;

        unsafe {device.destroy_shader_module(vs_shader_module, None)};
        unsafe {device.destroy_shader_module(fs_shader_module, None)};

        // frame buffers
        let mut framebuffers = Vec::with_capacity(swapchain_image_views.len());
        for &swapchain_image_view in swapchain_image_views.iter() {
            let image_view = [swapchain_image_view];

            let framebuffer_create_info = vk::FramebufferCreateInfo {
                s_type: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::FramebufferCreateFlags::empty(),
                render_pass, 
                attachment_count: image_view.len() as u32,
                p_attachments: image_view.as_ptr(),
                width: swapchain_extent.width,
                height: swapchain_extent.height,
                layers: 1,
            };

            let framebuffer = unsafe{device.create_framebuffer(&framebuffer_create_info, None)}?;
            framebuffers.push(framebuffer);
        }

        Ok(Self{
            swapchain_loader,
            swapchain,
            swapchain_format: *surface_format,
            swapchain_extent,
            swapchain_images,
            swapchain_image_views,
            render_pass,
            pipeline_layout,
            graphics_pipeline,
            framebuffers,
        })
    }
}
