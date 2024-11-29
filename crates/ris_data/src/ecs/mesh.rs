use ash::vk;

use ris_error::RisResult;
use ris_math::color::Rgb;
use ris_math::vector::Vec2;
use ris_math::vector::Vec3;
use ris_video_data::buffer::Buffer;

use super::decl::VideoMeshHandle;
use super::id::SceneKind;
use super::scene::Scene;
use super::vertex::Vertex;

#[derive(Debug, Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
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

impl VideoMesh {
    pub fn free(&mut self, device: &ash::Device) {
        let Some(inner) = self.inner.take() else {
            return;
        };

        unsafe {
            inner.vertices.free(device);
            inner.indices.free(device);
        }
    }
}

impl VideoMeshHandle {
    pub fn new(scene: &Scene) -> RisResult<Self> {
        let ptr = scene.create_new::<VideoMesh>(SceneKind::Other)?;
        Ok(ptr.borrow().handle.into())
    }

    pub fn free(self, scene: &Scene, device: &ash::Device) -> RisResult<()> {
        let ptr = scene.deref(self.into())?;
        let mut aref_mut = ptr.borrow_mut();
        aref_mut.free(device);

        Ok(())
    }

    pub fn upload(
        self,
        scene: &Scene,
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        mesh: Mesh,
    ) -> RisResult<()> {
        let ptr = scene.deref(self.into())?;
        if ptr.borrow_mut().inner.is_some() {
            return ris_error::new_result!("video mesh already stores an uploaded mesh");
        }

        // vertices
        let vertices = mesh.vertices.as_slice();
        let vertex_buffer_size = std::mem::size_of_val(vertices) as vk::DeviceSize;

        //ris_log::trace!(
        //    "uploading {} vertices with size {}...",
        //    vertices.len(),
        //    vertex_buffer_size
        //);
        let vertex_buffer = unsafe {
            let buffer = Buffer::alloc(
                device,
                vertex_buffer_size,
                vk::BufferUsageFlags::VERTEX_BUFFER,
                vk::MemoryPropertyFlags::HOST_VISIBLE
                    | vk::MemoryPropertyFlags::HOST_COHERENT
                    | vk::MemoryPropertyFlags::DEVICE_LOCAL,
                physical_device_memory_properties,
            )?;
            buffer.write(device, vertices)?;
            buffer
        };

        // indices
        let indices = mesh.indices.as_slice();
        let index_buffer_size = std::mem::size_of_val(indices) as vk::DeviceSize;

        //ris_log::trace!(
        //    "uploading {} indices with size {}...",
        //    indices.len(),
        //    index_buffer_size
        //);
        let index_buffer = unsafe {
            let buffer = Buffer::alloc(
                device,
                index_buffer_size,
                vk::BufferUsageFlags::INDEX_BUFFER,
                vk::MemoryPropertyFlags::HOST_VISIBLE
                    | vk::MemoryPropertyFlags::HOST_COHERENT
                    | vk::MemoryPropertyFlags::DEVICE_LOCAL,
                physical_device_memory_properties,
            )?;
            buffer.write(device, indices)?;
            buffer
        };

        let inner = VideoMeshInner {
            vertices: vertex_buffer,
            vertex_count: vertices.len(),
            indices: index_buffer,
            index_count: indices.len(),
        };

        ptr.borrow_mut().inner = Some(inner);

        Ok(())
    }

    pub fn vertices(self, scene: &Scene) -> RisResult<Option<Buffer>> {
        let ptr = scene.deref(self.into())?;
        let buffer = ptr.borrow().inner.as_ref().map(|x| x.vertices);
        Ok(buffer)
    }

    pub fn vertex_count(self, scene: &Scene) -> RisResult<Option<usize>> {
        let ptr = scene.deref(self.into())?;
        let count = ptr.borrow().inner.as_ref().map(|x| x.vertex_count);
        Ok(count)
    }

    pub fn indices(self, scene: &Scene) -> RisResult<Option<Buffer>> {
        let ptr = scene.deref(self.into())?;
        let buffer = ptr.borrow().inner.as_ref().map(|x| x.indices);
        Ok(buffer)
    }

    pub fn index_count(self, scene: &Scene) -> RisResult<Option<usize>> {
        let ptr = scene.deref(self.into())?;
        let count = ptr.borrow().inner.as_ref().map(|x| x.index_count);
        Ok(count)
    }
}

