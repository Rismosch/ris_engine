use std::ffi::CStr;

use ash::vk;

use ris_error::RisResult;

pub const ENTRY: &CStr = c"main";

pub fn create_module(device: &ash::Device, bytes: &[u8]) -> RisResult<vk::ShaderModule> {
    ris_error::assert!(bytes.len().is_multiple_of(4))?;

    let shader_module_create_info = vk::ShaderModuleCreateInfo {
        s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: vk::ShaderModuleCreateFlags::empty(),
        code_size: bytes.len(),
        p_code: bytes.as_ptr() as *const u32,
    };

    let shader_module = unsafe { device.create_shader_module(&shader_module_create_info, None) }?;

    Ok(shader_module)
}
