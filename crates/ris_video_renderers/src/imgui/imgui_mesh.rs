use ash::vk;

use imgui::DrawData;
use imgui::DrawVert;

use ris_error::RisResult;
use ris_video_data::buffer::Buffer;

pub struct Mesh {
    pub vertices: Buffer,
    pub vertex_count: usize,
    pub indices: Buffer,
    pub index_count: usize,
}

impl Mesh {
    /// # Safety
    ///
    /// Must only be called once. Memory must not be freed twice.
    pub unsafe fn free(&mut self, device: &ash::Device) {
        self.vertices.free(device);
        self.indices.free(device);
    }

    /// # Safety
    ///
    /// `free()` must be called, or you are leaking memory.
    pub unsafe fn alloc(
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        draw_data: &DrawData,
    ) -> RisResult<Self> {
        let vertices = Self::create_vertices(draw_data);
        let vertex_count = vertices.len();
        let indices = Self::create_indices(draw_data);
        let index_count = vertices.len();

        let vertices_slice = vertices.as_slice();
        let vertex_buffer_size = std::mem::size_of_val(vertices_slice) as vk::DeviceSize;
        let vertex_buffer = Buffer::alloc(
            device,
            vertex_buffer_size,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE
                | vk::MemoryPropertyFlags::HOST_COHERENT
                | vk::MemoryPropertyFlags::DEVICE_LOCAL,
            physical_device_memory_properties,
        )?;

        vertex_buffer.write(device, &vertices)?;

        let index_buffer_size = std::mem::size_of_val(indices.as_slice()) as vk::DeviceSize;
        let index_buffer = Buffer::alloc(
            device,
            index_buffer_size,
            vk::BufferUsageFlags::INDEX_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE
                | vk::MemoryPropertyFlags::HOST_COHERENT
                | vk::MemoryPropertyFlags::DEVICE_LOCAL,
            physical_device_memory_properties,
        )?;

        index_buffer.write(device, &indices)?;

        Ok(Self {
            vertices: vertex_buffer,
            vertex_count,
            indices: index_buffer,
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
        let vertices = Self::create_vertices(draw_data);
        let old_vertex_count = self.vertex_count;
        let new_vertex_count = draw_data.total_vtx_count as usize;

        if old_vertex_count < new_vertex_count {
            let vertex_buffer_size = std::mem::size_of_val(vertices.as_slice()) as vk::DeviceSize;
            let new_vertex_buffer = unsafe {
                Buffer::alloc(
                    device,
                    vertex_buffer_size,
                    vk::BufferUsageFlags::VERTEX_BUFFER,
                    vk::MemoryPropertyFlags::HOST_VISIBLE
                        | vk::MemoryPropertyFlags::HOST_COHERENT
                        | vk::MemoryPropertyFlags::DEVICE_LOCAL,
                    physical_device_memory_properties,
                )
            }?;

            self.vertex_count = vertices.len();

            let old_buffer = self.vertices;
            self.vertices = new_vertex_buffer;

            unsafe { old_buffer.free(device) };
        }
        unsafe { self.vertices.write(device, &vertices) }?;

        let indices = Self::create_indices(draw_data);
        let old_index_count = self.index_count;
        let new_index_count = draw_data.total_idx_count as usize;

        if old_index_count < new_index_count {
            let index_buffer_size = std::mem::size_of_val(indices.as_slice()) as vk::DeviceSize;
            let new_index_buffer = unsafe {
                Buffer::alloc(
                    device,
                    index_buffer_size,
                    vk::BufferUsageFlags::INDEX_BUFFER,
                    vk::MemoryPropertyFlags::HOST_VISIBLE
                        | vk::MemoryPropertyFlags::HOST_COHERENT
                        | vk::MemoryPropertyFlags::DEVICE_LOCAL,
                    physical_device_memory_properties,
                )
            }?;

            self.index_count = indices.len();

            let old_buffer = self.indices;
            self.indices = new_index_buffer;

            unsafe { old_buffer.free(device) };
        }
        unsafe { self.indices.write(device, &indices) }?;

        Ok(())
    }
}
