use vulkano::format::Format;
use vulkano::pipeline::graphics::vertex_input::VertexInputAttributeDescription;
use vulkano::pipeline::graphics::vertex_input::VertexInputBindingDescription;
use vulkano::pipeline::graphics::vertex_input::VertexInputRate;
use vulkano::pipeline::graphics::vertex_input::VertexInputState;

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct ImguiVertex {
    pub pos: [f32; 2],
    pub uv : [f32; 2],
    pub col: u32, // [u8; 4]
}

impl ImguiVertex {
    pub fn input_state() -> VertexInputState {
        let bindings = [(
            0u32,
            VertexInputBindingDescription {
                stride: 20,
                input_rate: VertexInputRate::Vertex,
            },
        )];

        let attributes = [
            (
                0,
                VertexInputAttributeDescription {
                    binding: 0,
                    format: Format::R32G32_SFLOAT,
                    offset: 0,
                },
            ),
            (
                1,
                VertexInputAttributeDescription {
                    binding: 0,
                    format: Format::R32G32_SFLOAT,
                    offset: 8,
                },
            ),
            (
                2,
                VertexInputAttributeDescription {
                    binding: 0,
                    format: Format::R32_UINT,
                    offset: 16,
                },
            ),
        ];

        VertexInputState::new()
            .bindings(bindings)
            .attributes(attributes)
    }
}
