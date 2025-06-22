use std::io::SeekFrom;

use ash::vk;

use ris_error::prelude::*;
use ris_io::FatPtr;
use ris_video_data::buffer::Buffer;

use crate::mesh::Indices;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TerrainVertex(pub i32, pub i32);

#[derive(Debug)]
pub struct TerrainMeshPrototype {
    pub vertices: Vec<TerrainVertex>,
    pub indices: Indices,
}

#[derive(Debug)]
pub struct TerrainCpuMesh {
    pub p_vertices: FatPtr,
    pub p_indices: FatPtr,
    pub index_type: vk::IndexType,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct TerrainGpuMesh {
    inner: Option<TerrainGpuMeshInner>,
}

#[derive(Debug)]
struct TerrainGpuMeshInner {
    p_vertices: vk::DeviceSize,
    p_indices: vk::DeviceSize,
    index_count: u32,
    index_type: vk::IndexType,
    buffer: Buffer,
}

impl TryFrom<TerrainCpuMesh> for TerrainMeshPrototype {
    type Error = RisError;

    fn try_from(value: TerrainCpuMesh) -> Result<Self, Self::Error> {
        let mut stream = std::io::Cursor::new(value.data);
        let s = &mut stream;

        let vertex_bytes = ris_io::read_at(s, value.p_vertices)?;
        let vertex_stride = 8; // 2 ints; sizeof u32: 4; 2 * 4 = 8
        ris_error::assert!(vertex_bytes.len() % vertex_stride == 0)?;
        let vertex_count = vertex_bytes.len() / vertex_stride;

        let index_bytes = ris_io::read_at(s, value.p_indices)?;
        let index_stride = Indices::stride_of(value.index_type);
        ris_error::assert!(index_bytes.len() % index_stride == 0)?;
        let index_count = index_bytes.len() / index_stride;

        let mut stream = std::io::Cursor::new(vertex_bytes);
        let s = &mut stream;
        let mut vertices = Vec::with_capacity(vertex_count);
        for _ in 0..vertex_count {
            let x = ris_io::read_i32(s)?;
            let y = ris_io::read_i32(s)?;
            let vertex = TerrainVertex(x, y);
            vertices.push(vertex);
        }

        let mut stream = std::io::Cursor::new(index_bytes);
        let s = &mut stream;
        let indices = match value.index_type {
            vk::IndexType::UINT16 => {
                let mut indices = Vec::with_capacity(index_count);
                for _ in 0..index_count {
                    let index = ris_io::read_u16(s)?;
                    indices.push(index);
                }

                Indices::U16(indices)
            },
            vk::IndexType::UINT32 => {
                let mut indices = Vec::with_capacity(index_count);
                for _ in 0..index_count {
                    let index = ris_io::read_u32(s)?;
                    indices.push(index);
                }

                Indices::U32(indices)
            },
            vk::IndexType::UINT8_EXT => {
                let mut indices = Vec::with_capacity(index_count);
                for _ in 0..index_count {
                    let index = ris_io::read_u8(s)?;
                    indices.push(index);
                }

                Indices::U8(indices)
            },
            vk::IndexType::NONE_KHR => Indices::None,
            index_type => ris_error::new_result!("unkown index type: {:?}", index_type)?,
        };

        Ok(Self{
            vertices,
            indices,
        })
    }
}

impl TryFrom<TerrainMeshPrototype> for TerrainCpuMesh {
    type Error = RisError;

