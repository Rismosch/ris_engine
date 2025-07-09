use ash::vk;

use ris_asset_data::mesh::GpuMesh;
use ris_asset_data::mesh::MeshLookupId;
use ris_asset_data::AssetId;
use ris_async::OneshotReceiver;
use ris_error::prelude::*;

#[derive(Default)]
pub struct TerrainMeshLookup {
    entries: Vec<Entry>,
}

struct Entry {
    lookup_id: MeshLookupId,
    value: Option<EntryState>,
}

enum EntryState {
    Loading(OneshotReceiver<RisResult<GpuMesh>>),
    Loaded(GpuMesh),
}


impl TerrainMeshLookup {
    pub fn free(&mut self, device: &ash::Device) {
        for entry in self.entries.iter_mut() {
            if let Some(mut gpu_mesh) = entry.take_gpu_mesh() {
                gpu_mesh.free(device);
            }
        }
    }


    pub fn alloc(&mut self) -> MeshLookupId {
        panic!();
    }
}

impl Entry {
    fn take_gpu_mesh(&mut self) -> Option<GpuMesh> {
        //match self.value.take() {
        //    Some(EntryState::Loading(receiver)) => match receiver.wait() {
        //        Ok(gpu_mesh) => Some(gpu_mesh),
        //        Err(e) => {
        //            ris_log::warning!("failed to load mesh {:?}: {}", self.asset_id, e);
        //            None
        //        }
        //    },
        //    Some(EntryState::Loaded(gpu_mesh)) => Some(gpu_mesh),
        //    None => None,
        //}
        panic!()
    }
}

impl EntryState {
    fn load(
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        asset_id: AssetId,
    ) -> Self {
        //let device = device.clone();
        //let receiver = crate::load_async(asset_id, move |bytes| {
        //    let cpu_mesh = ris_mesh::deserialize(&bytes)?;
        //    unsafe { GpuMesh::from_cpu_mesh(&device, physical_device_memory_properties, cpu_mesh) }
        //});

        //EntryState::Loading(receiver)
        panic!()
    }
}
