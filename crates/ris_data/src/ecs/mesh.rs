use ash::vk;

use ris_error::RisResult;
use ris_math::vector::Vec2;
use ris_math::vector::Vec3;
use ris_video_data::buffer::Buffer;

use super::decl::EcsTypeId;
use super::id::EcsObject;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Vertex {
    pub pos: Vec3,
    pub uv: Vec2,
}

#[derive(Debug, Default)]
pub struct Mesh {
    pub is_dirty: bool,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

#[derive(Debug, Default)]
pub struct VideoMesh {
    inner: Option<VideoMeshInner>,
}

#[derive(Debug)]
struct VideoMeshInner {
    vertices: Buffer,
    vertex_count: usize,
    indices: Buffer,
    index_count: usize,
}

impl Mesh {
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn set_vertices(&mut self, value: &[Vertex]) {
        self.is_dirty = true;
        ris_util::vec::fast_copy(&mut self.vertices, value);
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    pub fn set_indices(&mut self, value: &[u32]) {
        self.is_dirty = true;
        ris_util::vec::fast_copy(&mut self.indices, value);
    }
}

impl EcsObject for VideoMesh {
    fn ecs_type_id() -> EcsTypeId {
        EcsTypeId::VideoMesh
    }
}

impl VideoMesh {
    pub unsafe fn free(&mut self, device: &ash::Device) {
        if let Some(inner) = self.inner.take() {
            inner.vertices.free(device);
            inner.indices.free(device);
        }
    }

    pub unsafe fn alloc(
        &mut self,
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        mesh: Mesh,
    ) -> RisResult<()> {
        ris_log::warning!("IMPLEMENT STAGING, https://vulkan-tutorial.com/Vertex_buffers/Staging_buffer");

        if self.inner.is_some() {
            return ris_error::new_result!("video mesh is already allocated");
        }

        // vertices
        let vertices = mesh.vertices();
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

        // indices
        let indices = mesh.indices();
        let index_buffer_size = std::mem::size_of_val(indices) as vk::DeviceSize;
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

        let inner = VideoMeshInner {
            vertices: vertex_buffer,
            vertex_count: vertices.len(),
            indices: index_buffer,
            index_count: indices.len(),
        };

        self.inner = Some(inner);

        Ok(())
    }
}

//
// primitives
//

impl Mesh {
    pub fn primitive_cube() -> Self {
        Self {
            is_dirty: false,
            vertices: vec![
                // pos x
                Vertex {
                    pos: Vec3(0.5, -0.5, 0.5),
                    uv: Vec2(0.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, 0.5, 0.5),
                    uv: Vec2(1.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, 0.5, -0.5),
                    uv: Vec2(1.0, 1.0),
                },
                Vertex {
                    pos: Vec3(0.5, -0.5, -0.5),
                    uv: Vec2(0.0, 1.0),
                },
                // pos y
                Vertex {
                    pos: Vec3(0.5, 0.5, 0.5),
                    uv: Vec2(0.0, 0.0),
                },
                Vertex {
                    pos: Vec3(-0.5, 0.5, 0.5),
                    uv: Vec2(1.0, 0.0),
                },
                Vertex {
                    pos: Vec3(-0.5, 0.5, -0.5),
                    uv: Vec2(1.0, 1.0),
                },
                Vertex {
                    pos: Vec3(0.5, 0.5, -0.5),
                    uv: Vec2(0.0, 1.0),
                },
                // pos z
                Vertex {
                    pos: Vec3(-0.5, 0.5, 0.5),
                    uv: Vec2(0.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, 0.5, 0.5),
                    uv: Vec2(1.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, -0.5, 0.5),
                    uv: Vec2(1.0, 1.0),
                },
                Vertex {
                    pos: Vec3(-0.5, -0.5, 0.5),
                    uv: Vec2(0.0, 1.0),
                },
                // neg x
                Vertex {
                    pos: Vec3(-0.5, 0.5, 0.5),
                    uv: Vec2(0.0, 0.0),
                },
                Vertex {
                    pos: Vec3(-0.5, -0.5, 0.5),
                    uv: Vec2(1.0, 0.0),
                },
                Vertex {
                    pos: Vec3(-0.5, -0.5, -0.5),
                    uv: Vec2(1.0, 1.0),
                },
                Vertex {
                    pos: Vec3(-0.5, 0.5, -0.5),
                    uv: Vec2(0.0, 1.0),
                },
                // neg y
                Vertex {
                    pos: Vec3(-0.5, -0.5, 0.5),
                    uv: Vec2(0.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, -0.5, 0.5),
                    uv: Vec2(1.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, -0.5, -0.5),
                    uv: Vec2(1.0, 1.0),
                },
                Vertex {
                    pos: Vec3(-0.5, -0.5, -0.5),
                    uv: Vec2(0.0, 1.0),
                },
                // neg z
                Vertex {
                    pos: Vec3(-0.5, -0.5, -0.5),
                    uv: Vec2(0.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, -0.5, -0.5),
                    uv: Vec2(1.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, 0.5, -0.5),
                    uv: Vec2(1.0, 1.0),
                },
                Vertex {
                    pos: Vec3(-0.5, 0.5, -0.5),
                    uv: Vec2(0.0, 1.0),
                },
            ],
            indices: vec![
                0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12,
                16, 17, 18, 18, 19, 16, 20, 21, 22, 22, 23, 20,
            ],
        }
    }
}
