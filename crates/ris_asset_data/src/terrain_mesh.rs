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

impl TryFrom<TerrainGpuMesh> for TerrainMeshPrototype {
    type Error = RisError;

    fn try_from(value: TerrainGpuMesh) -> Result<Self, Self::Error> {
        todo;
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
        todo;
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
