use crate::vector::Vec3;
use crate::vector::Vec4;

//
// definition
//

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Quat(pub f32, pub f32, pub f32, pub f32);

pub type AngleAxis = (f32, Vec3);

//
// constructors
//

impl Quat {
    pub fn identity() -> Self {
        Self(0., 0., 0., 1.)
    }
}

impl Default for Quat {
    fn default() -> Self {
        Self::identity()
    }
}

impl From<[f32; 4]> for Quat {
    fn from(value: [f32; 4]) -> Self {
        Quat(value[0], value[1], value[2], value[3])
    }
}

impl From<Quat> for [f32; 4] {
    fn from(value: Quat) -> Self {
        [value.0, value.1, value.2, value.3]
    }
}

impl From<Vec4> for Quat {
    fn from(value: Vec4) -> Self {
        Self(value.0, value.1, value.2, value.3)
    }
}

impl From<Quat> for Vec4 {
    fn from(value: Quat) -> Self {
        Self(value.0, value.1, value.2, value.3)
    }
}

impl From<AngleAxis> for Quat {
    fn from(value: AngleAxis) -> Self {
        let angle = value.0;
        let axis = value.1;

        let n = axis.normalize();
        let t = angle * 0.5;
        let re = f32::cos(t);
        let im = f32::sin(t);

        Self(n.0 * im, n.1 * im, n.2 * im, re)
    }
}

impl From<Quat> for AngleAxis {
    fn from(value: Quat) -> Self {
        let mut q = value;

        // if w>1 acos and sqrt will produce errors, this cant happen if quaternion is normalized
        if q.3.abs() > 1. {
            q = q.normalize();
        }

        let t = 2. * f32::acos(q.3);
        let s = f32::sqrt(1. - q.3 * q.3);

        let n = if s < 0.001 {
            Vec3(1., 0., 0.)
        } else {
            Vec3(q.0 / s, q.1 / s, q.2 / s)
        };

        (t, n)
    }
}

//
// Components
//

impl Quat {
    pub fn x(self) -> f32 {
        self.0
    }

    pub fn y(self) -> f32 {
        self.1
    }

    pub fn z(self) -> f32 {
        self.2
    }

    pub fn w(self) -> f32 {
        self.3
    }

    pub fn i(self) -> f32 {
        self.0
    }

    pub fn j(self) -> f32 {
        self.1
    }

    pub fn k(self) -> f32 {
        self.2
    }

    pub fn re(self) -> f32 {
        self.3
    }

    pub fn set_x(&mut self, x: f32) {
        self.0 = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.1 = y;
    }

    pub fn set_z(&mut self, z: f32) {
        self.2 = z;
    }

    pub fn set_w(&mut self, w: f32) {
        self.3 = w;
    }

    pub fn set_i(&mut self, i: f32) {
        self.0 = i;
    }

    pub fn set_j(&mut self, j: f32) {
        self.1 = j;
    }

    pub fn set_k(&mut self, k: f32) {
        self.2 = k;
    }

    pub fn set_re(&mut self, re: f32) {
        self.3 = re;
    }
}

impl std::ops::Index<usize> for Quat {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 4);

        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => unreachable!(),
        }
    }
}

impl std::ops::IndexMut<usize> for Quat {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 4);

        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => unreachable!(),
        }
    }
}

//
// functions
//

impl Quat {
    // initialization
    // utility
    pub fn dot(p: Quat, q: Quat) -> f32 {
        let p_ = Vec4::from(p);
        let q_ = Vec4::from(q);

        p_.dot(q_)
    }

    pub fn conjugate(self) -> Self {
        Self(-self.0, -self.1, -self.2, self.3)
    }

    pub fn normalize(self) -> Self {
        let q_ = Vec4::from(self);
        Quat::from(q_.normalize())
    }

    pub fn length_squared(self) -> f32 {
        let q_ = Vec4::from(self);
        q_.length_squared()
    }

    pub fn length(self) -> f32 {
        let q_ = Vec4::from(self);
        q_.length()
    }

    // 3d transformation stuff
    pub fn rotate(self, p: Vec3) -> Vec3 {
        let r = self;
        let r_ = self.conjugate();
        let p = Quat(p.0, p.1, p.2, 0.);

        let p_ = r * p * r_;

        Vec3(p_.0, p_.1, p_.2)
    }
}

// Hamilton Product: https://en.wikipedia.org/wiki/Quaternion#Hamilton_product
impl std::ops::Mul<Quat> for Quat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let Quat(b1, c1, d1, a1) = self;
        let Quat(b2, c2, d2, a2) = rhs;

        let re = a1 * a2 - b1 * b2 - c1 * c2 - d1 * d2;
        let i = a1 * b2 + b1 * a2 + c1 * d2 - d1 * c2;
        let j = a1 * c2 - b1 * d2 + c1 * a2 + d1 * b2;
        let k = a1 * d2 + b1 * c2 - c1 * b2 + d1 * a2;

        Self(i, j, k, re)
    }
}
