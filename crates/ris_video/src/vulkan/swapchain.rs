use std::ptr;

use ash::vk;

use ris_asset::AssetId;
use ris_error::RisResult;

use super::buffer::Buffer;
use super::frame_in_flight::FrameInFlight;
use super::graphics_pipeline::GraphicsPipeline;
use super::graphics_pipeline::GraphicsPipelineCreateInfo;
use super::image::Image;
use super::image::ImageCreateInfo;
use super::renderer::Renderer;
use super::suitable_device::SuitableDevice;
use super::surface_details::SurfaceDetails;
use super::texture::Texture;
use super::transient_command::TransientCommandSync;
use super::uniform_buffer_object::UniformBufferObject;
use super::util;

pub struct BaseSwapchain {
    pub format: vk::SurfaceFormatKHR,
    pub extent: vk::Extent2D,
    pub loader: ash::extensions::khr::Swapchain,
    pub swapchain: vk::SwapchainKHR,
}

pub struct Swapchain {
    pub base: BaseSwapchain,
    pub graphics_pipeline: GraphicsPipeline,
    pub depth_image: Image,
    pub depth_image_view: vk::ImageView,
    pub entries: Vec<SwapchainEntry>,
    pub frames_in_flight: Option<Vec<FrameInFlight>>,
    command_buffers: Vec<vk::CommandBuffer>,
}

pub struct SwapchainEntry {
    pub image: vk::Image,
    pub image_view: vk::ImageView,
    pub uniform_buffer: Buffer,
    pub uniform_buffer_mapped: *mut UniformBufferObject,
    pub descriptor_set: vk::DescriptorSet,
    pub framebuffer: vk::Framebuffer,
    pub command_buffer: vk::CommandBuffer,
}

pub struct SwapchainCreateInfo<'a> {
    pub instance: &'a ash::Instance,
    pub suitable_device: &'a SuitableDevice,
    pub device: &'a ash::Device,
    pub graphics_queue: vk::Queue,
    pub command_pool: vk::CommandPool,
    pub transient_command_pool: vk::CommandPool,
    pub descriptor_set_layout: vk::DescriptorSetLayout,
    pub descriptor_pool: vk::DescriptorPool,
    pub texture: &'a Texture,
    pub vertex_buffer: &'a Buffer,
    pub index_buffer: &'a Buffer,
    pub base: BaseSwapchain,
    pub images: Vec<vk::Image>,
    pub descriptor_sets: Option<Vec<vk::DescriptorSet>>,
    pub frames_in_flight: Option<Vec<FrameInFlight>>,
    pub vs_asset_id: AssetId,
    pub fs_asset_id: AssetId,
}

