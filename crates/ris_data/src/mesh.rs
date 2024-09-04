use ris_math::vector::Vec2;
use ris_math::vector::Vec3;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Vertex {
    pub pos: Vec3,
    pub uv: Vec2,
}

#[derive(Debug, Default)]
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
        ris_util::vec::fast_copy(&mut self.vertices, value);
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    pub fn set_indices(&mut self, value: &[u32]) {
        self.is_dirty = true;
        ris_util::vec::fast_copy(&mut self.indices, value);
    }
}

//
// primitives
//

impl Mesh {
    pub fn primitive_cube() -> Self {
        Self {
            is_dirty: false,
            vertices: vec![
                // pos x
                Vertex {
                    pos: Vec3(0.5, -0.5, 0.5),
                    uv: Vec2(0.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, 0.5, 0.5),
                    uv: Vec2(1.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, 0.5, -0.5),
                    uv: Vec2(1.0, 1.0),
                },
                Vertex {
                    pos: Vec3(0.5, -0.5, -0.5),
                    uv: Vec2(0.0, 1.0),
                },
                // pos y
                Vertex {
                    pos: Vec3(0.5, 0.5, 0.5),
                    uv: Vec2(0.0, 0.0),
                },
                Vertex {
                    pos: Vec3(-0.5, 0.5, 0.5),
                    uv: Vec2(1.0, 0.0),
                },
                Vertex {
                    pos: Vec3(-0.5, 0.5, -0.5),
                    uv: Vec2(1.0, 1.0),
                },
                Vertex {
                    pos: Vec3(0.5, 0.5, -0.5),
                    uv: Vec2(0.0, 1.0),
                },
                // pos z
                Vertex {
                    pos: Vec3(-0.5, 0.5, 0.5),
                    uv: Vec2(0.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, 0.5, 0.5),
                    uv: Vec2(1.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, -0.5, 0.5),
                    uv: Vec2(1.0, 1.0),
                },
                Vertex {
                    pos: Vec3(-0.5, -0.5, 0.5),
                    uv: Vec2(0.0, 1.0),
                },
                // neg x
                Vertex {
                    pos: Vec3(-0.5, 0.5, 0.5),
                    uv: Vec2(0.0, 0.0),
                },
                Vertex {
                    pos: Vec3(-0.5, -0.5, 0.5),
                    uv: Vec2(1.0, 0.0),
                },
                Vertex {
                    pos: Vec3(-0.5, -0.5, -0.5),
                    uv: Vec2(1.0, 1.0),
                },
                Vertex {
                    pos: Vec3(-0.5, 0.5, -0.5),
                    uv: Vec2(0.0, 1.0),
                },
                // neg y
                Vertex {
                    pos: Vec3(-0.5, -0.5, 0.5),
                    uv: Vec2(0.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, -0.5, 0.5),
                    uv: Vec2(1.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, -0.5, -0.5),
                    uv: Vec2(1.0, 1.0),
                },
                Vertex {
                    pos: Vec3(-0.5, -0.5, -0.5),
                    uv: Vec2(0.0, 1.0),
                },
                // neg z
                Vertex {
                    pos: Vec3(-0.5, -0.5, -0.5),
                    uv: Vec2(0.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, -0.5, -0.5),
                    uv: Vec2(1.0, 0.0),
                },
                Vertex {
                    pos: Vec3(0.5, 0.5, -0.5),
                    uv: Vec2(1.0, 1.0),
                },
                Vertex {
                    pos: Vec3(-0.5, 0.5, -0.5),
                    uv: Vec2(0.0, 1.0),
                },
            ],
            indices: vec![
                0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12, 16, 17, 18, 18, 19, 16, 20, 21, 22, 22, 23, 20,
            ],
        }
    }
}
