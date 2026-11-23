use crate::transform::Transform;


#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Object(pub usize);

pub struct Ecs {
    pub transform_components: Vec<Transform>,
}

impl Ecs {
    pub fn new(capacity: usize) -> Self {
        Self {
            transform_components: Vec::with_capacity(capacity),
        }
    }
}
