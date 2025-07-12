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
    // counter_id determines how young this mesh is. the bigger the counter, the newer the mesh
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

    pub fn alloc(&mut self) -> RisResult<bool> {

        let mut empty = None;
        let mut min = None;
        let mut max = None;

        for (i, entry) in self.entries.iter_mut().enumerate() {
            if !entry.lookup_id.is_unique() {
                continue;
            }

            match entry.value.as_ref() {
                Some(EntryState::Loading(_)) => continue,
                None => empty = Some(i),
                _ => (),
            }

            min = match min.take() {
                Some((min_index, min_counter_id)) => {
                    if entry.counter_id < min_counter_id {
                        Some((i, entry.counter_id))
                    } else {
                        Some((min_index, min_counter_id))
                    }
                },
                None => {
                    Some((i, entry.counter_id))
                },
            };

            max = match max.take() {
                Some(max_counter_id) => {
                    if entry.counter_id > max_counter_id {
                        Some(entry.counter_id)
                    } else {
                        Some(max_counter_id)
                    }
                },
                None => {
                    Some(entry.counter_id)
                },
            };
        }

        let (index, mut counter_id) = match (empty, min, max) {
            (Some(index), _, Some(counter_id)) => (index, counter_id),
            (_, Some((index, _)), Some(counter_id)) => (index, counter_id),
            _ => return Ok(false), // no free entry. cannot allocate
        };

        counter_id.increase();

        let to_allocate = &mut self.entries[index];
        to_allocate.counter_id = counter_id;

        panic!(create mesh)


        Ok(true)
    }

    pub fn get_newest_id(&mut self) -> Option<MeshLookupId> {

        let mut max = None;

        for (i, entry) in self.entries.iter_mut().enumerate() {
            match entry.value.take() {
                Some(EntryState::Loading(receiver)) => match receiver.receive() {
                    Ok(Ok((cpu_mesh, gpu_mesh))) => entry.value = Some(EntryState::Loaded((cpu_mesh, gpu_mesh))),
                    Ok(Err(e)) => {
                        ris_log::error!("failed to load terrain: {}", e)
                    },
                    Err(receiver) => entry.value = Some(EntryState::Loading(receiver)),
                },
                value => entry.value = value,
            }

            max = match max.take() {
                Some((max_index, max_counter_id)) => {
                    if entry.counter_id > max_counter_id {
                        Some((i, entry.counter_id))
                    } else {
                        Some((max_index, max_counter_id))
                    }
                },
                None => Some((i, entry.counter_id)),
            }
        }

        match max {
            Some((newest_index, _)) => {
                let newest_entry = self.entries.get(newest_index)?;
                let newset_id = newest_entry.lookup_id.clone();
                Some(newset_id)
            },
            None => None,
        }
    }

    pub unsafe fn get(&mut self, id: &MeshLookupId) -> Option<(&CpuMesh, &GpuMesh)> {
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
