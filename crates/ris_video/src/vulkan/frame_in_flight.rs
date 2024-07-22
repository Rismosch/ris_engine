use std::ptr;

use ash::vk;

use ris_error::RisResult;

pub struct FrameInFlight {
    pub image_available: vk::Semaphore,
    pub main_render: vk::Semaphore,
    pub gizmo_shape_render: vk::Semaphore,
    pub gizmo_text_render: vk::Semaphore,
    pub ui_helper_render: vk::Semaphore,
    pub in_flight: vk::Fence,
}

impl FrameInFlight {
    /// # Safety
    ///
    /// `free()` must be called, or you are leaking memory.
    pub unsafe fn alloc(device: &ash::Device) -> RisResult<Self> {
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

        let image_available = unsafe { device.create_semaphore(&semaphore_create_info, None) }?;
        let main_render = unsafe { device.create_semaphore(&semaphore_create_info, None) }?;
        let gizmo_shape_render = unsafe { device.create_semaphore(&semaphore_create_info, None) }?;
        let gizmo_text_render = unsafe { device.create_semaphore(&semaphore_create_info, None) }?;
        let ui_helper_render = unsafe { device.create_semaphore(&semaphore_create_info, None) }?;
        let in_flight = unsafe { device.create_fence(&fence_create_info, None) }?;

        Ok(Self {
            image_available,
            main_render,
            gizmo_shape_render,
            gizmo_text_render,
            ui_helper_render,
            in_flight,
        })
    }

    /// # Safety
    ///
    /// Must only be called once. Memory must not be freed twice.
    pub unsafe fn free(&self, device: &ash::Device) {
        unsafe {
            device.destroy_fence(self.in_flight, None);
            device.destroy_semaphore(self.main_render, None);
            device.destroy_semaphore(self.gizmo_shape_render, None);
            device.destroy_semaphore(self.gizmo_text_render, None);
            device.destroy_semaphore(self.ui_helper_render, None);
            device.destroy_semaphore(self.image_available, None);
        }
    }
}
