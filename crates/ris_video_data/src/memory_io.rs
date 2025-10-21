use ash::vk;

use ris_error::prelude::*;

use super::buffer::Buffer;
use super::image::Image;
use super::transient_command::TransientCommand;
use super::transient_command::TransientCommandArgs;
use super::transient_command::TransientCommandSync;

const STAGING_SIZE: vk::DeviceSize = 1 << 16;

pub struct MemoryIO {
    staging_buffers: Vec<Option<Staging>>,
}

pub struct StagingBuffer<'a> {
    io: &'a mut MemoryIO
}

struct Staging {
    buffer: vk::Buffer,
    memory: vk::DeviceMemory,
}

pub struct MemoryIOArgs<'a, GpuObject> {
    pub transient_command_args: TransientCommandArgs,
    pub bytes: Vec<u8>,
    //pub bytes_offset: usize,
    pub gpu_object: &'a GpuObject,
    //pub gpu_object_offset: usize,
    //pub size: usize,
    pub staging_buffer: StagingBuffer,
}

//struct InternalIOArgs<TCallback>
//where TCallback: Fn(vk::CommandBuffer, vk::Buffer)
//{
//    pub transient_command_args: TransientCommandArgs,
//    pub bytes: Vec<u8>,
//    //pub bytes_offset: usize,
//    //pub gpu_object_offset: usize,
//    pub gpu_object_size: usize,
//    //pub size: usize,
//    pub copy_callback: TCallback,
//    pub staging_buffer: StagingBuffer,
//}

impl MemoryIO {
    pub unsafe fn free(&mut self, device: &ash::Device) {
        for staging in self.staging_buffers.iter() {
            let is_in_use = staging.in_use.load(Ordering::Relaxed);
            if is_in_use {
                ris_error::throw!("attempted to free a staging buffer that is still in use");
            }

            device.destroy_buffer(staging.buffer, None);
            device.free_memory(staging.memory, None);
        }
    }

    pub fn acquire_staging_buffer(&self) -> StagingBuffer {
        loop {
            for (index, staging_buffer) in self.staging_buffers.iter().enumerate() {
                if staging_buffer.in_use.swap(true, Ordering::Acquire) {
                    continue;
                } else {
                    return StagingBuffer{index}
                }
            }

            std::hint::spin_loop();
        }
    }

    pub fn alloc(
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        staging_buffer_count: usize,
    ) -> RisResult<Self> {
        let mut staging_buffers = Vec::with_capacity(staging_buffer_count);
        for _ in 0..staging_buffers.capacity() {
            let buffer_create_info = vk::BufferCreateInfo {
                s_type: vk::StructureType::BUFFER_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: vk::BufferCreateFlags::empty(),
                size: STAGING_SIZE,
                usage: vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                queue_family_index_count: 0,
                p_queue_family_indices: std::ptr::null(),
            };

            let buffer = unsafe { device.create_buffer(&buffer_create_info, None) }?;

            let memory_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };
            let memory_type_index = super::util::find_memory_type(
                memory_requirements.memory_type_bits,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
                physical_device_memory_properties,
            )?
            .into_ris_error()?;

            let memory_allocate_info = vk::MemoryAllocateInfo {
                s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
                p_next: std::ptr::null(),
                allocation_size: memory_requirements.size,
                memory_type_index,
            };

            let memory = unsafe {device.allocate_memory(&memory_allocate_info, None)}?;

            unsafe { device.bind_buffer_memory(buffer, memory, 0) }?;

            let staging = Staging {
                buffer,
                memory,
                in_use: AtomicBool::new(false),
            };
            staging_buffers.push(staging);
        }

        Ok(Self{staging_buffers})
    }

    pub unsafe fn write_to_buffer(
        &self,
        args: MemoryIOArgs<Buffer>) -> RisResult<Vec<u8>>{
        let MemoryIOArgs { 
            transient_command_args,
            bytes: src,
            gpu_object: dst,
            staging_buffer,
        } = args;

        // setup
        let size = src.len() as vk::DeviceSize;
        
        ris_error::assert!(size != 0)?;
        ris_error::assert!(size <= STAGING_SIZE)?;
        ris_error::assert!(size == dst.size() as vk::DeviceSize)?;

        let staging = self.staging_buffers
            .get(staging_buffer.index)
            .into_ris_error()?;

        let device = transient_command_args.device.clone();

        // write to staging buffer
        let ptr = device.map_memory(
            staging.memory,
            0,
            src.len() as vk::DeviceSize,
            vk::MemoryMapFlags::empty(),
        )? as *mut u8;

        ptr.copy_from_nonoverlapping(
            src.as_ptr(),
            src.len(),
        );

        device.unmap_memory(staging.memory);

        // copy from staging buffer
        let args = transient_command_args.clone();
        let command = TransientCommand::begin(args)?;

        device.cmd_copy_buffer(
            command.buffer(),
            staging.buffer,
            dst.buffer,
            &[vk::BufferCopy{
                src_offset: 0,
                dst_offset: 0,
                size,
            }],
        );

        // submit
        let sync = TransientCommandSync::default();
        let submit_future = command.end_and_submit(sync)?;
        submit_future.wait();

        // done
        Ok(src)
    }

    pub unsafe fn read_from_buffer(&self, args: MemoryIOArgs<Buffer>) -> RisResult<Vec<u8>>{
        let MemoryIOArgs { 
            transient_command_args,
            bytes: mut dst,
            gpu_object: src,
            staging_buffer,
        } = args;

        // setup
        let size = dst.len() as vk::DeviceSize;
        
        ris_error::assert!(size != 0)?;
        ris_error::assert!(size <= STAGING_SIZE)?;
        ris_error::assert!(size == src.size() as vk::DeviceSize)?;

        let staging = self.staging_buffers
            .get(staging_buffer.index)
            .into_ris_error()?;

        let device = transient_command_args.device.clone();

        // copy to staging buffer
        let args = transient_command_args.clone();
        let command = TransientCommand::begin(args)?;

        device.cmd_copy_buffer(
            command.buffer(),
            src.buffer,
            staging.buffer,
            &[vk::BufferCopy {
                src_offset: 0,
                dst_offset: 0,
                size,
            }],
        );

        // submit
        let sync = TransientCommandSync::default();
        let submit_future = command.end_and_submit(sync)?;
        submit_future.wait();

        // read from staging buffer
        let ptr = device.map_memory(
            staging.memory,
            0,
            size,
            vk::MemoryMapFlags::empty(),
        )? as *mut u8;

        ptr.copy_to_nonoverlapping(
            dst.as_mut_ptr(),
            dst.len(),
        );

        device.unmap_memory(staging.memory);

        // done
        Ok(dst)
    }
}
