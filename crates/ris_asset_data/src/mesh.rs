use std::io::SeekFrom;

use ash::vk;

use ris_error::prelude::*;
use ris_io::FatPtr;
use ris_math::vector::Vec2;
use ris_math::vector::Vec3;
use ris_video_data::buffer::Buffer;

pub const VERTEX_BINDING_DESCRIPTIONS: [vk::VertexInputBindingDescription; 3] = [
    // vertex
    vk::VertexInputBindingDescription{
        binding: 0,
        stride: std::mem::size_of::<Vec3>() as u32,
        input_rate: vk::VertexInputRate::VERTEX,
    },
    // normal
    vk::VertexInputBindingDescription{
        binding: 1,
        stride: std::mem::size_of::<Vec3>() as u32,
        input_rate: vk::VertexInputRate::VERTEX,
    },
    // uv
    vk::VertexInputBindingDescription{
        binding: 2,
        stride: std::mem::size_of::<Vec2>() as u32,
        input_rate: vk::VertexInputRate::VERTEX,
    },
];

pub const VERTEX_ATTRIBUTE_DESCRIPTIONS: [vk::VertexInputAttributeDescription; 3] = [
    // vertex
    vk::VertexInputAttributeDescription{
        location: 0,
        binding: 0,
        format: vk::Format::R32G32B32_SFLOAT,
        offset: 0,
    },
    // normal
    vk::VertexInputAttributeDescription{
        location: 1,
        binding: 1,
        format: vk::Format::R32G32B32_SFLOAT,
        offset: 0,
    },
    // uv
    vk::VertexInputAttributeDescription{
        location: 2,
        binding: 2,
        format: vk::Format::R32G32_SFLOAT,
        offset: 0,
    },
];

pub const INDEX_TYPE: vk::IndexType = vk::IndexType::UINT16;

pub struct MeshPrototype {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub uvs: Vec<Vec2>,
    pub indices: Vec<u16>,
}

pub struct CpuMesh {
    pub p_vertices: FatPtr,
    pub p_normals: FatPtr,
    pub p_uvs: FatPtr,
    pub p_indices: FatPtr,
    pub data: Vec<u8>,
}

pub struct GpuMesh {
    inner: Option<GpuMeshInner>,
}

struct GpuMeshInner {
    p_vertices: vk::DeviceSize,
    p_normals: vk::DeviceSize,
    p_uvs: vk::DeviceSize,
    p_indices: vk::DeviceSize,
    index_count: u32,
    buffer: Buffer,
}

impl TryFrom<MeshPrototype> for CpuMesh {
    type Error = RisError;

    fn try_from(value: MeshPrototype) -> Result<Self, Self::Error> {
        let len = value.vertices.len();
        ris_error::assert!(value.normals.len() == len)?;
        ris_error::assert!(value.uvs.len() == len)?;
        for &index in value.indices.iter() {
            let index = usize::try_from(index)?;
            ris_error::assert!(index < len)?;
        }

        let mut cursor = std::io::Cursor::new(Vec::new());
        let s = &mut cursor;

        let vertices_addr = ris_io::seek(s, SeekFrom::Current(0))?;
        for vertex in value.vertices {
            ris_io::write_vec3(s, vertex)?;
        }
        let normals_addr = ris_io::seek(s, SeekFrom::Current(0))?;
        for normal in value.normals {
            ris_io::write_vec3(s, normal)?;
        }
        let uv_addr = ris_io::seek(s, SeekFrom::Current(0))?;
        for uv in value.uvs {
            ris_io::write_vec2(s, uv)?;
        }
        let indices_addr = ris_io::seek(s, SeekFrom::Current(0))?;
        for index in value.indices {
            ris_io::write_u16(s, index)?;
        }
        let end = ris_io::seek(s, SeekFrom::Current(0))?;

        let p_vertices = FatPtr::begin_end(vertices_addr, normals_addr)?;
        let p_normals = FatPtr::begin_end(normals_addr, uv_addr)?;
        let p_uvs = FatPtr::begin_end(uv_addr, indices_addr)?;
        let p_indices = FatPtr::begin_end(indices_addr, end)?;
        let data = cursor.into_inner();

        Ok(CpuMesh{
            p_vertices,
            p_normals,
            p_uvs,
            p_indices,
            data,
        })
    }
}

impl GpuMesh {
    pub fn free(&mut self, device: &ash::Device) {
        if let Some(inner) = self.inner.take() {
            unsafe {inner.buffer.free(device)};
        };
    }

    pub fn from_prototype(
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        value: MeshPrototype,
    ) -> RisResult<Self> {
        let cpu_mesh = CpuMesh::try_from(value)?;
        unsafe {Self::from_cpu_mesh(
            device,
            physical_device_memory_properties,
            cpu_mesh,
        )}
    }

    /// # Safety
    ///
    /// this method does not validate the CpuMesh. client code must 
    /// ensure that the pointers point inside the data array, and the 
    /// indices may not index outside the vertex range.
    pub unsafe fn from_cpu_mesh(
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        value: CpuMesh,
    ) -> RisResult<Self> {
        let p_vertices = value.p_vertices.addr;
        let p_normals = value.p_normals.addr;
        let p_uvs = value.p_uvs.addr;
        let p_indices = value.p_indices.addr;
        let index_count = value.p_indices.len as u32 / std::mem::size_of::<u16>() as u32;

        let buffer = Buffer::alloc(
            device,
            value.data.len() as vk::DeviceSize,
            vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::INDEX_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE
                | vk::MemoryPropertyFlags::HOST_COHERENT
                | vk::MemoryPropertyFlags::DEVICE_LOCAL,
            physical_device_memory_properties,
        )?;
        unsafe {buffer.write(device, &value.data)}?;

        Ok(GpuMesh{inner: Some(GpuMeshInner{
            p_vertices,
            p_normals,
            p_uvs,
            p_indices,
            index_count,
            buffer,
        })})
    }

    pub fn vertex_buffers(&self) -> RisResult<Vec<vk::Buffer>> {
        let Some(inner) = self.inner.as_ref() else {
            return ris_error::new_result!("gpu mesh was freed");
        }; 

        Ok(vec![
            inner.buffer.buffer,
            inner.buffer.buffer,
            inner.buffer.buffer,
        ])
    }

    pub fn vertex_offsets(&self) -> RisResult<Vec<vk::DeviceSize>> {
        let Some(inner) = self.inner.as_ref() else {
            return ris_error::new_result!("gpu mesh was freed");
        }; 

        Ok(vec![
           inner.p_vertices,
           inner.p_normals,
           inner.p_uvs,
        ])
    }

    pub fn index_buffer(&self) -> RisResult<vk::Buffer> {
        let Some(inner) = self.inner.as_ref() else {
            return ris_error::new_result!("gpu mesh was freed");
        }; 

        Ok(inner.buffer.buffer)
    }

    pub fn index_offset(&self) -> RisResult<vk::DeviceSize> {
        let Some(inner) = self.inner.as_ref() else {
            return ris_error::new_result!("gpu mesh was freed");
        }; 

        Ok(inner.p_indices)
    }

    pub fn index_count(&self) -> RisResult<u32> {
        let Some(inner) = self.inner.as_ref() else {
            return ris_error::new_result!("gpu mesh was freed");
        }; 

        Ok(inner.index_count)
    }

    pub fn index_type(&self) -> vk::IndexType {
        INDEX_TYPE
    }
}
