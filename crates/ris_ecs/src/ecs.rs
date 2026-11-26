use crate::transform::Transform;


#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Object(pub usize);

#[derive(Debug)]
pub struct Ecs {
    pub transform_components: Vec<Transform>,
}

impl Ecs {
    pub fn new(capacity: usize) -> Self {
        let mut transform_components = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            transform_components.push(Default::default());
        }

        Self {
            transform_components,
        }
    }

    pub fn len(&self) -> usize {
        self.transform_components.len()
    }
}
