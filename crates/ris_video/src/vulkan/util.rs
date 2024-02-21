use std::ffi::CStr;

use ris_error::RisResult;

pub fn vk_to_string(vk_string_array: &[i8]) -> RisResult<String> {
    let cstr = unsafe {
        let ptr = vk_string_array.as_ptr();
        CStr::from_ptr(ptr)
    };

    let result = cstr
        .to_str()?
        .to_owned();

    Ok(result)
}
