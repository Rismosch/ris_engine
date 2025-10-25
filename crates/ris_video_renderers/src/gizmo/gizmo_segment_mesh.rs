use ash::vk;

use ris_debug::gizmo::GizmoSegmentVertex;
use ris_error::RisResult;
use ris_video_data::buffer::Buffer;
use ris_video_data::gpu_io;

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
        let vertex_buffer_size = std::mem::size_of_val(vertices);
        let vertex_buffer = Buffer::alloc(
            device,
            vertex_buffer_size,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE
                | vk::MemoryPropertyFlags::DEVICE_LOCAL,
            physical_device_memory_properties,
        )?;

        unsafe {gpu_io::write_to_memory(
            device,
            vertices,
            vertex_buffer.memory,
        )}?;

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
        if self.vertex_count < vertices.len() {
            self.vertex_count = vertices.len();
            let vertex_buffer_size = std::mem::size_of_val(vertices);
            unsafe {self.vertices.resize(
                vertex_buffer_size,
                device,
                physical_device_memory_properties,
            )}?;
        }

        unsafe {gpu_io::write_to_memory(
            device,
            vertices,
            self.vertices.memory,
        )}?;

        Ok(())
    }
}
