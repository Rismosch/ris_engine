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
        let mut q = [0.; 4];

        let nxt = [1, 2, 0];
        let tr = m.m00 + m.m11 + m.m22;

        // check the diagonal
        if tr > 0. {
            let s = super::sqrt(tr + 1.);
            q[3] = s / 2.;
            let s = 0.5 / s;
            q[0] = (m.m21 - m.m12) * s;
            q[1] = (m.m02 - m.m20) * s;
            q[2] = (m.m10 - m.m01) * s;
        } else {
            // diagonal is negative
            let mut i = 0;
            if m.m11 > m.m00 {
                i = 1;
            }
            if m.m22 > m.get(i, i) {
                i = 2;
            }

            let j = nxt[i];
            let k = nxt[j];

            let mut s = super::sqrt((m.get(i, i) - m.get(j, j) - m.get(k, k)) + 1.);

            q[i] = s * 0.5;

            if s != 0.0 {
                s = 0.5 / s;
            }

            q[3] = (m.get(k, j) - m.get(j, k)) * s;
            q[j] = (m.get(i, j) + m.get(j, i)) * s;
            q[k] = (m.get(i, k) + m.get(k, i)) * s;
        }

        Self {
            x: q[0],
            y: q[1],
            z: q[2],
            w: q[3],
        }
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
        let magnitude = self.magnitude();
        Self {
            w: self.w / magnitude,
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    pub fn magnitude_squared(&self) -> f32 {
        Self::dot(*self, *self)
    }

    pub fn magnitude(&self) -> f32 {
        super::sqrt(self.magnitude_squared())
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::identity()
    }
}
