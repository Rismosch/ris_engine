use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;

#[derive(BufferContents, Default)]
#[repr(C)]
pub struct UniformBufferObject {
    pub debug_x: i32,
    pub debug_y: i32,
}

#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct RisVertex {
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],
}
