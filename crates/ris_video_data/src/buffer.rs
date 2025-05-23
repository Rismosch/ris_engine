use std::ptr;

use ash::vk;

use ris_error::Extensions;
use ris_error::RisResult;

use super::transient_command::TransientCommand;
use super::transient_command::TransientCommandSync;

#[derive(Debug, Clone, Copy)]
pub struct Buffer {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
}

pub struct CopyToBufferInfo<'a> {
    pub device: &'a ash::Device,
    pub queue: vk::Queue,
    pub transient_command_pool: vk::CommandPool,
    pub dst: &'a Buffer,
    pub size: vk::DeviceSize,
    pub sync: TransientCommandSync,
}

pub struct CopyToImageInfo<'a> {
    pub device: &'a ash::Device,
    pub queue: vk::Queue,
    pub transient_command_pool: vk::CommandPool,
    pub image: vk::Image,
    pub width: u32,
    pub height: u32,
    pub sync: TransientCommandSync,
}

impl Buffer {
    /// # Safety
    ///
    /// May only be called once. Memory must not be freed twice.
    pub unsafe fn free(&self, device: &ash::Device) {
        unsafe {
            device.destroy_buffer(self.buffer, None);
            device.free_memory(self.memory, None);
        }
    }

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
        .into_ris_error()?;

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

    /// # Safety
    ///
    /// Must make sure that the buffer is big enough to hold `data`.
    pub unsafe fn write<T>(&self, device: &ash::Device, data: &[T]) -> RisResult<()> {
        let size = std::mem::size_of_val(data) as vk::DeviceSize;
        unsafe {
            let data_ptr =
                device.map_memory(self.memory, 0, size, vk::MemoryMapFlags::empty())? as *mut T;

            data_ptr.copy_from_nonoverlapping(data.as_ptr(), data.len());

            device.unmap_memory(self.memory);
        };

        Ok(())
    }

    /// # Safety
    ///
    /// Must make sure that `dst` is big enough to hold `self`.
    pub unsafe fn copy_to_buffer(&self, info: CopyToBufferInfo) -> RisResult<()> {
        let CopyToBufferInfo {
            device,
            queue,
            transient_command_pool,
            dst,
            size,
            sync,
        } = info;

        let transient_command = TransientCommand::begin(device, queue, transient_command_pool)?;

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

        transient_command.end_and_submit(sync)?;
        Ok(())
    }

    /// # Safety
    ///
    /// Must make sure that the image is big enough to hold the data of this buffer.
    pub unsafe fn copy_to_image(&self, info: CopyToImageInfo) -> RisResult<()> {
        let CopyToImageInfo {
            device,
            queue,
            transient_command_pool,
            image,
            width,
            height,
            sync,
        } = info;

        let transient_command = TransientCommand::begin(device, queue, transient_command_pool)?;

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

        transient_command.end_and_submit(sync)?;
        Ok(())
    }
}
