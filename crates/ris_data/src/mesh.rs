use ris_math::vector::Vec3;
use ris_math::vector::Vec2;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Vertex {
    pub pos: Vec3,
    pub uv: Vec2,
}

#[derive(Default)]
pub struct Mesh {
    pub is_dirty: bool,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Mesh {
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn set_vertices(&mut self, value: &[Vertex]) {
        self.is_dirty = true;
        self.vertices.clear();
        self.vertices.extend_from_slice(value);
        self.vertices.copy_from_slice(value)
    }
}
