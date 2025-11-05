use std::any::TypeId;
use std::ffi::CStr;
use std::ffi::CString;

use ash::vk;

use ris_error::RisResult;

pub fn vk_to_c_str(value: &[i8]) -> &CStr {
    unsafe { CStr::from_ptr(value.as_ptr()) }
}

pub fn vk_to_std_str(value: &[i8]) -> RisResult<&str> {
    let cstr = vk_to_c_str(value);
    let stdstr = cstr.to_str()?;
    Ok(stdstr)
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
            return Ok(candidate);
        }

        if tiling == vk::ImageTiling::OPTIMAL
            && format_properties.optimal_tiling_features.contains(features)
        {
            return Ok(candidate);
        }
    }

    ris_error::new_result!("formats are not supported")
}

pub fn has_stencil_component(format: vk::Format) -> bool {
    format == vk::Format::D32_SFLOAT_S8_UINT || format == vk::Format::D24_UNORM_S8_UINT
}

pub fn to_vk_fat_ptr<T>(value: impl AsRef<[T]>) -> (u32, *const T) {
    let value = value.as_ref();
    if value.is_empty() {
        (0, std::ptr::null())
    } else {
        (value.len() as u32, value.as_ptr())
    }
}

