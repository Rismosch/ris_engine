use std::ptr;

use ash::vk;

use ris_error::RisResult;

pub struct FrameInFlight {
    pub image_available: vk::Semaphore,
    pub render_finished: vk::Semaphore,
    pub in_flight: vk::Fence,
}

impl FrameInFlight {
    pub fn alloc(device: &ash::Device) -> RisResult<Self> {
        let semaphore_create_info = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::SemaphoreCreateFlags::empty(),
        };

        let fence_create_info = vk::FenceCreateInfo {
            s_type: vk::StructureType::FENCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::FenceCreateFlags::SIGNALED,
        };

        let image_available = unsafe{device.create_semaphore(&semaphore_create_info, None)}?;
        let render_finished = unsafe{device.create_semaphore(&semaphore_create_info, None)}?;
        let in_flight = unsafe{device.create_fence(&fence_create_info, None)}?;

        Ok(Self{
            image_available,
            render_finished,
            in_flight,
        })
    }

    pub fn free(&self, device: &ash::Device) {
        unsafe {
            device.destroy_fence(self.in_flight, None);
            device.destroy_semaphore(self.render_finished, None);
            device.destroy_semaphore(self.image_available, None);
        }
    }
}
