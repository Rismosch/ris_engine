use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;

use ris_math::matrix4x4::Matrix4x4;

#[derive(BufferContents, Default)]
#[repr(C)]
pub struct UniformBufferObject {
    pub view_matrix: Matrix4x4,
    pub debug_x: i32,
    pub debug_y: i32,
}

#[derive(BufferContents, Vertex, Default)]
#[repr(C)]
pub struct Vertex2d {
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],
    #[format(R32G32B32_SFLOAT)]
    pub color: [f32; 3],
}
