use ash::vk;

use ris_debug::gizmo::GizmoSegmentVertex;
use ris_error::RisResult;

use crate::vulkan::buffer::Buffer;

pub struct GizmoSegmentMesh {
    pub vertices: Buffer,
    pub vertex_count: usize,
}

impl GizmoSegmentMesh {
    /// # Safety
    ///
    /// Must only be called once. Memory must not be freed twice.
    pub unsafe fn free(&mut self, device: &ash::Device) {
        self.vertices.free(device);
    }

    /// # Safety
    ///
    /// `free()` must be called, or you are leaking memory.
    pub unsafe fn alloc(
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        vertices: &[GizmoSegmentVertex],
    ) -> RisResult<Self> {
        let vertex_buffer_size = std::mem::size_of_val(vertices) as vk::DeviceSize;
        let vertex_buffer = Buffer::alloc(
            device,
            vertex_buffer_size,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE
                | vk::MemoryPropertyFlags::HOST_COHERENT
                | vk::MemoryPropertyFlags::DEVICE_LOCAL,
            physical_device_memory_properties,
        )?;

        vertex_buffer.write(device, vertices)?;

        Ok(Self {
            vertices: vertex_buffer,
            vertex_count: vertices.len(),
        })
    }

    pub fn update(
        &mut self,
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        vertices: &[GizmoSegmentVertex],
    ) -> RisResult<()> {
        let old_vertex_count = self.vertex_count;
        let new_vertex_count = vertices.len();

        if old_vertex_count < new_vertex_count {
            let vertex_buffer_size = std::mem::size_of_val(vertices) as vk::DeviceSize;
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
        unsafe { self.vertices.write(device, vertices) }?;

        Ok(())
    }
}
