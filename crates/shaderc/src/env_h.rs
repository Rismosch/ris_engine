#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/* automatically generated by rust-bindgen 0.71.1 */

pub const _VCRT_COMPILER_PREPROCESSOR: u32 = 1;
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const _STL_LANG: u32 = 0;
pub const _HAS_CXX17: u32 = 0;
pub const _HAS_CXX20: u32 = 0;
pub const _HAS_CXX23: u32 = 0;
pub const _HAS_NODISCARD: u32 = 0;
pub const WCHAR_MIN: u32 = 0;
pub const WCHAR_MAX: u32 = 65535;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 65535;
pub type va_list = *mut ::std::os::raw::c_char;
extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
}
pub type __vcrt_bool = bool;
pub type wchar_t = ::std::os::raw::c_ushort;
extern "C" {
    pub fn __security_init_cookie();
}
extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize) -> !;
}
extern "C" {
    pub static mut __security_cookie: usize;
}
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_longlong;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulonglong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_int;
pub type int_fast32_t = ::std::os::raw::c_int;
pub type int_fast64_t = ::std::os::raw::c_longlong;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_uint;
pub type uint_fast32_t = ::std::os::raw::c_uint;
pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
pub const shaderc_target_env_shaderc_target_env_vulkan: shaderc_target_env = 0;
pub const shaderc_target_env_shaderc_target_env_opengl: shaderc_target_env = 1;
pub const shaderc_target_env_shaderc_target_env_opengl_compat: shaderc_target_env = 2;
pub const shaderc_target_env_shaderc_target_env_webgpu: shaderc_target_env = 3;
pub const shaderc_target_env_shaderc_target_env_default: shaderc_target_env = 0;
pub type shaderc_target_env = ::std::os::raw::c_int;
pub const shaderc_env_version_shaderc_env_version_vulkan_1_0: shaderc_env_version = 4194304;
pub const shaderc_env_version_shaderc_env_version_vulkan_1_1: shaderc_env_version = 4198400;
pub const shaderc_env_version_shaderc_env_version_vulkan_1_2: shaderc_env_version = 4202496;
pub const shaderc_env_version_shaderc_env_version_vulkan_1_3: shaderc_env_version = 4206592;
pub const shaderc_env_version_shaderc_env_version_vulkan_1_4: shaderc_env_version = 4210688;
pub const shaderc_env_version_shaderc_env_version_opengl_4_5: shaderc_env_version = 450;
pub const shaderc_env_version_shaderc_env_version_webgpu: shaderc_env_version = 451;
pub type shaderc_env_version = ::std::os::raw::c_int;
pub const shaderc_spirv_version_shaderc_spirv_version_1_0: shaderc_spirv_version = 65536;
pub const shaderc_spirv_version_shaderc_spirv_version_1_1: shaderc_spirv_version = 65792;
pub const shaderc_spirv_version_shaderc_spirv_version_1_2: shaderc_spirv_version = 66048;
pub const shaderc_spirv_version_shaderc_spirv_version_1_3: shaderc_spirv_version = 66304;
pub const shaderc_spirv_version_shaderc_spirv_version_1_4: shaderc_spirv_version = 66560;
pub const shaderc_spirv_version_shaderc_spirv_version_1_5: shaderc_spirv_version = 66816;
pub const shaderc_spirv_version_shaderc_spirv_version_1_6: shaderc_spirv_version = 67072;
pub type shaderc_spirv_version = ::std::os::raw::c_int;
