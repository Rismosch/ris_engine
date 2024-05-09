use ash::vk;

use ris_error::Extensions;
use ris_error::RisResult;

pub struct SurfaceDetails {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

impl SurfaceDetails {
    pub fn query(
        surface_loader: &ash::extensions::khr::Surface,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> RisResult<Self> {
        let capabilities = unsafe {
            surface_loader.get_physical_device_surface_capabilities(physical_device, surface)
        }?;
        let formats = unsafe {
            surface_loader.get_physical_device_surface_formats(physical_device, surface)
        }?;
        let present_modes = unsafe {
            surface_loader.get_physical_device_surface_present_modes(physical_device, surface)
        }?;

        Ok(Self {
            capabilities,
            formats,
            present_modes,
        })
    }
}
