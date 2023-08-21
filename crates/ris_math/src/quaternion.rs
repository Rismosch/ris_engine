use crate::vector3::Vector3;

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

    pub fn from_angle_axis(angle: f32, axis: Vector3) -> Self {
        let n = axis.normalized();
        let t = angle * 0.5;
        let re = super::cos(t);
        let im = super::sin(t);

        Self {
            w: re,
            x: n.x * im,
            y: n.y * im,
            z: n.z * im,
        }
    }

    pub fn to_angle_axis(self) -> (f32, Vector3) {
        let mut q = self;

        // if w>1 acos and sqrt will produce errors, this cant happen if quaternion is normalized
        if super::abs(q.w) > 1. {
            q = q.normalized();
        }

        let t = 2. * super::acos(q.w);
        let s = super::sqrt(1. - q.w * q.w);

        let n = if s < 0.001 {
            Vector3 {
                x: 1.,
                y: 0.,
                z: 0.,
            }
        } else {
            Vector3 {
                x: q.x / s,
                y: q.y / s,
                z: q.z / s,
            }
        };

        (t, n)
    }

    // utility
    pub fn dot(p: Quaternion, q: Quaternion) -> f32 {
        p.w * q.w + p.x * q.x + p.y * q.y + p.z * q.z
    }

    pub fn conjugate(self) -> Self {
        Self {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn normalized(self) -> Self {
        let magnitude = self.magnitude();
        if magnitude < super::MIN_NORM {
            Self::identity()
        } else {
            Self {
                w: self.w / magnitude,
                x: self.x / magnitude,
                y: self.y / magnitude,
                z: self.z / magnitude,
            }
        }
    }

    pub fn magnitude_squared(self) -> f32 {
        Self::dot(self, self)
    }

    pub fn magnitude(self) -> f32 {
        super::sqrt(self.magnitude_squared())
    }

    // 3d transformation stuff
    pub fn rotate(self, p: Vector3) -> Vector3 {
        let r = self;
        let r_ = self.conjugate();
        let p = Quaternion {
            w: 0.,
            x: p.x,
            y: p.y,
            z: p.z,
        };

        let p_ = r * p * r_;

        Vector3 {
            x: p_.x,
            y: p_.y,
            z: p_.z,
        }
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::identity()
    }
}

// Hamilton Product: https://en.wikipedia.org/wiki/Quaternion#Hamilton_product
impl std::ops::Mul<Quaternion> for Quaternion {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let p = self;
        let q = rhs;

        Self {
            w: p.w * q.w - p.x * q.x - p.y * q.y - p.z * q.z,
            x: p.w * q.x + p.x * q.w + p.y * q.z - p.z * q.y,
            y: p.w * q.y - p.x * q.z + p.y * q.w + p.z * q.x,
            z: p.w * q.z + p.x * q.y - p.y * q.x + p.z * q.w,
        }
    }
}