impl BaseSwapchain {
    /// # Safety
    ///
    /// `free()` must be called, or you are leaking memory.
    pub unsafe fn alloc(
        instance: &ash::Instance,
        surface_loader: &ash::extensions::khr::Surface,
        surface: &vk::SurfaceKHR,
        suitable_device: &SuitableDevice,
        device: &ash::Device,
        window_size: (u32, u32),
    ) -> RisResult<(Self, Vec<vk::Image>)> {
        let SurfaceDetails {
            capabilities,
            formats,
            present_modes,
        } = SurfaceDetails::query(surface_loader, suitable_device.physical_device, *surface)?;

        let preferred_surface_format = formats.iter().find(|x| {
            x.format == super::PREFERRED_FORMAT && x.color_space == super::PREFERRED_COLOR_SPACE
        });
        let format = match preferred_surface_format {
            Some(format) => *format,
            None => formats[0],
        };

        let preferred_surface_present_mode = present_modes
            .iter()
            .find(|&&x| x == super::PREFERRED_PRESENT_MODE);
        let surface_present_mode = match preferred_surface_present_mode {
            Some(present_mode) => present_mode,
            None => &present_modes[0],
        };

        let extent = if capabilities.current_extent.width != u32::MAX {
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

            vk::Extent2D { width, height }
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
            if suitable_device.graphics_queue_family != suitable_device.present_queue_family {
                (
                    vk::SharingMode::CONCURRENT,
                    2,
                    vec![
                        suitable_device.graphics_queue_family,
                        suitable_device.present_queue_family,
                    ],
                )
            } else {
                (vk::SharingMode::EXCLUSIVE, 0, vec![])
            };

        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: vk::SwapchainCreateFlagsKHR::empty(),
            surface: *surface,
            min_image_count: swapchain_image_count,
            image_color_space: format.color_space,
            image_format: format.format,
            image_extent: extent,
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

        let loader = ash::extensions::khr::Swapchain::new(instance, device);
        let swapchain = unsafe { loader.create_swapchain(&swapchain_create_info, None) }?;

        // images
        let images = unsafe { loader.get_swapchain_images(swapchain) }?;

        Ok((
            Self {
                format,
                extent,
                loader,
                swapchain,
            },
            images,
        ))
    }

    /// # Safety
    ///
    /// Must only be called once. Memory must not be freed twice.
    pub unsafe fn free(&self) {
        unsafe {
            self.loader.destroy_swapchain(self.swapchain, None);
        }
    }
}

impl Swapchain {
    /// # Safety
    ///
    /// `free()` must be called, or you are leaking memory.
    pub unsafe fn alloc(info: SwapchainCreateInfo) -> RisResult<Self> {
        let SwapchainCreateInfo {
            instance,
            suitable_device,
            device,
            graphics_queue,
            command_pool,
            transient_command_pool,
            descriptor_set_layout,
            descriptor_pool,
            texture,
            vertex_buffer,
            index_buffer,
            base,
            images,
            descriptor_sets,
            frames_in_flight,
            vs_asset_id,
            fs_asset_id,
        } = info;

        // graphics pipeline
        let graphics_pipeline = GraphicsPipeline::alloc(GraphicsPipelineCreateInfo {
            instance,
            physical_device: suitable_device.physical_device,
            device,
            color_format: base.format.format,
            swapchain_extent: base.extent,
            descriptor_set_layout,
            vs_asset_id,
            fs_asset_id,
        })?;

        // depth buffer
        let depth_format = util::find_depth_format(instance, suitable_device.physical_device)?;

        let physical_device_memory_properties = unsafe {
            instance.get_physical_device_memory_properties(suitable_device.physical_device)
        };

        let depth_image = Image::alloc(ImageCreateInfo {
            device,
            width: base.extent.width,
            height: base.extent.height,
            format: depth_format,
            tiling: vk::ImageTiling::OPTIMAL,
            usage: vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            memory_property_flags: vk::MemoryPropertyFlags::DEVICE_LOCAL,
            physical_device_memory_properties,
        })?;

        let depth_image_view = Image::alloc_view(
            device,
            depth_image.image,
            depth_format,
            vk::ImageAspectFlags::DEPTH,
        )?;

        depth_image.transition_layout(
            device,
            graphics_queue,
            transient_command_pool,
            depth_format,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            TransientCommandSync::default(),
        )?;

        // swapchain entries
        let swapchain_entry_count = images.len();

        // descriptor sets
        let descriptor_sets = match descriptor_sets {
            Some(x) => x,
            None => {
                let mut descriptor_set_layouts = Vec::with_capacity(swapchain_entry_count);
                for _ in 0..swapchain_entry_count {
                    descriptor_set_layouts.push(descriptor_set_layout);
                }

                let descriptor_set_allocate_info = vk::DescriptorSetAllocateInfo {
                    s_type: vk::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
                    p_next: ptr::null(),
                    descriptor_pool,
                    descriptor_set_count: descriptor_set_layouts.len() as u32,
                    p_set_layouts: descriptor_set_layouts.as_ptr(),
                };

                unsafe { device.allocate_descriptor_sets(&descriptor_set_allocate_info) }?
            }
        };

        // command buffers
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_pool,
            level: vk::CommandBufferLevel::PRIMARY,
            command_buffer_count: swapchain_entry_count as u32,
        };

        let command_buffers =
            unsafe { device.allocate_command_buffers(&command_buffer_allocate_info) }?;

