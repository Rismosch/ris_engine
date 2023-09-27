use vulkano::buffer::BufferContents;
use vulkano::format::Format;
use vulkano::pipeline::graphics::vertex_input::VertexInputAttributeDescription;
use vulkano::pipeline::graphics::vertex_input::VertexInputBindingDescription;
use vulkano::pipeline::graphics::vertex_input::VertexInputRate;
use vulkano::pipeline::graphics::vertex_input::VertexInputState;

use ris_math::matrix4x4::Matrix4x4;

#[derive(BufferContents, Default)]
#[repr(C)]
pub struct UniformBufferObject {
    pub view: Matrix4x4,
    pub proj: Matrix4x4,
    pub view_proj: Matrix4x4,
}

#[derive(BufferContents, Default)]
#[repr(C)]
pub struct Vertex3d {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex3d {
    pub fn input_state() -> VertexInputState {
        let bindings = [(
            0u32,
            VertexInputBindingDescription {
                stride: 24,
                input_rate: VertexInputRate::Vertex,
            },
        )];

        let attributes = [
            (
                0,
                VertexInputAttributeDescription {
                    binding: 0,
                    format: Format::R32G32B32_SFLOAT,
                    offset: 0,
                },
            ),
            (
                1,
                VertexInputAttributeDescription {
                    binding: 0,
                    format: Format::R32G32B32_SFLOAT,
                    offset: 12,
                },
            ),
        ];

        VertexInputState::new()
            .bindings(bindings)
            .attributes(attributes)
    }
}
