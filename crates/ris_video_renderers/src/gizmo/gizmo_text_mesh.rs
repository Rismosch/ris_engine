use ash::vk;

use ris_debug::gizmo::GizmoTextVertex;
use ris_error::RisResult;
use ris_video_data::buffer::Buffer;
use ris_video_data::buffer::CopyToImageInfo;
use ris_video_data::core::VulkanCore;
use ris_video_data::image::TransitionLayoutInfo;
use ris_video_data::texture::Texture;
use ris_video_data::texture::TextureCreateInfo;
use ris_video_data::transient_command::TransientCommandSync;

pub struct GizmoTextMesh {
    pub vertices: Buffer,
    pub vertex_count: usize,
    pub text_texture: Texture,
    pub text_len: usize,
}

impl GizmoTextMesh {
    /// # Safety
    ///
    /// May only be called once. Memory must not be freed twice.
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

        let vertex_buffer_size = std::mem::size_of_val(vertices) as vk::DeviceSize;
        let vertex_buffer = Buffer::alloc(
            device,
            vertex_buffer_size,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE
                | vk::MemoryPropertyFlags::HOST_COHERENT
                | vk::MemoryPropertyFlags::DEVICE_LOCAL,
            physical_device_memory_properties,
        )?;

        unsafe { vertex_buffer.write(device, vertices) }?;

        let text_texture = Texture::alloc(TextureCreateInfo {
            device,
            queue: *graphics_queue,
            transient_command_pool: *transient_command_pool,
            physical_device_memory_properties,
            physical_device_properties,
            width: (text.len() / 4) as u32,
            height: 1,
            format: vk::Format::R8G8B8A8_UINT,
            filter: vk::Filter::NEAREST,
            pixels_rgba: text,
        })?;

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

        let old_vertex_count = self.vertex_count;
        let new_vertex_count = vertices.len();

        if old_vertex_count < new_vertex_count {
            let vertex_buffer_size = std::mem::size_of_val(vertices) as vk::DeviceSize;
            let new_vertex_buffer = Buffer::alloc(
                device,
                vertex_buffer_size,
                vk::BufferUsageFlags::VERTEX_BUFFER,
                vk::MemoryPropertyFlags::HOST_VISIBLE
                    | vk::MemoryPropertyFlags::HOST_COHERENT
                    | vk::MemoryPropertyFlags::DEVICE_LOCAL,
                physical_device_memory_properties,
            )?;

            self.vertex_count = vertices.len();

            let old_buffer = self.vertices;
            self.vertices = new_vertex_buffer;

            unsafe { old_buffer.free(device) };
        }
        unsafe { self.vertices.write(device, vertices) }?;

        let old_text_len = self.text_len;
        let new_text_len = text.len();

        if old_text_len < new_text_len {
            let new_text_texture = Texture::alloc(TextureCreateInfo {
                device,
                queue: *graphics_queue,
                transient_command_pool: *transient_command_pool,
                physical_device_memory_properties,
                physical_device_properties,
                width: (text.len() / 4) as u32,
                height: 1,
                format: vk::Format::R8G8B8A8_UINT,
                filter: vk::Filter::NEAREST,
                pixels_rgba: text,
            })?;

            self.text_len = text.len();

            let old_texture = self.text_texture;
            self.text_texture = new_text_texture;

            unsafe { old_texture.free(device) };
        } else {
            unsafe {
                let image = self.text_texture.image;

                let staging_buffer = Buffer::alloc(
                    device,
                    text.len() as vk::DeviceSize,
                    vk::BufferUsageFlags::TRANSFER_SRC,
                    vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
                    physical_device_memory_properties,
                )?;

                staging_buffer.write(device, text)?;

                image.transition_layout(TransitionLayoutInfo {
                    device,
                    queue: *graphics_queue,
                    transient_command_pool: *transient_command_pool,
                    format: vk::Format::R8G8B8A8_UINT,
                    old_layout: vk::ImageLayout::UNDEFINED,
                    new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    sync: TransientCommandSync::default(),
                })?;

                staging_buffer.copy_to_image(CopyToImageInfo {
                    device,
                    queue: *graphics_queue,
                    transient_command_pool: *transient_command_pool,
                    image: image.image,
                    width: (text.len() / 4) as u32,
                    height: 1,
                    sync: TransientCommandSync::default(),
                })?;

                image.transition_layout(TransitionLayoutInfo {
                    device,
                    queue: *graphics_queue,
                    transient_command_pool: *transient_command_pool,
                    format: vk::Format::R8G8B8A8_UINT,
                    old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                    sync: TransientCommandSync::default(),
                })?;

                staging_buffer.free(device);
            }
        }

        Ok(())
    }
}
