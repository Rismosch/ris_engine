use ash::vk;

use imgui::DrawData;
use imgui::DrawVert;

use ris_error::prelude::*;

pub struct Mesh {
    pub vertex_buffer: vk::Buffer,
    pub vertex_memory: vk::DeviceMemory,
    pub vertex_mapped_memory: *mut DrawVert,
    pub vertex_count: usize,
    pub index_buffer: vk::Buffer,
    pub index_memory: vk::DeviceMemory,
    pub index_mapped_memory: *mut u16,
    pub index_count: usize,
}

impl Mesh {
    /// # Safety
    ///
    /// May only be called once. Memory must not be freed twice.
    pub unsafe fn free(&mut self, device: &ash::Device) {
        device.destroy_buffer(self.vertex_buffer, None);
        device.free_memory(self.vertex_memory, None);
        device.destroy_buffer(self.index_buffer, None);
        device.free_memory(self.index_memory, None);
    }

    pub fn alloc(
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        draw_data: &DrawData,
    ) -> RisResult<Self> {
        // vertices
        let vertices = Self::create_vertices(draw_data);
        let vertex_count = vertices.len();
        let vertex_buffer_size = std::mem::size_of_val(vertices.as_slice()) as vk::DeviceSize;

        let (vertex_buffer, vertex_memory, vertex_mapped_memory) =
            Self::alloc_buffer_and_memory::<DrawVert>(
                device,
                vertex_buffer_size,
                vk::BufferUsageFlags::VERTEX_BUFFER,
                vk::MemoryPropertyFlags::HOST_VISIBLE
                    | vk::MemoryPropertyFlags::HOST_COHERENT
                    | vk::MemoryPropertyFlags::DEVICE_LOCAL,
                physical_device_memory_properties,
            )?;

        unsafe { vertex_mapped_memory.copy_from_nonoverlapping(vertices.as_ptr(), vertices.len()) };

        // indices
        let indices = Self::create_indices(draw_data);
        let index_count = vertices.len();
        let index_buffer_size = std::mem::size_of_val(indices.as_slice()) as vk::DeviceSize;

        let (index_buffer, index_memory, index_mapped_memory) = Self::alloc_buffer_and_memory::<u16>(
            device,
            index_buffer_size,
            vk::BufferUsageFlags::INDEX_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE
                | vk::MemoryPropertyFlags::HOST_COHERENT
                | vk::MemoryPropertyFlags::DEVICE_LOCAL,
            physical_device_memory_properties,
        )?;

        unsafe { index_mapped_memory.copy_from_nonoverlapping(indices.as_ptr(), indices.len()) };

        // finish
        Ok(Self {
            vertex_buffer,
            vertex_memory,
            vertex_mapped_memory,
            vertex_count,
            index_buffer,
            index_memory,
            index_mapped_memory,
            index_count,
        })
    }

    pub fn create_vertices(draw_data: &DrawData) -> Vec<DrawVert> {
        let vertex_count = draw_data.total_vtx_count as usize;
        let mut vertices = Vec::with_capacity(vertex_count);
        for draw_list in draw_data.draw_lists() {
            vertices.extend_from_slice(draw_list.vtx_buffer());
        }
        vertices
    }

    pub fn create_indices(draw_data: &DrawData) -> Vec<u16> {
        let index_count = draw_data.total_idx_count as usize;
        let mut indices = Vec::with_capacity(index_count);
        for draw_list in draw_data.draw_lists() {
            indices.extend_from_slice(draw_list.idx_buffer());
        }
        indices
    }

    pub fn update(
        &mut self,
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        draw_data: &DrawData,
    ) -> RisResult<()> {
        // vertices
        let vertices = Self::create_vertices(draw_data);
        if self.vertex_count < vertices.len() {
            self.vertex_count = vertices.len();

            unsafe {
                device.destroy_buffer(self.vertex_buffer, None);
                device.free_memory(self.vertex_memory, None);
            }

            let vertex_buffer_size = std::mem::size_of_val(vertices.as_slice()) as vk::DeviceSize;
            let (new_buffer, new_memory, new_mapped_memory) =
                Self::alloc_buffer_and_memory::<DrawVert>(
                    device,
                    vertex_buffer_size,
                    vk::BufferUsageFlags::VERTEX_BUFFER,
                    vk::MemoryPropertyFlags::HOST_VISIBLE
                        | vk::MemoryPropertyFlags::HOST_COHERENT
                        | vk::MemoryPropertyFlags::DEVICE_LOCAL,
                    physical_device_memory_properties,
                )?;

            self.vertex_buffer = new_buffer;
            self.vertex_memory = new_memory;
            self.vertex_mapped_memory = new_mapped_memory;
        }
        unsafe {
            self.vertex_mapped_memory
                .copy_from_nonoverlapping(vertices.as_ptr(), vertices.len())
        };

        // indices
        let indices = Self::create_indices(draw_data);
        if self.index_count < indices.len() {
            self.index_count = indices.len();

            unsafe {
                device.destroy_buffer(self.index_buffer, None);
                device.free_memory(self.index_memory, None);
            }

            let index_buffer_size = std::mem::size_of_val(indices.as_slice()) as vk::DeviceSize;
            let (new_buffer, new_memory, new_mapped_memory) = Self::alloc_buffer_and_memory(
                device,
                index_buffer_size,
                vk::BufferUsageFlags::INDEX_BUFFER,
                vk::MemoryPropertyFlags::HOST_VISIBLE
                    | vk::MemoryPropertyFlags::HOST_COHERENT
                    | vk::MemoryPropertyFlags::DEVICE_LOCAL,
                physical_device_memory_properties,
            )?;

            self.index_buffer = new_buffer;
            self.index_memory = new_memory;
            self.index_mapped_memory = new_mapped_memory;
        }
        unsafe {
            self.index_mapped_memory
                .copy_from_nonoverlapping(indices.as_ptr(), indices.len())
        };

        Ok(())
    }

    pub fn alloc_buffer_and_memory<T>(
        device: &ash::Device,
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        memory_property_flags: vk::MemoryPropertyFlags,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
    ) -> RisResult<(vk::Buffer, vk::DeviceMemory, *mut T)> {
        // buffer
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

        // memory
        let memory_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };
        let memory_type_index = ris_gpu::util::find_memory_type(
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

        let mapped_memory =
            unsafe { device.map_memory(memory, 0, size, vk::MemoryMapFlags::empty()) }? as *mut T;

        Ok((buffer, memory, mapped_memory))
    }
}
