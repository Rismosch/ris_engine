use ash::vk;

use ris_debug::gizmo::GizmoTextVertex;
use ris_error::RisResult;
use ris_video_data::buffer::Buffer;
use ris_video_data::core::VulkanCore;
use ris_video_data::gpu_io;
use ris_video_data::gpu_io::GpuIOArgs;
use ris_video_data::image::TransitionLayoutInfo;
use ris_video_data::texture::Texture;
use ris_video_data::texture::TextureCreateInfo;
use ris_video_data::transient_command::TransientCommandArgs;

pub struct GizmoTextMesh {
    pub vertices: Buffer,
    pub vertex_count: usize,
    pub text_texture: Texture,
    pub text_len: usize,
}

impl GizmoTextMesh {
    /// # Safety
    ///
    /// - May only be called once. Memory must not be freed twice.
    /// - This object must not be used after it was freed
    pub unsafe fn free(&mut self, device: &ash::Device) {
        self.vertices.free(device);
        self.text_texture.free(device);
    }

    pub fn alloc(core: &VulkanCore, vertices: &[GizmoTextVertex], text: &[u8]) -> RisResult<Self> {
        let VulkanCore {
            instance,
            suitable_device,
            device,
            graphics_queue,
            transient_command_pool,
            ..
        } = core;

        ris_error::debug_assert!(text.len().is_multiple_of(4))?;

        let physical_device_memory_properties = unsafe {
            instance.get_physical_device_memory_properties(suitable_device.physical_device)
        };
        let physical_device_properties =
            unsafe { instance.get_physical_device_properties(suitable_device.physical_device) };

        let tcas = TransientCommandArgs {
            device: device.clone(),
            queue: *graphics_queue,
            command_pool: *transient_command_pool,
        };

        let vertex_buffer_size = std::mem::size_of_val(vertices);
        let text_data_size = std::mem::size_of_val(text);
        let staging_size = usize::max(vertex_buffer_size, text_data_size);
        let staging =
            Buffer::alloc_staging(device, staging_size, physical_device_memory_properties)?;

        let vertex_buffer = Buffer::alloc(
            device,
            vertex_buffer_size,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::DEVICE_LOCAL,
            physical_device_memory_properties,
        )?;

        unsafe { gpu_io::write_to_memory(device, vertices, vertex_buffer.memory) }?;

        let text_texture = Texture::alloc(TextureCreateInfo {
            transient_command_args: tcas.clone(),
            staging: &staging,
            physical_device_memory_properties,
            physical_device_properties,
            width: (text.len() / 4),
            height: 1,
            format: vk::Format::R8G8B8A8_UINT,
            filter: vk::Filter::NEAREST,
            pixels: text,
        })?;

        unsafe { staging.free(device) };

        Ok(Self {
            vertices: vertex_buffer,
            vertex_count: vertices.len(),
            text_texture,
            text_len: text.len(),
        })
    }

    pub fn update(
        &mut self,
        core: &VulkanCore,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        physical_device_properties: vk::PhysicalDeviceProperties,
        vertices: &[GizmoTextVertex],
        text: &[u8],
    ) -> RisResult<()> {
        let VulkanCore {
            device,
            graphics_queue,
            transient_command_pool,
            ..
        } = core;

        ris_error::debug_assert!(text.len().is_multiple_of(4))?;

        let tcas = TransientCommandArgs {
            device: device.clone(),
            queue: *graphics_queue,
            command_pool: *transient_command_pool,
        };

        if self.vertex_count < vertices.len() {
            self.vertex_count = vertices.len();
            let vertex_buffer_size = std::mem::size_of_val(vertices);
            self.vertices.resize(
                vertex_buffer_size,
                device,
                physical_device_memory_properties,
            )?;
        }
        unsafe { gpu_io::write_to_memory(device, vertices, self.vertices.memory) }?;

        let staging = Buffer::alloc_staging(device, text.len(), physical_device_memory_properties)?;

        if self.text_len < text.len() {
            self.text_len = text.len();

            let new_text_texture = Texture::alloc(TextureCreateInfo {
                transient_command_args: tcas.clone(),
                staging: &staging,
                physical_device_memory_properties,
                physical_device_properties,
                width: text.len() / 4,
                height: 1,
                format: vk::Format::R8G8B8A8_UINT,
                filter: vk::Filter::NEAREST,
                pixels: text,
            })?;

            self.text_len = text.len();

            let old_texture = self.text_texture;
            self.text_texture = new_text_texture;

            unsafe { old_texture.free(device) };
        } else {
            let mut image = self.text_texture.image;

            let fence_create_info = vk::FenceCreateInfo {
                s_type: vk::StructureType::FENCE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: vk::FenceCreateFlags::empty(),
            };
            let fence = unsafe { device.create_fence(&fence_create_info, None) }?;

            image.transition_layout(TransitionLayoutInfo {
                transient_command_args: tcas.clone(),
                new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                fence: Some(fence),
            })?;

            unsafe {
                device.reset_fences(&[fence])?;

                gpu_io::write_to_image(GpuIOArgs {
                    transient_command_args: tcas.clone(),
                    values: text,
                    gpu_object: &image,
                    staging: &staging,
                })?;
            }

            image.transition_layout(TransitionLayoutInfo {
                transient_command_args: tcas.clone(),
                new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                fence: Some(fence),
            })?;

            unsafe {
                device.destroy_fence(fence, None);
            }
        }

        unsafe { staging.free(device) };

        Ok(())
    }
}
