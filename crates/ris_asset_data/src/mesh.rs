use std::io::SeekFrom;
use std::sync::Arc;

use ash::vk;

use ris_error::prelude::*;
use ris_io::FatPtr;
use ris_math::vector::Vec2;
use ris_math::vector::Vec3;
use ris_video_data::buffer::Buffer;
use ris_video_data::gpu_io;
use ris_video_data::gpu_io::GpuIOArgs;
use ris_video_data::transient_command::TransientCommandArgs;

#[derive(Debug, Clone)]
pub struct MeshLookupId {
    index: usize,
    references: Arc<()>,
}

impl MeshLookupId {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            references: Arc::default(),
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn is_unique(&mut self) -> bool {
        Arc::get_mut(&mut self.references).is_some()
    }
}

pub const VERTEX_BINDING_DESCRIPTIONS: [vk::VertexInputBindingDescription; 3] = [
    // vertex
    vk::VertexInputBindingDescription {
        binding: 0,
        stride: std::mem::size_of::<Vec3>() as u32,
        input_rate: vk::VertexInputRate::VERTEX,
    },
    // normal
    vk::VertexInputBindingDescription {
        binding: 1,
        stride: std::mem::size_of::<Vec3>() as u32,
        input_rate: vk::VertexInputRate::VERTEX,
    },
    // uv
    vk::VertexInputBindingDescription {
        binding: 2,
        stride: std::mem::size_of::<Vec2>() as u32,
        input_rate: vk::VertexInputRate::VERTEX,
    },
];

pub const VERTEX_ATTRIBUTE_DESCRIPTIONS: [vk::VertexInputAttributeDescription; 3] = [
    // vertex
    vk::VertexInputAttributeDescription {
        location: 0,
        binding: 0,
        format: vk::Format::R32G32B32_SFLOAT,
        offset: 0,
    },
    // normal
    vk::VertexInputAttributeDescription {
        location: 1,
        binding: 1,
        format: vk::Format::R32G32B32_SFLOAT,
        offset: 0,
    },
    // uv
    vk::VertexInputAttributeDescription {
        location: 2,
        binding: 2,
        format: vk::Format::R32G32_SFLOAT,
        offset: 0,
    },
];

#[derive(Debug)]
pub enum Indices {
    U16(Vec<u16>),
    U32(Vec<u32>),
    U8(Vec<u8>),
    None,
}

impl Indices {
    pub fn index_type(&self) -> vk::IndexType {
        match *self {
            Self::U16(_) => vk::IndexType::UINT16,
            Self::U32(_) => vk::IndexType::UINT32,
            Self::U8(_) => vk::IndexType::UINT8_EXT,
            Self::None => vk::IndexType::NONE_KHR,
        }
    }

    pub fn stride(&self) -> usize {
        let index_type = self.index_type();
        Self::stride_of(index_type)
    }

    pub fn stride_of(index_type: vk::IndexType) -> usize {
        match index_type {
            vk::IndexType::UINT16 => std::mem::size_of::<u16>(),
            vk::IndexType::UINT32 => std::mem::size_of::<u32>(),
            vk::IndexType::UINT8_EXT => std::mem::size_of::<u8>(),
            vk::IndexType::NONE_KHR => 0,
            index_type => {
                ris_log::error!("unkown index type: {:?}", index_type);
                0
            }
        }
    }

    pub fn usize_iter(&self) -> IndicesUsizeIter<'_> {
        IndicesUsizeIter {
            indices: self,
            i: 0,
        }
    }

    pub fn triangles(&self) -> TriangleIter<'_> {
        let indices = self.usize_iter();
        TriangleIter { indices }
    }
}

pub struct IndicesUsizeIter<'a> {
    indices: &'a Indices,
    i: usize,
}

pub struct TriangleIter<'a> {
    indices: IndicesUsizeIter<'a>,
}

impl Iterator for IndicesUsizeIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let index = match self.indices {
            Indices::U16(indices) => *indices.get(self.i)? as usize,
            Indices::U32(indices) => *indices.get(self.i)? as usize,
            Indices::U8(indices) => *indices.get(self.i)? as usize,
            Indices::None => return None,
        };

        self.i += 1;
        Some(index)
    }
}

impl Iterator for TriangleIter<'_> {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.indices.next()?;
        let y = self.indices.next()?;
        let z = self.indices.next()?;
        Some((x, y, z))
    }
}

#[derive(Debug)]
pub struct MeshPrototype {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub uvs: Vec<Vec2>,
    pub indices: Indices,
}

#[derive(Debug)]
pub struct CpuMesh {
    pub p_vertices: FatPtr,
    pub p_normals: FatPtr,
    pub p_uvs: FatPtr,
    pub p_indices: FatPtr,
    pub index_type: vk::IndexType,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct GpuMesh {
    inner: Option<GpuMeshInner>,
}

#[derive(Debug)]
struct GpuMeshInner {
    p_vertices: vk::DeviceSize,
    p_normals: vk::DeviceSize,
    p_uvs: vk::DeviceSize,
    p_indices: vk::DeviceSize,
    index_count: u32,
    index_type: vk::IndexType,
    buffer: Buffer,
}

impl TryFrom<CpuMesh> for MeshPrototype {
    type Error = RisError;

