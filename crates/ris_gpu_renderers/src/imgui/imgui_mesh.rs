use ash::vk;

use imgui::DrawData;
use imgui::DrawVert;

use ris_error::prelude::*;
use ris_gpu::buffer::Buffer;

pub struct Mesh {
    pub vertices: Buffer,
    pub vertex_mapped_memory: *mut DrawVert,
    pub vertex_count: usize,
    pub indices: Buffer,
    pub index_mapped_memory: *mut u16,
    pub index_count: usize,
}

impl Mesh {
    /// # Safety
    ///
    /// May only be called once. Memory must not be freed twice.
    pub unsafe fn free(&mut self, device: &ash::Device) {
        self.vertices.free(device);
        self.indices.free(device);
    }

    pub fn alloc(
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        draw_data: &DrawData,
    ) -> RisResult<Self> {
        // vertices
        let vertex_data = Self::create_vertices(draw_data);
        let vertex_count = vertex_data.len();
        let vertex_buffer_size = std::mem::size_of_val(vertex_data.as_slice());

        let vertices = Buffer::alloc(
            device,
            vertex_buffer_size,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::DEVICE_LOCAL,
            physical_device_memory_properties,
        )?;
        let vertex_mapped_memory = vertices.map_memory(device)?;
        unsafe {
            ris_gpu::io::write_to_mapped_memory(
                device,
                &vertex_data,
                vertices.memory,
                vertex_mapped_memory,
            )
        }?;

        // indices
        let index_data = Self::create_indices(draw_data);
        let index_count = index_data.len();
        let index_buffer_size = std::mem::size_of_val(index_data.as_slice());

        let indices = Buffer::alloc(
            device,
            index_buffer_size,
            vk::BufferUsageFlags::INDEX_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::DEVICE_LOCAL,
            physical_device_memory_properties,
        )?;
        let index_mapped_memory = indices.map_memory(device)?;
        unsafe {
            ris_gpu::io::write_to_mapped_memory(
                device,
                &index_data,
                indices.memory,
                index_mapped_memory,
            )
        }?;

        // finish
        Ok(Self {
            vertices,
            vertex_mapped_memory,
            vertex_count,
            indices,
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

            let new_size = std::mem::size_of_val(vertices.as_slice());
            self.vertices
                .resize(new_size, device, physical_device_memory_properties)?;
            self.vertex_mapped_memory = self.vertices.map_memory(device)?;
        }
        unsafe {
            ris_gpu::io::write_to_mapped_memory(
                device,
                vertices,
                self.vertices.memory,
                self.vertex_mapped_memory,
            )
        }?;

        // indices
        let indices = Self::create_indices(draw_data);
        if self.index_count < indices.len() {
            self.index_count = indices.len();

            let new_size = std::mem::size_of_val(indices.as_slice());
            self.indices
                .resize(new_size, device, physical_device_memory_properties)?;
            self.index_mapped_memory = self.indices.map_memory(device)?;
        }
        unsafe {
            ris_gpu::io::write_to_mapped_memory(
                device,
                indices,
                self.indices.memory,
                self.index_mapped_memory,
            )
        }?;

        Ok(())
    }
}
