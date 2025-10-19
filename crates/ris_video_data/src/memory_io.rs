use std::sync::Arc;
use std::sync::Mutex;

use ash::vk;

use ris_async::ThreadPool;
use ris_async::JobFuture;
use ris_error::prelude::*;

use super::buffer::Buffer;
use super::transient_command::TransientCommand;
use super::transient_command::TransientCommandArgs;
use super::transient_command::TransientCommandSync;

pub static mut CHUNK_SIZE: usize = 1 << 16;

pub struct MemoryIO {
    staging: Arc<Mutex<Staging>>, // TODO VEC HERE
}

struct Staging {
    buffer: vk::Buffer,
    memory: vk::DeviceMemory,
}

pub struct BufferIOArgs<'a> {
    pub transient_command_args: TransientCommandArgs,
    pub bytes: Vec<u8>,
    pub buffer: &'a Buffer,
    pub bytes_offset: usize,
    pub buffer_offset: usize,
    pub size: usize,
}

impl MemoryIO {
    pub unsafe fn free(&mut self, device: &ash::Device) {
        let staging = ThreadPool::lock(&self.staging);

        device.destroy_buffer(staging.buffer, None);
        device.free_memory(staging.memory, None);
    }

    pub fn alloc(
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
    ) -> RisResult<Self> {
        let chunk_size = unsafe {CHUNK_SIZE};

        let buffer_create_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BUFFER_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::BufferCreateFlags::empty(),
            size: chunk_size as vk::DeviceSize,
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

        let staging = Arc::new(Mutex::new(Staging{
            buffer,
            memory,
        }));

        Ok(Self{staging})
    }

    pub unsafe fn write_to_buffer(&self, args: BufferIOArgs) -> RisResult<JobFuture<RisResult<Vec<u8>>>>{
        let BufferIOArgs { 
            transient_command_args,
            bytes: src,
            buffer: dst,
            bytes_offset: src_offset,
            buffer_offset: dst_offset,
            size,
        } = args;

        if size == 0 {
            return Ok(JobFuture::finished(Ok(src)));
        }

        let src_start = src_offset;
        let src_end = src_start + size;
        let dst_start = dst_offset;
        let dst_end = dst_start + size;

        ris_error::assert!(src_end <= src.len())?;
        ris_error::assert!(dst_end <= dst.size())?;

        let device = transient_command_args.device.clone();
        let staging = self.staging.clone();
        let dst_buffer = dst.buffer;

        let future = ThreadPool::submit(async move {
            // TODO: do not block while holding the lock:
            let staging = ThreadPool::lock(&staging);

            let mut src_i = src_start;
            let mut dst_i = dst_start;
            let chunk_size = unsafe {CHUNK_SIZE};

            while src_i < src_end {
                let src_from = src_i;
                let dst_offset = dst_i as vk::DeviceSize;

                src_i += chunk_size;
                dst_i += chunk_size;

                let src_to = src_from + chunk_size;
                let src_to = usize::min(src_to, src_end);
                let src_slice = &src[src_from..src_to];

                let ptr = device.map_memory(
                    staging.memory,
                    0,
                    src_slice.len() as vk::DeviceSize,
                    vk::MemoryMapFlags::empty(),
                )? as *mut u8;

                ptr.copy_from_nonoverlapping(
                    src_slice.as_ptr(),
                    src_slice.len(),
                );

                device.unmap_memory(staging.memory);

                let args = transient_command_args.clone();
                let command = TransientCommand::begin(args)?;

                let buffer_copy = vk::BufferCopy{
                    src_offset: 0,
                    dst_offset,
                    size: src_slice.len() as vk::DeviceSize,
                };

                device.cmd_copy_buffer(
                    command.buffer(),
                    staging.buffer,
                    dst_buffer,
                    &[buffer_copy],
                );

                let sync = TransientCommandSync::default();
                let submit_future = command.end_and_submit(sync)?;
                submit_future.wait();
            }

            Ok(src)
        });

        Ok(future)
    }

    pub unsafe fn read_from_buffer(&self, args: BufferIOArgs) -> RisResult<JobFuture<RisResult<Vec<u8>>>>{
        let BufferIOArgs { 
            transient_command_args,
            bytes: mut dst,
            buffer: src,
            bytes_offset: dst_offset,
            buffer_offset: src_offset,
            size,
        } = args;

        if size == 0 {
            return Ok(JobFuture::finished(Ok(dst)))
        }

        let src_start = src_offset;
        let src_end = src_start + size;
        let dst_start = dst_offset;
        let dst_end = dst_start + size;

        ris_error::assert!(src_end <= src.size())?;
        ris_error::assert!(dst_end <= dst.len())?;

        let device = transient_command_args.device.clone();
        let staging = self.staging.clone();
        let src_buffer = src.buffer;

        let future = ThreadPool::submit(async move {
            // TODO: do not block while holding the lock:
            let staging = ThreadPool::lock(&staging);

            let mut src_i = src_start;
            let mut dst_i = dst_start;
            let chunk_size = unsafe {CHUNK_SIZE};

            while src_i < src_end {
                let src_offset = src_i as vk::DeviceSize;
                let dst_from = dst_i;

                src_i += chunk_size;
                dst_i += chunk_size;

                let dst_to = dst_from + chunk_size;
                let dst_to = usize::min(dst_to, dst_end);
                let dst_slice = &mut dst[dst_from..dst_to];

                let args = transient_command_args.clone();
                let command = TransientCommand::begin(args)?;

                let buffer_copy = vk::BufferCopy {
                    src_offset,
                    dst_offset: 0,
                    size: dst_slice.len() as vk::DeviceSize,
                };
                device.cmd_copy_buffer(
                    command.buffer(),
                    src_buffer,
                    staging.buffer,
                    &[buffer_copy],
                );

                let sync = TransientCommandSync::default();
                let submit_future = command.end_and_submit(sync)?;
                submit_future.wait();

                let ptr = device.map_memory(
                    staging.memory,
                    0,
                    dst_slice.len() as vk::DeviceSize,
                    vk::MemoryMapFlags::empty(),
                )? as *mut u8;

                ptr.copy_to_nonoverlapping(
                    dst_slice.as_mut_ptr(),
                    dst_slice.len(),
                );

                device.unmap_memory(staging.memory);
            }

            Ok(dst)
        });

        Ok(future)
    }
}
