pub mod buffer;
pub mod core;
pub mod frames_in_flight;
pub mod image;
pub mod io;
pub mod layers;
pub mod shader;
pub mod suitable_device;
pub mod surface_details;
pub mod swapchain;
pub mod texture;
pub mod transient_command;
pub mod util;

use ash::vk;

const PREFERRED_FORMAT: vk::Format = vk::Format::B8G8R8A8_SRGB;
const PREFERRED_COLOR_SPACE: vk::ColorSpaceKHR = vk::ColorSpaceKHR::SRGB_NONLINEAR;
const PREFERRED_PRESENT_MODE: vk::PresentModeKHR = vk::PresentModeKHR::FIFO;