//
// primitives
//

impl Mesh {
    pub fn primitive_cube() -> Self {
        Self {
            vertices: vec![
                // pos x
                Vertex {
                    pos: Vec3(0.5, -0.5, 0.5),
                    color: Rgb(1.0, 0.0, 0.0),
                    uv: Vec2(0.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, 0.5, 0.5),
                    color: Rgb(1.0, 0.0, 0.0),
                    uv: Vec2(1.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, 0.5, -0.5),
                    color: Rgb(1.0, 0.0, 0.0),
                    uv: Vec2(1.0, 1.0),
                },
                Vertex {
                    pos: Vec3(0.5, -0.5, -0.5),
                    color: Rgb(1.0, 0.0, 0.0),
                    uv: Vec2(0.0, 1.0),
                },
                // pos y
                Vertex {
                    pos: Vec3(0.5, 0.5, 0.5),
                    color: Rgb(0.0, 1.0, 0.0),
                    uv: Vec2(0.0, 0.0),
                },
                Vertex {
                    pos: Vec3(-0.5, 0.5, 0.5),
                    color: Rgb(0.0, 1.0, 0.0),
                    uv: Vec2(1.0, 0.0),
                },
                Vertex {
                    pos: Vec3(-0.5, 0.5, -0.5),
                    color: Rgb(0.0, 1.0, 0.0),
                    uv: Vec2(1.0, 1.0),
                },
                Vertex {
                    pos: Vec3(0.5, 0.5, -0.5),
                    color: Rgb(0.0, 1.0, 0.0),
                    uv: Vec2(0.0, 1.0),
                },
                // pos z
                Vertex {
                    pos: Vec3(-0.5, 0.5, 0.5),
                    color: Rgb(0.0, 0.0, 1.0),
                    uv: Vec2(0.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, 0.5, 0.5),
                    color: Rgb(0.0, 0.0, 1.0),
                    uv: Vec2(1.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, -0.5, 0.5),
                    color: Rgb(0.0, 0.0, 1.0),
                    uv: Vec2(1.0, 1.0),
                },
                Vertex {
                    pos: Vec3(-0.5, -0.5, 0.5),
                    color: Rgb(0.0, 0.0, 1.0),
                    uv: Vec2(0.0, 1.0),
                },
                // neg x
                Vertex {
                    pos: Vec3(-0.5, 0.5, 0.5),
                    color: Rgb(0.0, 1.0, 1.0),
                    uv: Vec2(0.0, 0.0),
                },
                Vertex {
                    pos: Vec3(-0.5, -0.5, 0.5),
                    color: Rgb(0.0, 1.0, 1.0),
                    uv: Vec2(1.0, 0.0),
                },
                Vertex {
                    pos: Vec3(-0.5, -0.5, -0.5),
                    color: Rgb(0.0, 1.0, 1.0),
                    uv: Vec2(1.0, 1.0),
                },
                Vertex {
                    pos: Vec3(-0.5, 0.5, -0.5),
                    color: Rgb(0.0, 1.0, 1.0),
                    uv: Vec2(0.0, 1.0),
                },
                // neg y
                Vertex {
                    pos: Vec3(-0.5, -0.5, 0.5),
                    color: Rgb(1.0, 0.0, 1.0),
                    uv: Vec2(0.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, -0.5, 0.5),
                    color: Rgb(1.0, 0.0, 1.0),
                    uv: Vec2(1.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, -0.5, -0.5),
                    color: Rgb(1.0, 0.0, 1.0),
                    uv: Vec2(1.0, 1.0),
                },
                Vertex {
                    pos: Vec3(-0.5, -0.5, -0.5),
                    color: Rgb(1.0, 0.0, 1.0),
                    uv: Vec2(0.0, 1.0),
                },
                // neg z
                Vertex {
                    pos: Vec3(-0.5, -0.5, -0.5),
                    color: Rgb(1.0, 1.0, 0.0),
                    uv: Vec2(0.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, -0.5, -0.5),
                    color: Rgb(1.0, 1.0, 0.0),
                    uv: Vec2(1.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, 0.5, -0.5),
                    color: Rgb(1.0, 1.0, 0.0),
                    uv: Vec2(1.0, 1.0),
                },
                Vertex {
                    pos: Vec3(-0.5, 0.5, -0.5),
                    color: Rgb(1.0, 1.0, 0.0),
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