        let mut entries = Vec::with_capacity(swapchain_entry_count);
        for (i, image) in images.into_iter().enumerate() {
            // swapchain image view
            let image_view = Image::alloc_view(
                device,
                image,
                base.format.format,
                vk::ImageAspectFlags::COLOR,
            )?;

            // uniform buffer
            let uniform_buffer_size = std::mem::size_of::<UniformBufferObject>() as vk::DeviceSize;
            let uniform_buffer = Buffer::alloc(
                device,
                uniform_buffer_size,
                vk::BufferUsageFlags::UNIFORM_BUFFER,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
                physical_device_memory_properties,
            )?;
            let uniform_buffer_mapped = unsafe {
                device.map_memory(
                    uniform_buffer.memory,
                    0,
                    uniform_buffer_size,
                    vk::MemoryMapFlags::empty(),
                )
            }? as *mut UniformBufferObject;

            // descriptor set
            let descriptor_set = descriptor_sets[i];

            let descriptor_buffer_info = [vk::DescriptorBufferInfo {
                buffer: uniform_buffer.buffer,
                offset: 0,
                range: std::mem::size_of::<UniformBufferObject>() as vk::DeviceSize,
            }];

            let descriptor_image_info = [vk::DescriptorImageInfo {
                sampler: texture.sampler,
                image_view: texture.view,
                image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            }];

            let write_descriptor_sets = [
                vk::WriteDescriptorSet {
                    s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
                    p_next: ptr::null(),
                    dst_set: descriptor_set,
                    dst_binding: 0,
                    dst_array_element: 0,
                    descriptor_count: 1,
                    descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                    p_image_info: ptr::null(),
                    p_buffer_info: descriptor_buffer_info.as_ptr(),
                    p_texel_buffer_view: ptr::null(),
                },
                vk::WriteDescriptorSet {
                    s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
                    p_next: ptr::null(),
                    dst_set: descriptor_set,
                    dst_binding: 1,
                    dst_array_element: 0,
                    descriptor_count: 1,
                    descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                    p_image_info: descriptor_image_info.as_ptr(),
                    p_buffer_info: ptr::null(),
                    p_texel_buffer_view: ptr::null(),
                },
            ];

            unsafe { device.update_descriptor_sets(&write_descriptor_sets, &[]) };

            // frame buffer
            let attachments = [image_view, depth_image_view];

            let framebuffer_create_info = vk::FramebufferCreateInfo {
                s_type: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::FramebufferCreateFlags::empty(),
                render_pass: graphics_pipeline.render_pass,
                attachment_count: attachments.len() as u32,
                p_attachments: attachments.as_ptr(),
                width: base.extent.width,
                height: base.extent.height,
                layers: 1,
            };

            let framebuffer = unsafe { device.create_framebuffer(&framebuffer_create_info, None) }?;

            // command buffer
            let command_buffer = command_buffers[i];

            let command_buffer_reset_flags = vk::CommandBufferResetFlags::empty();
            unsafe { device.reset_command_buffer(command_buffer, command_buffer_reset_flags) }?;

            let command_buffer_begin_info = vk::CommandBufferBeginInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
                p_next: ptr::null(),
                flags: vk::CommandBufferUsageFlags::empty(),
                p_inheritance_info: ptr::null(),
            };

            unsafe { device.begin_command_buffer(command_buffer, &command_buffer_begin_info) }?;