    fn try_from(value: CpuMesh) -> Result<Self, Self::Error> {
        let mut stream = std::io::Cursor::new(value.data);
        let s = &mut stream;

        let vertex_bytes = ris_io::read_at(s, value.p_vertices)?;
        let vertex_stride = std::mem::size_of::<Vec3>();
        ris_error::assert!(vertex_bytes.len() % vertex_stride == 0)?;
        let vertex_count = vertex_bytes.len() / vertex_stride;

        let normal_bytes = ris_io::read_at(s, value.p_normals)?;
        let normal_stride = std::mem::size_of::<Vec3>();
        ris_error::assert!(normal_bytes.len() % normal_stride == 0)?;
        let normal_count = normal_bytes.len() / normal_stride;

        let uv_bytes = ris_io::read_at(s, value.p_uvs)?;
        let uv_stride = std::mem::size_of::<Vec2>();
        ris_error::assert!(uv_bytes.len() % uv_stride == 0)?;
        let uv_count = uv_bytes.len() / uv_stride;

        let index_bytes = ris_io::read_at(s, value.p_indices)?;
        let index_stride = Indices::stride_of(value.index_type);
        ris_error::assert!(index_bytes.len() % index_stride == 0)?;
        let index_count = index_bytes.len() / index_stride;

        let mut stream = std::io::Cursor::new(vertex_bytes);
        let s = &mut stream;
        let mut vertices = Vec::with_capacity(vertex_count);
        for _ in 0..vertex_count {
            let vertex = ris_io::read_vec3(s)?;
            vertices.push(vertex);
        }

        let mut stream = std::io::Cursor::new(normal_bytes);
        let s = &mut stream;
        let mut normals = Vec::with_capacity(normal_count);
        for _ in 0..normal_count {
            let normal = ris_io::read_vec3(s)?;
            normals.push(normal);
        }

        let mut stream = std::io::Cursor::new(uv_bytes);
        let s = &mut stream;
        let mut uvs = Vec::with_capacity(uv_count);
        for _ in 0..uv_count {
            let uv = ris_io::read_vec2(s)?;
            uvs.push(uv);
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
            }
            vk::IndexType::UINT32 => {
                let mut indices = Vec::with_capacity(index_count);
                for _ in 0..index_count {
                    let index = ris_io::read_u32(s)?;
                    indices.push(index);
                }

                Indices::U32(indices)
            }
            vk::IndexType::UINT8_EXT => {
                let mut indices = Vec::with_capacity(index_count);
                for _ in 0..index_count {
                    let index = ris_io::read_u8(s)?;
                    indices.push(index);
                }

                Indices::U8(indices)
            }
            vk::IndexType::NONE_KHR => Indices::None,
            index_type => ris_error::new_result!("unkown index type: {:?}", index_type)?,
        };

        Ok(Self {
            vertices,
            normals,
            uvs,
            indices,
        })
    }
}

impl TryFrom<MeshPrototype> for CpuMesh {
    type Error = RisError;

    fn try_from(value: MeshPrototype) -> Result<Self, Self::Error> {
        let len = value.vertices.len();
        ris_error::assert!(value.normals.len() == len)?;
        ris_error::assert!(value.uvs.len() == len)?;

        match &value.indices {
            Indices::U16(indices) => {
                for &index in indices.iter() {
                    let index = usize::from(index);
                    ris_error::assert!(index < len)?;
                }
            }
            Indices::U32(indices) => {
                for &index in indices.iter() {
                    let index = usize::try_from(index)?;
                    ris_error::assert!(index < len)?;
                }
            }
            Indices::U8(indices) => {
                for &index in indices.iter() {
                    let index = usize::from(index);
                    ris_error::assert!(index < len)?;
                }
            }
            Indices::None => (),
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
        let index_type = match value.indices {
            Indices::U16(indices) => {
                for index in indices {
                    ris_io::write_u16(s, index)?;
                }

                vk::IndexType::UINT16
            }
            Indices::U32(indices) => {
                for index in indices {
                    ris_io::write_u32(s, index)?;
                }

                vk::IndexType::UINT32
            }
            Indices::U8(indices) => {
                for index in indices {
                    ris_io::write_u8(s, index)?;
                }

                vk::IndexType::UINT8_EXT
            }
            Indices::None => vk::IndexType::NONE_KHR,
        };
        let end = ris_io::seek(s, SeekFrom::Current(0))?;

        let p_vertices = FatPtr::begin_end(vertices_addr, normals_addr)?;
        let p_normals = FatPtr::begin_end(normals_addr, uv_addr)?;
        let p_uvs = FatPtr::begin_end(uv_addr, indices_addr)?;
        let p_indices = FatPtr::begin_end(indices_addr, end)?;
        let data = cursor.into_inner();

        Ok(CpuMesh {
            p_vertices,
            p_normals,
            p_uvs,
            p_indices,
            index_type,
            data,
        })
    }
}

impl GpuMesh {
    pub fn free(&mut self, device: &ash::Device) {
        if let Some(inner) = self.inner.take() {
            unsafe { inner.buffer.free(device) };
        };
    }

