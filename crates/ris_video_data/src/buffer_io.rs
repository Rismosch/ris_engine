use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

use ash::vk;

use ris_async::ThreadPool;
use ris_async::JobFuture;
use ris_error::prelude::*;

use super::buffer::Buffer;

const CHUNK_SIZE: usize = 1 << 20;

pub struct BufferIO {
    staging: Arc<Mutex<Staging>>,
}

struct Staging {
    buffer: vk::Buffer,
    memory: vk::DeviceMemory,
}

pub struct UploadArgs<'a> {
    pub device: &'a ash::Device,
    pub buffer: &'a Buffer,
    pub data: Vec<u8>,
    pub offset: usize,
    pub length: usize,
}

impl BufferIO {
    pub unsafe fn free(&mut self) {

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

        let staging = Mutex::new(Staging{
            buffer,
            memory,
        });

        Ok(Self{staging})
    }

    pub fn upload(
        &self,
        args: UploadArgs,
    ) -> JobFuture<Vec<u8>>{
        let staging = self.staging.clone();

        let UploadArgs { 
            device,
            buffer,
            data,
            offset,
            length,
        } = args;

        let device = device.clone();

        ThreadPool::submit(async move {
            let mut i = offset;
            while i < data.len() {
                let start = i;
                i += CHUNK_SIZE;

                let end = start + CHUNK_SIZE;
                let end = usize::min(end, data.len());

                let slice = &data[start..end];

                unsafe {
                    let ptr = device.map_memory(
                        self.memory,
                        0,
                        slice.len(),
                        vk::MemoryMapFlags::empty(),
                    );
                }
            }

            data
        })
    }

    pub fn download(
        &self,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
    ) -> RisResult<JobFuture<Vec<u8>>>{
        todo!();
    }
}
