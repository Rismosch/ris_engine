use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

use ash::vk;

use ris_asset::AssetId;
use ris_error::Extensions;
use ris_error::RisResult;

use super::buffer::Buffer;
use super::texture::Texture;
use super::uniform_buffer_object::UniformBufferObject;

pub struct FrameInFlight {
    pub command_buffer: vk::CommandBuffer,
    pub uniform_buffer: Buffer,
    pub uniform_buffer_mapped: *mut UniformBufferObject,
    pub descriptor_set: vk::DescriptorSet,
    pub image_available_semaphore: vk::Semaphore,
    pub render_finished_semaphore: vk::Semaphore,
    pub in_flight_fence: vk::Fence,
}

impl FrameInFlight {
    pub fn alloc(
        device: &ash::Device,
        command_buffer: vk::CommandBuffer,
        descriptor_set: vk::DescriptorSet,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        texture: &Texture,
    ) -> RisResult<Self> {
        // uniform buffer
        let uniform_buffer_size = std::mem::size_of::<UniformBufferObject>() as vk::DeviceSize;
        let uniform_buffer = Buffer::alloc(
            &device,
            uniform_buffer_size,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            physical_device_memory_properties,
        )?;
        let uniform_buffer_mapped = unsafe{device.map_memory(
            uniform_buffer.memory,
            0,
            uniform_buffer_size,
            vk::MemoryMapFlags::empty()
        )}? as *mut UniformBufferObject;

        // descriptor set
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

        let write_descriptor_set = [
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

        unsafe{device.update_descriptor_sets(&write_descriptor_set, &[])};

        // synchronization objects
        let semaphore_create_info = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::SemaphoreCreateFlags::empty(),
        };

        let fence_create_info = vk::FenceCreateInfo {
            s_type: vk::StructureType::FENCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::FenceCreateFlags::SIGNALED,
        };

        let image_available_semaphore = unsafe{device.create_semaphore(&semaphore_create_info, None)}?;
        let render_finished_semaphore = unsafe{device.create_semaphore(&semaphore_create_info, None)}?;
        let in_flight_fence = unsafe{device.create_fence(&fence_create_info, None)}?;

        Ok(FrameInFlight {
            command_buffer,
            descriptor_set,
            uniform_buffer,
            uniform_buffer_mapped,
            image_available_semaphore,
            render_finished_semaphore,
            in_flight_fence,
        })
    }

    pub fn free(
        &self,
        device: &ash::Device,
    ) {
        unsafe {
            device.destroy_fence(self.in_flight_fence, None);
            device.destroy_semaphore(self.render_finished_semaphore, None);
            device.destroy_semaphore(self.image_available_semaphore, None);

            self.uniform_buffer.free(device);
        }
    }
}