    fn try_from(value: TerrainMeshPrototype) -> Result<Self, Self::Error> {
        let len = value.vertices.len();

        match &value.indices {
            Indices::U16(indices) => {
                for &index in indices.iter() {
                    let index = usize::from(index);
                    ris_error::assert!(index < len)?;
                }
            },
            Indices::U32(indices) => {
                for &index in indices.iter() {
                    let index = usize::try_from(index)?;
                    ris_error::assert!(index < len)?;
                }
            },
            Indices::U8(indices) => {
                for &index in indices.iter() {
                    let index = usize::from(index);
                    ris_error::assert!(index < len)?;
                }
            },
            Indices::None => (),
        }

        let mut cursor = std::io::Cursor::new(Vec::new());
        let s = &mut cursor;

        let vertices_addr = ris_io::seek(s, SeekFrom::Current(0))?;
        for vertex in value.vertices {
            ris_io::write_i32(s, vertex.0)?;
            ris_io::write_i32(s, vertex.1)?;
        }

        let indices_addr = ris_io::seek(s, SeekFrom::Current(0))?;
        let index_type = match value.indices {
            Indices::U16(indices) => {
                for index in indices {
                    ris_io::write_u16(s, index)?;
                }

                vk::IndexType::UINT16
            },
            Indices::U32(indices) => {
                for index in indices {
                    ris_io::write_u32(s, index)?;
                }

                vk::IndexType::UINT32
            },
            Indices::U8(indices) => {
                for index in indices {
                    ris_io::write_u8(s, index)?;
                }

                vk::IndexType::UINT8_EXT
            },
            Indices::None => vk::IndexType::NONE_KHR,
        };
        let end = ris_io::seek(s, SeekFrom::Current(0))?;

        let p_vertices = FatPtr::begin_end(vertices_addr, indices_addr)?;
        let p_indices = FatPtr::begin_end(indices_addr, end)?;
        let data = cursor.into_inner();

        Ok(TerrainCpuMesh{
            p_vertices,
            p_indices,
            index_type,
            data,
        })
    }
}

impl TerrainGpuMesh {
    pub fn free(&mut self, device: &ash::Device) {
        if let Some(inner) = self.inner.take() {
            unsafe {inner.buffer.free(device)};
        }
    }

    pub fn from_prototype(
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        value: TerrainMeshPrototype,
    ) -> RisResult<Self> {
        let cpu_mesh = TerrainCpuMesh::try_from(value)?;
        unsafe {Self::from_cpu_mesh(device, physical_device_memory_properties, cpu_mesh)}
    }

    /// # Safety
    ///
    /// this method does not validate the CpuMesh. client code must
    /// ensure that the pointers point inside the data array, and the
    /// indices may not index outside the vertex range.
    pub unsafe fn from_cpu_mesh(
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        value: TerrainCpuMesh,
    ) -> RisResult<Self> {
        let p_vertices = value.p_vertices.addr;
        let p_indices = value.p_indices.addr;
        let index_size = Indices::stride_of(value.index_type);
        ris_error::assert!(index_size > 0)?;
        let index_count = value.p_indices.len as u32 / index_size as u32;
        let index_type = value.index_type;

        let buffer = Buffer::alloc(
            device,
            value.data.len() as vk::DeviceSize,
            vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::INDEX_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE
                | vk::MemoryPropertyFlags::HOST_COHERENT
                | vk::MemoryPropertyFlags::DEVICE_LOCAL,
            physical_device_memory_properties,
        )?;
        buffer.write(device, &value.data)?;
        
        Ok(Self{
            inner: Some(TerrainGpuMeshInner {
                p_vertices,
                p_indices,
                index_count,
                index_type,
                buffer,
            })
        })
    }

    pub fn vertex_buffers(&self) -> RisResult<Vec<vk::Buffer>> {
        let inner = self.get_inner()?;
        Ok(vec![inner.buffer.buffer])
    }

    pub fn vertex_offsets(&self) -> RisResult<Vec<vk::DeviceSize>> {
        let inner = self.get_inner()?;
        Ok(vec![inner.p_vertices])
    }

    pub fn index_buffer(&self) -> RisResult<vk::Buffer> {
        let inner = self.get_inner()?;
        Ok(inner.buffer.buffer)
    }

    pub fn index_offset(&self) -> RisResult<vk::DeviceSize> {
        let inner = self.get_inner()?;
        Ok(inner.p_indices)
    }

    pub fn index_count(&self) -> RisResult<u32> {
        let inner = self.get_inner()?;
        Ok(inner.index_count)
    }

    pub fn index_type(&self) -> RisResult<vk::IndexType> {
        let inner = self.get_inner()?;
        Ok(inner.index_type)
    }

    fn get_inner(&self) -> RisResult<&TerrainGpuMeshInner> {
        match self.inner.as_ref() {
            Some(inner) => Ok(inner),
            None => ris_error::new_result!("gpu mesh was freed"),
}
    }
}
