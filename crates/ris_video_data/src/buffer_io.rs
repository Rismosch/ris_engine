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

const CHUNK_SIZE: usize = 1 << 20;

pub struct BufferIO {
    staging: Arc<Mutex<Staging>>,
}

struct Staging {
    buffer: vk::Buffer,
    memory: vk::DeviceMemory,
}

pub struct WriteToBufferArgs<'a> {
    pub transient_command_args: TransientCommandArgs,
    pub src: Vec<u8>,
    pub dst: &'a Buffer,
    pub src_offset: usize,
    pub dst_offset: usize,
    pub size: usize,
}

impl BufferIO {
    pub unsafe fn free(&mut self) {
        free buffer
    }

    pub fn alloc(
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
    ) -> RisResult<Self> {
        let buffer_create_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BUFFER_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::BufferCreateFlags::empty(),
            size: CHUNK_SIZE as vk::DeviceSize,
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

        let staging = Arc::new(Mutex::new(Staging{
            buffer,
            memory,
        }));

        Ok(Self{staging})
    }

    pub fn write_to_buffer(&self, args: WriteToBufferArgs) -> RisResult<JobFuture<RisResult<()>>>{
        let WriteToBufferArgs { 
            transient_command_args,
            src,
            dst,
            src_offset,
            dst_offset,
            size,
        } = args;

        let src_start = src_offset;
        let src_end = src_start + size;
        let dst_start = dst_offset;
        let dst_end = dst_start + size;

        ris_error::assert!(src_end <= src.len())?;
        ris_error::assert!(dst_end <= dst.len())?;

        let device = transient_command_args.device.clone();
        let staging = self.staging.clone();
        let dst_buffer = dst.buffer;

        let future = ThreadPool::submit(async move {
            let staging = ThreadPool::lock(&staging);
            let mut submit_future = JobFuture::finished(());

            let mut src_i = src_start;
            let mut dst_i = dst_start;
            loop {
                let from = src_i;
                let dst_offset = dst_i as vk::DeviceSize;

                src_i += CHUNK_SIZE;
                dst_i += CHUNK_SIZE;

                if src_i > src_end {
                    break;
                }

                let to = from + CHUNK_SIZE;
                let to = usize::min(to, src_end);
                let slice = &src[from..to];

                unsafe {
                    let ptr = device.map_memory(
                        staging.memory,
                        0,
                        slice.len() as vk::DeviceSize,
                        vk::MemoryMapFlags::empty(),
                    )? as *mut u8;

                    ptr.copy_from_nonoverlapping(
                        slice.as_ptr(),
                        slice.len(),
                    );

                    device.unmap_memory(staging.memory);

                    let args = transient_command_args.clone();
                    let command = TransientCommand::begin(args)?;

                    let buffer_copy = vk::BufferCopy{
                        src_offset: 0,
                        dst_offset,
                        size: slice.len() as vk::DeviceSize,
                    };
                    device.cmd_copy_buffer(
                        command.buffer(),
                        staging.buffer,
                        dst_buffer,
                        &[buffer_copy],
                    );

                    submit_future.wait();
                    let sync = TransientCommandSync::default();
                    submit_future = command.end_and_submit(sync)?;
                }
            }

            submit_future.wait();
            Ok(())
        });

        Ok(future)
    }

    pub fn download(&self) -> RisResult<JobFuture<Vec<u8>>>{
        todo!();
    }
}
