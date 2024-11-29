use ash::vk;

use ris_error::RisResult;
use ris_math::color::Rgb;
use ris_math::vector::Vec2;
use ris_math::vector::Vec3;
use ris_video_data::buffer::Buffer;

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Vertex {
    pub pos: Vec3,
    pub color: Rgb,
    pub uv: Vec2,
}

impl Vertex {
    pub fn binding_descriptions() -> [vk::VertexInputBindingDescription; 1] {
        [vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Vertex>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }]
    }

    pub fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 3] {
        [
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: std::mem::offset_of!(Vertex, pos) as u32,
            },
            vk::VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: std::mem::offset_of!(Vertex, color) as u32,
            },
            vk::VertexInputAttributeDescription {
                location: 2,
                binding: 0,
                format: vk::Format::R32G32_SFLOAT,
                offset: std::mem::offset_of!(Vertex, uv) as u32,
            },
        ]
    }
}
