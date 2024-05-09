use std::ptr;

use ash::vk;

use ris_error::Extensions;
use ris_error::RisResult;

use super::transient_command::TransientCommand;

#[derive(Clone, Copy)]
pub struct Buffer {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
}

impl Buffer {
    pub fn alloc(
        device: &ash::Device,
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        memory_property_flags: vk::MemoryPropertyFlags,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
    ) -> RisResult<Self> {
        let buffer_create_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BUFFER_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::BufferCreateFlags::empty(),
            size,
            usage,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
        };

        let buffer = unsafe { device.create_buffer(&buffer_create_info, None) }?;

        let memory_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };
        let memory_type_index = super::util::find_memory_type(
            memory_requirements.memory_type_bits,
            memory_property_flags,
            physical_device_memory_properties,
        )?
        .unroll()?;

        let memory_allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
            p_next: ptr::null(),
            allocation_size: memory_requirements.size,
            memory_type_index,
        };

        let memory = unsafe { device.allocate_memory(&memory_allocate_info, None) }?;

        unsafe { device.bind_buffer_memory(buffer, memory, 0) }?;

        Ok(Self { buffer, memory })
    }

    pub fn free(&self, device: &ash::Device) {
        unsafe {
            device.destroy_buffer(self.buffer, None);
            device.free_memory(self.memory, None);
        }
    }

    pub fn write<T>(&self, device: &ash::Device, data: &[T]) -> RisResult<()> {
        let size = std::mem::size_of_val(data) as vk::DeviceSize;
        unsafe {
            let data_ptr = device.map_memory(
                self.memory,
                0,
                size,
                vk::MemoryMapFlags::empty(),
            )? as *mut T;

            data_ptr.copy_from_nonoverlapping(data.as_ptr(), data.len());

            device.unmap_memory(self.memory);
        };

        Ok(())
    }

    pub fn copy_to_buffer(
        &self,
        device: &ash::Device,
        queue: vk::Queue,
        transient_command_pool: vk::CommandPool,
        dst: &Self,
        size: vk::DeviceSize,
    ) -> RisResult<()> {
        let transient_command = TransientCommand::begin(&device, queue, transient_command_pool)?;

        let copy_regions = [vk::BufferCopy {
            src_offset: 0,
            dst_offset: 0,
            size,
        }];

        unsafe {
            device.cmd_copy_buffer(
                transient_command.buffer(),
                self.buffer,
                dst.buffer,
                &copy_regions,
            )
        };

        transient_command.end_and_submit()?;
        Ok(())
    }

    pub fn copy_to_image(
        &self,
        device: &ash::Device,
        queue: vk::Queue,
        transient_command_pool: vk::CommandPool,
        image: vk::Image,
        width: u32,
        height: u32,
    ) -> RisResult<()> {
        let transient_command = TransientCommand::begin(&device, queue, transient_command_pool)?;

        let regions = [vk::BufferImageCopy {
            buffer_offset: 0,
            buffer_row_length: 0,
            buffer_image_height: 0,
            image_subresource: vk::ImageSubresourceLayers {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                mip_level: 0,
                base_array_layer: 0,
                layer_count: 1,
            },
            image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
            image_extent: vk::Extent3D {
                width,
                height,
                depth: 1,
            },
        }];

        unsafe {
            device.cmd_copy_buffer_to_image(
                transient_command.buffer(),
                self.buffer,
                image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &regions,
            )
        };

        transient_command.end_and_submit()?;
        Ok(())
    }
}
