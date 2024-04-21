pub mod buffer;
pub mod frame_in_flight;
pub mod renderer;
pub mod suitable_device;
pub mod surface_details;
pub mod swapchain_objects;
pub mod transient_command;
pub mod uniform_buffer_object;
pub mod util;
pub mod vertex;

use ash::vk;

use ris_math::color::Rgb;
use ris_math::vector::Vec3;
use ris_math::vector::Vec2;

use vertex::Vertex;

const REQUIRED_INSTANCE_LAYERS: &[&str] = &["VK_LAYER_KHRONOS_validation"];
const REQUIRED_DEVICE_EXTENSIONS: &[*const i8] = &[ash::extensions::khr::Swapchain::name().as_ptr()];
#[cfg(not(debug_assertions))]
const VALIDATION_ENABLED: bool = false;
#[cfg(debug_assertions)]
const VALIDATION_ENABLED: bool = true;

const PREFERRED_FORMAT: vk::Format = vk::Format::B8G8R8A8_SRGB;
const PREFERRED_COLOR_SPACE: vk::ColorSpaceKHR = vk::ColorSpaceKHR::SRGB_NONLINEAR;
const PREFERRED_PRESENT_MODE: vk::PresentModeKHR = vk::PresentModeKHR::IMMEDIATE;

const MAX_FRAMES_IN_FLIGHT: u32 = 2;

pub const VERTICES: [Vertex; 4 * 6] = [
    // pos x
    Vertex{
        pos: Vec3(0.5, -0.5, -0.5),
        color: Rgb{r:1.0, g:0.0, b:0.0},
        uv: Vec2(0.0, 0.0),
    },
    Vertex{
        pos: Vec3(0.5, -0.5, 0.5),
        color: Rgb{r:1.0, g:0.0, b:0.0},
        uv: Vec2(0.0, 1.0),
    },
    Vertex{
        pos: Vec3(0.5, 0.5, 0.5),
        color: Rgb{r:1.0, g:0.0, b:0.0},
        uv: Vec2(1.0, 1.0),
    },
    Vertex{
        pos: Vec3(0.5, 0.5, -0.5),
        color: Rgb{r:1.0, g:0.0, b:0.0},
        uv: Vec2(1.0, 0.0),
    },
    // pos y
    Vertex{
        pos: Vec3(0.5, 0.5, -0.5),
        color: Rgb{r:0.0, g:1.0, b:0.0},
        uv: Vec2(0.0, 0.0),
    },
    Vertex{
        pos: Vec3(0.5, 0.5, 0.5),
        color: Rgb{r:0.0, g:1.0, b:0.0},
        uv: Vec2(0.0, 1.0),
    },
    Vertex{
        pos: Vec3(-0.5, 0.5, 0.5),
        color: Rgb{r:0.0, g:1.0, b:0.0},
        uv: Vec2(1.0, 1.0),
    },
    Vertex{
        pos: Vec3(-0.5, 0.5, -0.5),
        color: Rgb{r:0.0, g:1.0, b:0.0},
        uv: Vec2(1.0, 0.0),
    },
    // pos z
    Vertex{
        pos: Vec3(-0.5, -0.5, 0.5),
        color: Rgb{r:0.0, g:0.0, b:1.0},
        uv: Vec2(0.0, 0.0),
    },
    Vertex{
        pos: Vec3(-0.5, 0.5, 0.5),
        color: Rgb{r:0.0, g:0.0, b:1.0},
        uv: Vec2(0.0, 1.0),
    },
    Vertex{
        pos: Vec3(0.5, 0.5, 0.5),
        color: Rgb{r:0.0, g:0.0, b:1.0},
        uv: Vec2(1.0, 1.0),
    },
    Vertex{
        pos: Vec3(0.5, -0.5, 0.5),
        color: Rgb{r:0.0, g:0.0, b:1.0},
        uv: Vec2(1.0, 0.0),
    },
    // neg x
    Vertex{
        pos: Vec3(-0.5, 0.5, -0.5),
        color: Rgb{r:0.0, g:1.0, b:1.0},
        uv: Vec2(0.0, 0.0),
    },
    Vertex{
        pos: Vec3(-0.5, 0.5, 0.5),
        color: Rgb{r:0.0, g:1.0, b:1.0},
        uv: Vec2(0.0, 1.0),
    },
    Vertex{
        pos: Vec3(-0.5, -0.5, 0.5),
        color: Rgb{r:0.0, g:1.0, b:1.0},
        uv: Vec2(1.0, 1.0),
    },
    Vertex{
        pos: Vec3(-0.5, -0.5, -0.5),
        color: Rgb{r:0.0, g:1.0, b:1.0},
        uv: Vec2(1.0, 0.0),
    },
    // neg y
    Vertex{
        pos: Vec3(-0.5, -0.5, -0.5),
        color: Rgb{r:1.0, g:0.0, b:1.0},
        uv: Vec2(0.0, 0.0),
    },
    Vertex{
        pos: Vec3(-0.5, -0.5, 0.5),
        color: Rgb{r:1.0, g:0.0, b:1.0},
        uv: Vec2(0.0, 1.0),
    },
    Vertex{
        pos: Vec3(0.5, -0.5, 0.5),
        color: Rgb{r:1.0, g:0.0, b:1.0},
        uv: Vec2(1.0, 1.0),
    },
    Vertex{
        pos: Vec3(0.5, -0.5, -0.5),
        color: Rgb{r:1.0, g:0.0, b:1.0},
        uv: Vec2(1.0, 0.0),
    },
    // neg z
    Vertex{
        pos: Vec3(-0.5, 0.5, -0.5),
        color: Rgb{r:1.0, g:1.0, b:0.0},
        uv: Vec2(0.0, 0.0),
    },
    Vertex{
        pos: Vec3(-0.5, -0.5, -0.5),
        color: Rgb{r:1.0, g:1.0, b:0.0},
        uv: Vec2(0.0, 1.0),
    },
    Vertex{
        pos: Vec3(0.5, -0.5, -0.5),
        color: Rgb{r:1.0, g:1.0, b:0.0},
        uv: Vec2(1.0, 1.0),
    },
    Vertex{
        pos: Vec3(0.5, 0.5, -0.5),
        color: Rgb{r:1.0, g:1.0, b:0.0},
        uv: Vec2(1.0, 0.0),
    },
];

pub const INDICES: [u32; 6 * 6] = [
    0, 1, 2, 2, 3, 0,
    4, 5, 6, 6, 7, 4,
    8, 9, 10, 10, 11, 8,
    12, 13, 14, 14, 15, 12,
    16, 17, 18, 18, 19, 16,
    20, 21, 22, 22, 23, 20,
];
