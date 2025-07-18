use ash::vk;

use ris_asset_data::mesh::CpuMesh;
use ris_asset_data::mesh::GpuMesh;
use ris_asset_data::mesh::MeshLookupId;
use ris_asset_data::AssetId;
use ris_async::OneshotReceiver;
use ris_error::prelude::*;

#[derive(Default)]
pub struct TerrainMeshRingBuffer {
    entries: Vec<Entry>,
    head: usize,
}

struct Entry{
    lookup_id: MeshLookupId,
    value: Option<EntryState>,
}

enum EntryState {
    Loading(OneshotReceiver<RisResult<GpuMesh>>),
    Loaded(GpuMesh),
}

impl TerrainMeshRingBuffer {
    pub fn free(&mut self, device: &ash::Device) {
        for entry in self.entries.iter_mut() {
            if let Some(mut gpu_mesh) = entry.take_gpu_mesh() {
                gpu_mesh.free(device);
            }
        }
    }

    pub fn alloc(&mut self, device: &ash::Device) -> RisResult<bool> {
        let new_head = (self.head + 1) % self.entries.len();
        let entry = &mut self.entries[new_head];

        if !entry.lookup_id.is_unique() {
            // cannot allocate into the current entry, because the mesh is still in use
            return Ok(false);
        }

        if let Some(mut gpu_mesh) = entry.take_gpu_mesh() {
            gpu_mesh.free(device);
        }

        todo!("create mesh")
    }

    pub fn get_latest_id(&mut self) -> Option<MeshLookupId> {
        let mut i = self.head;
        let count = self.entries.len();
        for _ in 0.. {
            let mut entry = &mut self.entries[i];

            match entry.value.take() {
                Some(EntryState::Loading(receiver)) => match receiver.receive() {
                    Ok(Ok(gpu_mesh)) => {
                        entry.value = Some(EntryState::Loaded(gpu_mesh));
                        let id = entry.lookup_id.clone();
                        return Some(id)
                    },
                    Ok(Err(e)) => ris_log::warning!("failed to load terrain: {}", e),
                    Err(receiver) => entry.value = Some(EntryState::Loading(receiver)),
                },
                value => {
                    entry.value = value;
                    if entry.value.is_some() {
                        let id = entry.lookup_id.clone();
                        return Some(id);
                    }
                },
            }

            if i == 0 {
                i = count - 1;
            } else {
                i -= 1;
            }
        }

        None
    }

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

