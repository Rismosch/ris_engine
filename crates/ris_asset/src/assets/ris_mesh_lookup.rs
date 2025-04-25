use ash::vk;

use ris_asset_data::AssetId;
use ris_asset_data::mesh::GpuMesh;
use ris_asset_data::mesh::MeshLookupId;
use ris_async::OneshotReceiver;
use ris_error::prelude::*;

#[derive(Default)]
pub struct MeshLookup {
    entries: Vec<MeshLookupEntry>,
}

struct MeshLookupEntry {
    id: AssetId,
    reference_count: usize,
    value: Option<MeshLookupEntryState>,
}

enum MeshLookupEntryState {
    Loading(OneshotReceiver<RisResult<GpuMesh>>),
    Loaded(GpuMesh),
}

impl MeshLookup {
    pub fn alloc(
        &mut self,
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        id: AssetId,
    ) -> MeshLookupId {
        let position = self.entries
            .iter()
            .position(|x| x.id == id);

        let (index, entry) = match position {
            Some(position) => {
                let entry = &mut self.entries[position];
                entry.reference_count += 1;
                (position, entry)
            },
            None => {
                let position = self.entries
                    .iter()
                    .position(|x| x.reference_count == 0);

                match position {
                    Some(position) => {
                        let entry = &mut self.entries[position];
                        entry.id = id;
                        entry.reference_count += 1;
                        (position, entry)
                    },
                    None => {
                        let index = self.entries.len();
                        let entry = MeshLookupEntry {
                            id,
                            reference_count: 1,
                            value: None,
                        };
                        self.entries.push(entry);
                        let entry = self.entries.last_mut().unwrap();
                        (index, entry)
                    },
                }
            },
        };

        if entry.value.is_none() {
            let device = device.clone();
            let receiver = crate::load_async(entry.id.clone(), move |bytes| {
                let cpu_mesh = super::ris_mesh::deserialize(&bytes)?;
                unsafe {GpuMesh::from_cpu_mesh(
                    &device,
                    physical_device_memory_properties,
                    cpu_mesh,
                )}
            });

            entry.value = Some(MeshLookupEntryState::Loading(receiver));
        }

        MeshLookupId{index}
    }

    pub fn free(&mut self, device: &ash::Device, id: MeshLookupId) {
        let Some(entry) = self.entries.get_mut(id.index) else {
            return;
        };

        entry.reference_count = entry.reference_count.saturating_sub(1);

        if entry.reference_count != 0 {
            return;
        }

        let mut gpu_mesh = match entry.value.take() {
            Some(MeshLookupEntryState::Loading(receiver)) => match receiver.wait() {
                Ok(gpu_mesh) => gpu_mesh,
                Err(e) => {
                    ris_log::warning!("failed to load mesh {:?}: {}", entry.id, e);
                    return;
                }
            },
            Some(MeshLookupEntryState::Loaded(gpu_mesh)) => gpu_mesh,
            None => return,
        };

        gpu_mesh.free(device);
        ris_log::trace!("freed mesh {:?}", entry.id);
    }

    pub fn get(&mut self, id: MeshLookupId) -> Option<&GpuMesh> {
        let Some(entry) = self.entries.get_mut(id.index) else {
            return None;
        };

        match entry.value.take() {
            Some(MeshLookupEntryState::Loading(receiver)) => match receiver.receive() {
                Ok(Ok(gpu_mesh)) => entry.value = Some(MeshLookupEntryState::Loaded(gpu_mesh)),
                Ok(Err(e)) => {
                    ris_log::error!("failed to load mesh {:?}: {}", entry.id, e);
                    entry.value = None;
                },
                Err(receiver) => entry.value = Some(MeshLookupEntryState::Loading(receiver)),
            },
            value => entry.value = value,
        }

        match entry.value.as_ref() {
            Some(MeshLookupEntryState::Loaded(gpu_mesh)) => Some(gpu_mesh),
            _ => None,
        }
    }
}

