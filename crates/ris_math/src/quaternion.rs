use crate::matrix4x4::Matrix4x4;

#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Quaternion {
    // initialization
    pub fn identity() -> Self {
        Self {
            w: 1.,
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn from_matrix(m: Matrix4x4) -> Self {

    }

    // utility
    pub fn multiply(p: Quaternion, q: Quaternion) -> Self {
        Self {
            w: p.w * q.w - p.x * q.x - p.y * q.y - p.z * q.z,
            x: p.x * q.w + p.w * q.x + p.y * q.z + p.z * q.y,
            y: p.y * q.w + p.w * q.y + p.z * q.x + p.x * q.z,
            z: p.z * q.w + p.w * q.z + p.x * q.y + p.y * q.x,
        }
    }

    pub fn dot(p: Quaternion, q: Quaternion) -> f32 {
        p.w * q.w + p.x * q.x + p.y * q.y + p.z * q.z
    }

    pub fn conjugate(&self) -> Self {
        Self {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn normalized(&self) -> Self {
        let magnitude = super::sqrt(Self::dot(*self, *self));
        Self {
            w: self.w / magnitude,
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::identity()
    }
}
