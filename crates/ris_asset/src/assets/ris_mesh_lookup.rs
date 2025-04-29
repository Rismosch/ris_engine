use ash::vk;

use ris_asset_data::mesh::GpuMesh;
use ris_asset_data::mesh::MeshLookupId;
use ris_asset_data::AssetId;
use ris_async::OneshotReceiver;
use ris_error::prelude::*;

#[derive(Default)]
pub struct MeshLookup {
    entries: Vec<MeshLookupEntry>,
}

struct MeshLookupEntry {
    asset_id: AssetId,
    lookup_id: MeshLookupId,
    value: Option<MeshLookupEntryState>,
}

enum MeshLookupEntryState {
    Loading(OneshotReceiver<RisResult<GpuMesh>>),
    Loaded(GpuMesh),
}

impl MeshLookup {
    pub fn free(&mut self, device: &ash::Device) {
        for entry in self.entries.iter_mut() {
            if let Some(mut gpu_mesh) = entry.take_gpu_mesh() {
                gpu_mesh.free(device);
            }
        }
    }

    pub fn reimport_everything(
        &mut self,
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
    ) {
        for entry in self.entries.iter_mut() {
            if entry.value.is_none() {
                continue;
            }

            if let Some(mut gpu_mesh) = entry.take_gpu_mesh() {
                gpu_mesh.free(device);
            }

            let state = MeshLookupEntryState::load(
                device,
                physical_device_memory_properties,
                entry.asset_id.clone(),
            );
            entry.value = Some(state);
        }
    }

    pub fn alloc(
        &mut self,
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        asset_id: AssetId,
    ) -> MeshLookupId {
        let position = self.entries.iter().position(|x| x.asset_id == asset_id);

        let entry = match position {
            Some(position) => &mut self.entries[position],
            None => {
                let position = self
                    .entries
                    .iter_mut()
                    .position(|x| x.lookup_id.is_unique());

                match position {
                    Some(position) => {
                        let entry = &mut self.entries[position];
                        entry.asset_id = asset_id;
                        entry
                    }
                    None => {
                        let index = self.entries.len();
                        let entry = MeshLookupEntry {
                            asset_id,
                            lookup_id: MeshLookupId::new(index),
                            value: None,
                        };
                        self.entries.push(entry);
                        let entry = self.entries.last_mut().unwrap();
                        entry
                    }
                }
            }
        };

        if entry.value.is_none() {
            //let device = device.clone();
            //let receiver = crate::load_async(entry.asset_id.clone(), move |bytes| {
            //    let cpu_mesh = super::ris_mesh::deserialize(&bytes)?;
            //    unsafe {
            //        GpuMesh::from_cpu_mesh(&device, physical_device_memory_properties, cpu_mesh)
            //    }
            //});

            //entry.value = Some(MeshLookupEntryState::Loading(receiver));

            let state = MeshLookupEntryState::load(
                device,
                physical_device_memory_properties,
                entry.asset_id.clone(),
            );
            entry.value = Some(state);
        }

        entry.lookup_id.clone()
    }

    pub fn free_unused_meshes(&mut self, device: &ash::Device) -> RisResult<()> {
        let mut must_wait = true;

        for entry in self.entries.iter_mut() {
            if !entry.lookup_id.is_unique() {
                continue;
            }

            if must_wait {
                unsafe { device.device_wait_idle() }?;
                must_wait = false;
            }

            if let Some(mut gpu_mesh) = entry.take_gpu_mesh() {
                gpu_mesh.free(device);
                ris_log::trace!("freed mesh {:?}", entry.asset_id);
            }
        }

        Ok(())
    }

    pub fn get(&mut self, id: &MeshLookupId) -> Option<&GpuMesh> {
        let entry = self.entries.get_mut(id.index())?;

        match entry.value.take() {
            Some(MeshLookupEntryState::Loading(receiver)) => match receiver.receive() {
                Ok(Ok(gpu_mesh)) => entry.value = Some(MeshLookupEntryState::Loaded(gpu_mesh)),
                Ok(Err(e)) => {
                    ris_log::error!("failed to load mesh {:?}: {}", entry.asset_id, e);
                    entry.value = None;
                }
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

impl MeshLookupEntry {
    fn take_gpu_mesh(&mut self) -> Option<GpuMesh> {
        match self.value.take() {
            Some(MeshLookupEntryState::Loading(receiver)) => match receiver.wait() {
                Ok(gpu_mesh) => Some(gpu_mesh),
                Err(e) => {
                    ris_log::warning!("failed to load mesh {:?}: {}", self.asset_id, e);
                    None
                }
            },
            Some(MeshLookupEntryState::Loaded(gpu_mesh)) => Some(gpu_mesh),
            None => None,
        }
    }
}

impl MeshLookupEntryState {
    fn load(
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        asset_id: AssetId,
    ) -> Self {
        let device = device.clone();
        let receiver = crate::load_async(asset_id, move |bytes| {
            let cpu_mesh = super::ris_mesh::deserialize(&bytes)?;
            unsafe { GpuMesh::from_cpu_mesh(&device, physical_device_memory_properties, cpu_mesh) }
        });

        MeshLookupEntryState::Loading(receiver)
    }
}
