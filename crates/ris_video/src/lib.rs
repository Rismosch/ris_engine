pub mod allocators;
pub mod buffers;
pub mod command_buffers;
pub mod gpu_objects;
pub mod imgui;
pub mod physical_device;
pub mod pipeline;
pub mod render_pass;
pub mod renderer;
pub mod shaders;
pub mod swapchain;
pub mod video;

use vulkano::format::Format;

pub const DEPTH_FORMAT: Format = Format::D24_UNORM_S8_UINT;
