use ash::vk;

use ris_error::RisResult;
use ris_math::color::Rgb;
use ris_math::matrix::Mat4;
use ris_math::vector::Vec3;
use ris_math::vector::Vec2;

use crate::vulkan::buffer::Buffer;

#[repr(C)]
pub struct Vertex {
    pub pos: Vec3,
    pub color: Rgb,
    pub uv: Vec2,
}

pub struct Mesh {
    pub vertices: Buffer,
    pub vertex_count: usize,
    pub indices: Buffer,
    pub index_count: usize,
}

impl Mesh {
    pub unsafe fn free(&mut self, device: &ash::Device) {
        self.vertices.free(device);
        self.indices.free(device);
    }

    pub unsafe fn alloc(
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        vertices: &[Vertex],
        indices: &[u32],
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

        let index_buffer_size = std::mem::size_of_val(indices) as vk::DeviceSize;
        let index_buffer = Buffer::alloc(
            device,
            vertex_buffer_size,
            vk::BufferUsageFlags::INDEX_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE
                | vk::MemoryPropertyFlags::HOST_COHERENT
                | vk::MemoryPropertyFlags::DEVICE_LOCAL,
            physical_device_memory_properties,
        )?;
        index_buffer.write(device, indices);

        Ok(Self{
            vertices: vertex_buffer,
            vertex_count: vertices.len(),
            indices: index_buffer,
            index_count: indices.len(),
        })
    }

    pub fn update(
        &mut self,
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        vertices: &[Vertex],
        indices: &[u32],
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

            unsafe {old_buffer.free(device)};
        }
        unsafe {self.vertices.write(device, &vertices)}?;

        let old_index_count = self.index_count;
        let new_index_count = indices.len();
        if old_index_count < new_index_count {
            let index_buffer_size = std::mem::size_of_val(indices) as vk::DeviceSize;
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

pub const VERTICES: [Vertex; 4 * 6] = [
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
];

pub const INDICES: [u32; 6 * 6] = [
    0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12, 16, 17, 18,
    18, 19, 16, 20, 21, 22, 22, 23, 20,
];
