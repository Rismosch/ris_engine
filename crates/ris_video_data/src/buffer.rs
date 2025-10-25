use ash::vk;

use ris_error::Extensions;
use ris_error::RisResult;

#[derive(Debug)]
pub struct Buffer {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
    usage: vk::BufferUsageFlags,
    memory_property_flags: vk::MemoryPropertyFlags,
    size: usize,
    capacity: usize,
}

impl Buffer {
    /// # Safety
    ///
    /// - May only be called once. Memory must not be freed twice.
    /// - This object must not be used after it was freed
    pub unsafe fn free(&self, device: &ash::Device) {
        device.destroy_buffer(self.buffer, None);
        device.free_memory(self.memory, None);
    }

    pub fn alloc_local(
        device: &ash::Device,
        size: usize,
        usage: vk::BufferUsageFlags,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
    ) -> RisResult<Self> {
        Self::alloc(
            device,
            size,
            usage,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            physical_device_memory_properties,
        )
    }

    pub fn alloc_staging(
        device: &ash::Device,
        size: usize,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
    ) -> RisResult<Self> {
        Self::alloc(
            device,
            size,
            vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            physical_device_memory_properties,
        )
    }

    pub fn alloc(
        device: &ash::Device,
        size: usize,
        usage: vk::BufferUsageFlags,
        memory_property_flags: vk::MemoryPropertyFlags,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
    ) -> RisResult<Self> {
        if memory_property_flags.intersects(vk::MemoryPropertyFlags::HOST_COHERENT) {
            ris_log::warning!(
                "attempted to allocate gpu buffer with memory property {:?}. this may be slower than flushing manually. it is generally adised to avoid {:?}",
                vk::MemoryPropertyFlags::HOST_COHERENT,
                vk::MemoryPropertyFlags::HOST_COHERENT,
            )
        }

        let (buffer, memory) = Self::alloc_buffer_and_memory(
            device,
            size as vk::DeviceSize,
            usage,
            memory_property_flags,
            physical_device_memory_properties,
        )?;

        Ok(Self {
            buffer,
            memory,
            usage,
            memory_property_flags,
            size,
            capacity: size,
        })
    }

    fn alloc_buffer_and_memory(
        device: &ash::Device,
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        memory_property_flags: vk::MemoryPropertyFlags,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
    ) -> RisResult<(vk::Buffer, vk::DeviceMemory)> {
        if size == 0 {
            return ris_error::new_result!("cannot allocate memory of size 0");
        }

        let buffer_create_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BUFFER_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::BufferCreateFlags::empty(),
            size,
            usage,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: std::ptr::null(),
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
            p_next: std::ptr::null(),
            allocation_size: memory_requirements.size,
            memory_type_index,
        };

        let memory = unsafe { device.allocate_memory(&memory_allocate_info, None) }?;

        unsafe { device.bind_buffer_memory(buffer, memory, 0) }?;

        Ok((buffer, memory))
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub unsafe fn resize(
        &mut self,
        new_size: usize,
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
    ) -> RisResult<()> {
        if new_size > self.capacity {
            self.free(device);
            let (buffer, memory) = Self::alloc_buffer_and_memory(
                device,
                new_size as vk::DeviceSize,
                self.usage,
                self.memory_property_flags,
                physical_device_memory_properties,
            )?;

            self.buffer = buffer;
            self.memory = memory;
            self.capacity = new_size;
        }

        self.size = new_size;

        Ok(())
    }
}
