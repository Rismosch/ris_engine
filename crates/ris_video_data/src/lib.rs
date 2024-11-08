pub mod buffer;
pub mod core;
pub mod frame_in_flight;
pub mod image;
pub mod layers;
pub mod shader;
pub mod suitable_device;
pub mod surface_details;
pub mod swapchain;
pub mod texture;
pub mod transient_command;
pub mod util;

use ash::vk;

const REQUIRED_INSTANCE_LAYERS: &[&str] = &["VK_LAYER_KHRONOS_validation"];
const REQUIRED_DEVICE_EXTENSIONS: &[*const i8] =
    &[ash::extensions::khr::Swapchain::name().as_ptr()];
#[cfg(not(debug_assertions))]
const VALIDATION_ENABLED: bool = false;
#[cfg(debug_assertions)]
const VALIDATION_ENABLED: bool = true;

const PREFERRED_FORMAT: vk::Format = vk::Format::B8G8R8A8_SRGB;
const PREFERRED_COLOR_SPACE: vk::ColorSpaceKHR = vk::ColorSpaceKHR::SRGB_NONLINEAR;
const PREFERRED_PRESENT_MODE: vk::PresentModeKHR = vk::PresentModeKHR::IMMEDIATE;

const MAX_FRAMES_IN_FLIGHT: usize = 2;
