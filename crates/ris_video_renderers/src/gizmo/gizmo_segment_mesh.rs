use ash::vk;

use ris_debug::gizmo::GizmoSegmentVertex;
use ris_error::RisResult;
use ris_video_data::buffer::Buffer;

pub struct GizmoSegmentMesh {
    pub vertices: Buffer,
    pub vertex_count: usize,
}

impl GizmoSegmentMesh {
    /// # Safety
    ///
    /// - May only be called once. Memory must not be freed twice.
    /// - This object must not be used after it was freed
    pub unsafe fn free(&mut self, device: &ash::Device) {
        self.vertices.free(device);
    }

    pub fn alloc(
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        vertices: &[GizmoSegmentVertex],
    ) -> RisResult<Self> {
        todo!();
        //let vertex_buffer_size = std::mem::size_of_val(vertices) as vk::DeviceSize;
        //let vertex_buffer = Buffer::alloc(
        //    device,
        //    vertex_buffer_size,
        //    vk::BufferUsageFlags::VERTEX_BUFFER,
        //    vk::MemoryPropertyFlags::HOST_VISIBLE
        //        | vk::MemoryPropertyFlags::HOST_COHERENT
        //        | vk::MemoryPropertyFlags::DEVICE_LOCAL,
        //    physical_device_memory_properties,
        //)?;

        //unsafe { vertex_buffer.write(device, vertices) }?;

        //Ok(Self {
        //    vertices: vertex_buffer,
        //    vertex_count: vertices.len(),
        //})
    }

    pub fn update(
        &mut self,
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        vertices: &[GizmoSegmentVertex],
    ) -> RisResult<()> {
        todo!();
        //let old_vertex_count = self.vertex_count;
        //let new_vertex_count = vertices.len();

        //if old_vertex_count < new_vertex_count {
        //    let vertex_buffer_size = std::mem::size_of_val(vertices) as vk::DeviceSize;
        //    let new_vertex_buffer = Buffer::alloc(
        //        device,
        //        vertex_buffer_size,
        //        vk::BufferUsageFlags::VERTEX_BUFFER,
        //        vk::MemoryPropertyFlags::HOST_VISIBLE
        //            | vk::MemoryPropertyFlags::HOST_COHERENT
        //            | vk::MemoryPropertyFlags::DEVICE_LOCAL,
        //        physical_device_memory_properties,
        //    )?;

        //    self.vertex_count = vertices.len();

        //    let old_buffer = self.vertices;
        //    self.vertices = new_vertex_buffer;

        //    unsafe { old_buffer.free(device) };
        //}
        //unsafe { self.vertices.write(device, vertices) }?;

        //Ok(())
    }
}
