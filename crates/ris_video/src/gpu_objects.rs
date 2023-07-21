use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;

#[derive(BufferContents, Default)]
#[repr(C)]
pub struct UniformBufferObject {
    pub debug_x: i32,
    pub debug_y: i32,
}

#[derive(BufferContents, Vertex, Default)]
#[repr(C)]
pub struct Vertex2d {
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],
}
