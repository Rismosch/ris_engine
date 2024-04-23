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

        let result = cstr
            .to_str()?
            .to_owned();

        Ok(Self{
            value: result,
        })
    }

    pub unsafe fn as_ptr(&self) -> *const i8{
        self.value.as_ptr() as *const i8
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

pub fn find_memory_type(
    type_filter: u32,
    memory_property_flags: vk::MemoryPropertyFlags,
    physical_device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
) -> RisResult<Option<u32>> {
    for (i, potential_memory_type) in physical_device_memory_properties.memory_types.iter().enumerate() {
        if (type_filter & (1 << i)) > 0 &&
            potential_memory_type.property_flags.contains(memory_property_flags) {
            return Ok(Some(i as u32));
        }
    }

    Ok(None)
}
