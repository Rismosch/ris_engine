use std::f32::consts::PI;

use ash::vk;

use ris_asset_data::mesh::GpuMesh;
use ris_asset_data::mesh::MeshLookupId;
use ris_asset_data::mesh::MeshPrototype;
use ris_asset_data::terrain_mesh::TerrainMeshPrototype;
use ris_asset_data::AssetId;
use ris_async::OneshotReceiver;
use ris_error::prelude::*;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec2;
use ris_math::vector::Vec3;

use crate::assets::ris_terrain;
use crate::RisGodAsset;

pub struct TerrainMeshRingBuffer {
    mesh_asset_id: AssetId,
    entries: Vec<Entry>,
    head: usize,
    offset: Vec3,
}

struct Entry {
    lookup_id: MeshLookupId,
    value: Option<EntryState>,
}

enum EntryState {
    Loading(OneshotReceiver<RisResult<GpuMesh>>),
    Loaded(GpuMesh),
}

impl TerrainMeshRingBuffer {
    pub fn new(god_asset: &RisGodAsset, entries: usize) -> Self {
        let mut entries = Vec::with_capacity(entries);
        for i in 0..entries.capacity() {
            let entry = Entry {
                lookup_id: MeshLookupId::new(i),
                value: None,
            };

            entries.push(entry);
        }

        Self {
            mesh_asset_id: god_asset.terrain.clone(),
            entries,
            head: 0,
            offset: Vec3(1.0, 1.0, 0.0),
        }
    }

    pub fn free(&mut self, device: &ash::Device) {
        for entry in self.entries.iter_mut() {
            if let Some(mut gpu_mesh) = entry.take_gpu_mesh() {
                gpu_mesh.free(device);
            }
        }
    }

    pub fn alloc(
        &mut self,
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
    ) -> RisResult<bool> {
        let new_head = (self.head + 1) % self.entries.len();
        let entry = &mut self.entries[new_head];
        self.head = new_head;

        if !entry.lookup_id.is_unique() {
            // cannot allocate into the current entry, because the mesh is still in use
            return Ok(false);
        }

        let rotation = Quat::angle_axis(PI / 2.0, Vec3(0.0, 0.0, 1.0));
        self.offset = rotation.rotate(self.offset);

        // allocate into the current entry
        let gpu_mesh = entry.take_gpu_mesh();
        let device = device.clone();
        let offset = self.offset;
        let receiver = crate::load_async(self.mesh_asset_id.clone(), move |bytes| {
            let terrain_cpu_mesh = ris_terrain::deserialize(&bytes)?;
            let TerrainMeshPrototype {
                vertices: terrain_vertices,
                indices,
            } = TerrainMeshPrototype::try_from(terrain_cpu_mesh)?;
            let vertex_count = terrain_vertices.len();

            // vertices
            let mut vertices = Vec::with_capacity(vertex_count);
            for terrain_vertex in terrain_vertices {
                let vertex = Vec3(terrain_vertex.0 as f32, terrain_vertex.1 as f32, 0.0);
                let vertex_with_offset = vertex + offset;
                vertices.push(vertex_with_offset);
            }

            // normals
            let mut normals = Vec::with_capacity(vertices.len());
            for _ in vertices.iter() {
                let normal = Vec3::up();
                normals.push(normal);
            }

            // uvs
            let mut uvs = Vec::with_capacity(vertices.len());
            for _ in vertices.iter() {
                let uv = Vec2(1.0, 1.0);
                uvs.push(uv);
            }

            let mesh_prototype = MeshPrototype {
                vertices,
                normals,
                uvs,
                indices,
            };

            let gpu_mesh = match gpu_mesh {
                Some(mut gpu_mesh) => {
                    // # Safety
                    //
                    // this is safe, because the code above always creates the
                    // same amount of data
                    unsafe { gpu_mesh.overwrite_with_prototype(&device, mesh_prototype) }?;
                    gpu_mesh
                }
                None => GpuMesh::from_prototype(
                    &device,
                    physical_device_memory_properties,
                    mesh_prototype,
                )?,
            };

            Ok(gpu_mesh)
        });

        entry.value = Some(EntryState::Loading(receiver));

        Ok(true)
    }

    pub fn get_latest_id(&mut self) -> Option<MeshLookupId> {
        let mut i = self.head;
        let count = self.entries.len();
        for _ in 0..count {
            let entry = &mut self.entries[i];

            match entry.value.take() {
                Some(EntryState::Loading(receiver)) => match receiver.receive() {
                    Ok(Ok(gpu_mesh)) => {
                        entry.value = Some(EntryState::Loaded(gpu_mesh));
                        let id = entry.lookup_id.clone();
                        return Some(id);
                    }
                    Ok(Err(e)) => ris_log::warning!("failed to load terrain: {}", e),
                    Err(receiver) => entry.value = Some(EntryState::Loading(receiver)),
                },
                value => {
                    entry.value = value;
                    if entry.value.is_some() {
                        let id = entry.lookup_id.clone();
                        return Some(id);
                    }
                }
            }

            if i == 0 {
                i = count - 1;
            } else {
                i -= 1;
            }
        }

        None
    }

    /// # Safety
    ///
    /// only pass MeshLookupIds that were returned from this lookup container. this is
    /// because the id is tied to the GpuMesh. if the id goes out of scope, the mesh
    /// will be freed, potentially screwing a currently running render pipeline.
    ///
    /// additionaly, also because of the reason given above, the MeshLookupId must live
    /// longer than the entire time the GpuMesh is bound.
    pub unsafe fn get(&mut self, id: &MeshLookupId) -> RisResult<&GpuMesh> {
        let entry = self.entries.get(id.index()).into_ris_error()?;
        match entry.value.as_ref() {
            Some(EntryState::Loaded(gpu_mesh)) => Ok(gpu_mesh),
            _ => ris_error::new_result!("invalid id. this is not supposed to happen, as `get_latest_id` should only give out valid ids or return an error"),
        }
    }
}

impl Entry {
    fn take_gpu_mesh(&mut self) -> Option<GpuMesh> {
        match self.value.take() {
            Some(EntryState::Loading(receiver)) => match receiver.wait() {
                Ok(gpu_mesh) => Some(gpu_mesh),
                Err(e) => {
                    ris_log::warning!("failed to load terrain: {}", e);
                    None
                }
            },
            Some(EntryState::Loaded(gpu_mesh)) => Some(gpu_mesh),
            None => None,
        }
    }
}
