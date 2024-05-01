use std::ffi::CStr;
use std::os::raw::c_void;

use ash::vk;

use ris_error::RisResult;

pub struct VkStr {
    value: String,
}

impl std::fmt::Display for VkStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl VkStr {
    /// # Safety
    ///
    /// See `std::ffi::Cstr::from_ptr()`
    pub unsafe fn from(value: &[i8]) -> RisResult<Self> {
        let cstr = unsafe {
            let ptr = value.as_ptr();
            CStr::from_ptr(ptr)
        };

        let result = cstr.to_str()?.to_owned();

        Ok(Self { value: result })
    }

    pub unsafe fn as_ptr(&self) -> *const i8 {
        self.value.as_ptr() as *const i8
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

pub fn find_memory_type(
    type_filter: u32,
    memory_property_flags: vk::MemoryPropertyFlags,
    physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
) -> RisResult<Option<u32>> {
    for (i, potential_memory_type) in physical_device_memory_properties
        .memory_types
        .iter()
        .enumerate()
    {
        if (type_filter & (1 << i)) > 0
            && potential_memory_type
                .property_flags
                .contains(memory_property_flags)
        {
            return Ok(Some(i as u32));
        }
    }

    Ok(None)
}

pub fn find_depth_format(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> RisResult<vk::Format> {
    find_supported_format(
        instance,
        physical_device,
        &[
            vk::Format::D32_SFLOAT,
            vk::Format::D32_SFLOAT_S8_UINT,
            vk::Format::D24_UNORM_S8_UINT,
        ],
        vk::ImageTiling::OPTIMAL,
        vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
    )
}

pub fn find_supported_format(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    candidates: &[vk::Format],
    tiling: vk::ImageTiling,
    features: vk::FormatFeatureFlags,
) -> RisResult<vk::Format> {
    for &candidate in candidates.iter() {
        let format_properties =
            unsafe { instance.get_physical_device_format_properties(physical_device, candidate) };

        if tiling == vk::ImageTiling::LINEAR
            && format_properties.linear_tiling_features.contains(features)
        {
            return Ok(candidate.clone());
        }

        if tiling == vk::ImageTiling::OPTIMAL
            && format_properties.optimal_tiling_features.contains(features)
        {
            return Ok(candidate.clone());
        }
    }

    ris_error::new_result!("failed to find supported format")
}

pub fn has_stencil_component(format: vk::Format) -> bool {
    format == vk::Format::D32_SFLOAT_S8_UINT || format == vk::Format::D24_UNORM_S8_UINT
}