            let clear_values = [
                vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 0.0],
                    },
                },
                vk::ClearValue {
                    depth_stencil: vk::ClearDepthStencilValue {
                        depth: 1.0,
                        stencil: 0,
                    },
                },
            ];

            let render_pass_begin_info = vk::RenderPassBeginInfo {
                s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
                p_next: ptr::null(),
                render_pass: graphics_pipeline.render_pass,
                framebuffer,
                render_area: vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: base.extent,
                },
                clear_value_count: clear_values.len() as u32,
                p_clear_values: clear_values.as_ptr(),
            };

            unsafe {
                device.cmd_begin_render_pass(
                    command_buffer,
                    &render_pass_begin_info,
                    vk::SubpassContents::INLINE,
                )
            };
            unsafe {
                device.cmd_bind_pipeline(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    graphics_pipeline.pipeline,
                )
            };

            let vertex_buffers = [vertex_buffer.buffer];
            let offsets = [0_u64];
            unsafe { device.cmd_bind_vertex_buffers(command_buffer, 0, &vertex_buffers, &offsets) };
            unsafe {
                device.cmd_bind_index_buffer(
                    command_buffer,
                    index_buffer.buffer,
                    0,
                    vk::IndexType::UINT32,
                )
            };
            let descriptor_sets = [descriptor_set];
            unsafe {
                device.cmd_bind_descriptor_sets(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    graphics_pipeline.layout,
                    0,
                    &descriptor_sets,
                    &[],
                )
            };

            let index_count = super::INDICES.len() as u32;
            unsafe { device.cmd_draw_indexed(command_buffer, index_count, 1, 0, 0, 0) };
            unsafe { device.cmd_end_render_pass(command_buffer) };
            unsafe { device.end_command_buffer(command_buffer) }?;

            // entry
            let swapchain_entry = SwapchainEntry {
                image,
                image_view,
                uniform_buffer,
                uniform_buffer_mapped,
                descriptor_set,
                framebuffer,
                command_buffer,
            };

            entries.push(swapchain_entry);
        } // end swapchain entries

        // frames in flight
        let frames_in_flight = match frames_in_flight {
            Some(x) => Some(x),
            None => {
                let mut frames_in_flight = Vec::with_capacity(command_buffers.len());
                for _ in 0..super::MAX_FRAMES_IN_FLIGHT {
                    let frame_in_flight = FrameInFlight::alloc(device)?;

                    frames_in_flight.push(frame_in_flight);
                }

                Some(frames_in_flight)
            }
        };

        Ok(Self {
            base,
            graphics_pipeline,
            depth_image,
            depth_image_view,
            entries,
            frames_in_flight,
            command_buffers,
        })
    }

    /// # Safety
    ///
    /// Must only be called once. Memory must not be freed twice.
    pub unsafe fn free(&mut self, device: &ash::Device, command_pool: vk::CommandPool) {
        unsafe {
            self.depth_image.free(device);
            device.destroy_image_view(self.depth_image_view, None);

            device.free_command_buffers(command_pool, &self.command_buffers);

            if let Some(frames_in_flight) = self.frames_in_flight.take() {
                for frame_in_flight in frames_in_flight.iter() {
                    frame_in_flight.free(device);
                }
            }

            for entry in self.entries.iter() {
                device.destroy_framebuffer(entry.framebuffer, None);
                entry.uniform_buffer.free(device);
                device.destroy_image_view(entry.image_view, None);
            }

            self.graphics_pipeline.free(device);

            self.base.free();
        }
    }

    pub fn recreate(
        renderer: &mut Renderer,
        window_size: (u32, u32),
        vs_asset_id: AssetId,
        fs_asset_id: AssetId,
    ) -> RisResult<Self> {
        // gather data which should not be cleaned up
        let mut descriptor_sets = Vec::with_capacity(renderer.swapchain.entries.len());
        for entry in renderer.swapchain.entries.iter() {
            let descriptor_set = entry.descriptor_set;
            descriptor_sets.push(descriptor_set);
        }

        let descriptor_sets = Some(descriptor_sets);
        let frames_in_flight = renderer.swapchain.frames_in_flight.take();

        // free old swapchain
        unsafe {
            renderer
                .swapchain
                .free(&renderer.device, renderer.command_pool)
        };

        // create new swapchain
        let (base, images) = unsafe {
            BaseSwapchain::alloc(
                &renderer.instance,
                &renderer.surface_loader,
                &renderer.surface,
                &renderer.suitable_device,
                &renderer.device,
                window_size,
            )
        }?;

        unsafe {
            Self::alloc(SwapchainCreateInfo {
                instance: &renderer.instance,
                suitable_device: &renderer.suitable_device,
                device: &renderer.device,
                graphics_queue: renderer.graphics_queue,
                command_pool: renderer.command_pool,
                transient_command_pool: renderer.transient_command_pool,
                descriptor_set_layout: renderer.descriptor_set_layout,
                descriptor_pool: renderer.descriptor_pool,
                texture: &renderer.texture,
                vertex_buffer: &renderer.vertex_buffer,
                index_buffer: &renderer.index_buffer,
                base,
                images,
                descriptor_sets,
                frames_in_flight,
                vs_asset_id,
                fs_asset_id,
            })
        }
    }
}
