use ash::vk;

use ris_asset_data::mesh::CpuMesh;
use ris_asset_data::mesh::GpuMesh;
use ris_asset_data::mesh::MeshLookupId;
use ris_asset_data::AssetId;
use ris_async::OneshotReceiver;
use ris_data::counter::Counter;
use ris_error::prelude::*;

#[derive(Default)]
pub struct TerrainMeshLookup {
    entries: Vec<Entry>,
}

struct Entry {
    // counter_id determines how young this mesh is. the bigger the counter, the younger the mesh
    counter_id: Counter,
    lookup_id: MeshLookupId,
    value: Option<EntryState>,
}

enum EntryState {
    Loading(OneshotReceiver<RisResult<(CpuMesh, GpuMesh)>>),
    Loaded((CpuMesh, GpuMesh)),
}

impl TerrainMeshLookup {
    pub fn free(&mut self, device: &ash::Device) {
        //for entry in self.entries.iter_mut() {
        //    if let Some(mut gpu_mesh) = entry.take_gpu_mesh() {
        //        gpu_mesh.free(device);
        //    }
        //}
    }


    pub fn alloc(&mut self) -> RisResult<MeshLookupId> {
        // allocate into the mesh that is either
        //  1. not allocated or
        //  2. is
        //      a. unused and
        //      b. lowest counter id

        let unallocated_entry_index = self.entries.iter().position(|x| x.value.is_none());

        let to_allocate_index = if let Some(unallocated_entry_index) = unallocated_entry_index {
            unallocated_entry_index
        } else {

            let mut candidates = self.entries
                .iter()
                .enumerate()
                .filter(|&(i, x)| x.lookup_id.is_unique())
                .collect::<Vec<_>>();

            let max = candidates.first().into_ris_error()?;

            for (i, entry) in candidates {

            }

            unused_entry_index
        };


        let to_allocate = &mut self.entries[to_allocate_index];
        panic!("allocate");
    }

    pub fn get(&mut self) -> Option<&(CpuMesh, GpuMesh)> {
        // get the mesh, which is
        //  1. allocated and
        //  2. highest counter id
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
