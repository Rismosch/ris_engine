use ash::vk;

use imgui::DrawData;

use ris_error::RisResult;

use crate::imgui::imgui_mesh::Mesh;

pub struct Frames {
    pub index: usize,
    pub count: usize,
    pub meshes: Vec<Mesh>,
}

impl Frames {
    pub unsafe fn alloc(
        device: &ash::Device,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
        draw_data: &DrawData,
        count: usize,
    ) -> RisResult<Self> {
        let meshes = (0..count)
            .map(|_| Mesh::alloc(device, physical_device_memory_properties, draw_data))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            index: 0,
            count,
            meshes,
        })
    }

    pub unsafe fn free(&self, device: &ash::Device) {
        for mesh in self.meshes.iter() {
            mesh.free(device);
        }
    }

    pub fn acquire_next_mesh(&mut self) -> &mut Mesh {
        let result = &mut self.meshes[self.index];
        self.index = (self.index + 1) % self.count;
        result
    }
}