    pub fn from_prototype(
        transient_command_args: TransientCommandArgs,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        value: MeshPrototype,
    ) -> RisResult<Self> {
        let cpu_mesh = CpuMesh::try_from(value)?;
        unsafe {
            Self::from_cpu_mesh(
                transient_command_args,
                physical_device_memory_properties,
                cpu_mesh,
            )
        }
    }

    /// # Safety
    ///
    /// this method does not validate the CpuMesh. client code must
    /// ensure that the pointers point inside the data array, and the
    /// indices may not index outside the vertex range.
    pub unsafe fn from_cpu_mesh(
        transient_command_args: TransientCommandArgs,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        value: CpuMesh,
    ) -> RisResult<Self> {
        let buffer = Buffer::alloc_local(
            &transient_command_args.device,
            value.data.len(),
            vk::BufferUsageFlags::VERTEX_BUFFER
                | vk::BufferUsageFlags::INDEX_BUFFER
                | vk::BufferUsageFlags::TRANSFER_DST,
            physical_device_memory_properties,
        )?;

        let mut gpu_mesh = Self {
            inner: Some(GpuMeshInner {
                p_vertices: Default::default(),
                p_normals: Default::default(),
                p_uvs: Default::default(),
                p_indices: Default::default(),
                index_count: Default::default(),
                index_type: Default::default(),
                buffer,
            }),
        };

        gpu_mesh.overwrite_with_cpu_mesh(
            transient_command_args,
            physical_device_memory_properties,
            value,
        )?;

        Ok(gpu_mesh)
    }

    /// # Safety
    ///
    /// client code must ensure that this GpuMesh is big enough to fit
    /// the prototype
    pub unsafe fn overwrite_with_prototype(
        &mut self,
        transient_command_args: TransientCommandArgs,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        value: MeshPrototype,
    ) -> RisResult<()> {
        let cpu_mesh = CpuMesh::try_from(value)?;
        self.overwrite_with_cpu_mesh(
            transient_command_args,
            physical_device_memory_properties,
            cpu_mesh,
        )
    }

    /// # Safety
    ///
    /// this method does not validate the CpuMesh. client code must
    /// ensure that the pointers point inside the data array, and the
    /// indices may not index outside the vertex range.
    ///
    /// additionally, client code must ensure that this GpuMesh is big
    /// enough to fit the CpuMesh.
    pub unsafe fn overwrite_with_cpu_mesh(
        &mut self,
        transient_command_args: TransientCommandArgs,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        value: CpuMesh,
    ) -> RisResult<()> {
        let Some(inner) = self.inner.as_mut() else {
            return ris_error::new_result!("gpu mesh was freed");
        };

        // prepare ptrs
        let p_vertices = value.p_vertices.addr;
        let p_normals = value.p_normals.addr;
        let p_uvs = value.p_uvs.addr;
        let p_indices = value.p_indices.addr;
        let index_size = match value.index_type {
            vk::IndexType::UINT16 => std::mem::size_of::<u16>(),
            vk::IndexType::UINT32 => std::mem::size_of::<u32>(),
            vk::IndexType::UINT8_EXT => std::mem::size_of::<u8>(),
            vk::IndexType::NONE_KHR => ris_error::new_result!("index type was none")?,
            index_type => ris_error::new_result!("unknown index type: {:?}", index_type)?,
        };

        let index_count = value.p_indices.len as u32 / index_size as u32;
        let index_type = value.index_type;

        // assign values
        inner.p_vertices = p_vertices;
        inner.p_normals = p_normals;
        inner.p_uvs = p_uvs;
        inner.p_indices = p_indices;
        inner.index_count = index_count;
        inner.index_type = index_type;

        // write to gpu
        let device = &transient_command_args.device.clone();
        let staging =
            Buffer::alloc_staging(device, value.data.len(), physical_device_memory_properties)?;
        gpu_io::write_to_buffer(GpuIOArgs {
            transient_command_args,
            values: &value.data,
            gpu_object: &inner.buffer,
            staging: &staging,
        })?;

        staging.free(device);

        Ok(())
    }

    pub fn vertex_buffers(&self) -> RisResult<Vec<vk::Buffer>> {
        let inner = self.get_inner()?;
        Ok(vec![
            inner.buffer.buffer,
            inner.buffer.buffer,
            inner.buffer.buffer,
        ])
    }

    pub fn vertex_offsets(&self) -> RisResult<Vec<vk::DeviceSize>> {
        let inner = self.get_inner()?;
        Ok(vec![inner.p_vertices, inner.p_normals, inner.p_uvs])
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

    fn get_inner(&self) -> RisResult<&GpuMeshInner> {
        match self.inner.as_ref() {
            Some(inner) => Ok(inner),
            None => ris_error::new_result!("gpu mesh was freed"),
        }
    }
}
