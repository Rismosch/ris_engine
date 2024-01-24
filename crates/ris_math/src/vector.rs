use vulkano::buffer::BufferContents;

use crate::matrix::Mat2;

// definition
#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Vec2(pub f32, pub f32);

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Vec3(pub f32, pub f32, pub f32);

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Vec4(pub f32, pub f32, pub f32, pub f32);

// constructors
impl Vec2 {
    fn init(value: f32) -> Self {
        Self(value, value)
    }
}

impl From<Vec3> for Vec2 {
    fn from(value: Vec3) -> Self {
        Self(value.0, value.1)
    }
}

impl Vec3 {
    fn init(value: f32) -> Self {
        Self(value, value, value)
    }
}

impl From<Vec4> for Vec3 {
    fn from(value: Vec4) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl Vec4 {
    fn init(value: f32) -> Self {
        Self(value, value, value, value)
    }
}

impl From<Mat2> for Vec4 {
    fn from(value: Mat2) -> Self {
        Self(value.0, value.1, value.2, value.3)
    }
}

// components
impl Vec2 {
    pub fn x(self) -> f32 {
        self.0
    }

    pub fn y(self) -> f32 {
        self.1
    }

    pub fn r(self) -> f32 {
        self.0
    }

    pub fn g(self) -> f32 {
        self.1
    }

    pub fn s(self) -> f32 {
        self.0
    }

    pub fn t(self) -> f32 {
        self.1
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.0
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.1
    }

    pub fn r_mut(&mut self) -> &mut f32 {
        &mut self.0
    }

    pub fn g_mut(&mut self) -> &mut f32 {
        &mut self.g
    }

    pub fn s_mut(&mut self) -> &mut f32 {
        &mut self.0
    }

    pub fn t_mut(&mut self) -> &mut f32 {
        &mut self.1
    }
}

impl std::ops::Index<usize> for Vec2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 2);

        match index {
            0 => &self.0,
            1 => &self.1,
            _ => unreachable!(),
        }
    }
}

impl std::ops::IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 2);

        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => unreachable!(),
        }
    }
}

impl Vec3 {
    pub fn x(self) -> f32 {
        self.0
    }

    pub fn y(self) -> f32 {
        self.1
    }

    pub fn z(self) -> f32 {
        self.2
    }

    pub fn r(self) -> f32 {
        self.0
    }

    pub fn g(self) -> f32 {
        self.1
    }

    pub fn b(self) -> f32 {
        self.2
    }

    pub fn s(self) -> f32 {
        self.0
    }

    pub fn t(self) -> f32 {
        self.1
    }

    pub fn p(self) -> f32 {
        self.2
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.0
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.1
    }

    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.2
    }

    pub fn r_mut(&mut self) -> &mut f32 {
        &mut self.0
    }

    pub fn g_mut(&mut self) -> &mut f32 {
        &mut self.g
    }

    pub fn b_mut(&mut self) -> &mut f32 {
        &mut self.2
    }

    pub fn s_mut(&mut self) -> &mut f32 {
        &mut self.0
    }

    pub fn t_mut(&mut self) -> &mut f32 {
        &mut self.1
    }

    pub fn p_mut(&mut self) -> &mut f32 {
        &mut self.2
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 3);

        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => unreachable!(),
        }
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 3);

        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => unreachable!(),
        }
    }
}

impl Vec4 {
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

    pub fn r(self) -> f32 {
        self.0
    }

    pub fn g(self) -> f32 {
        self.1
    }

    pub fn b(self) -> f32 {
        self.2
    }

    pub fn a(self) -> f32 {
        self.3
    }

    pub fn s(self) -> f32 {
        self.0
    }

    pub fn t(self) -> f32 {
        self.1
    }

    pub fn p(self) -> f32 {
        self.2
    }

    pub fn q(self) -> f32 {
        self.3
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.0
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.1
    }

    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.2
    }

    pub fn w_mut(&mut self) -> &mut f32 {
        &mut self.3
    }

    pub fn r_mut(&mut self) -> &mut f32 {
        &mut self.0
    }

    pub fn g_mut(&mut self) -> &mut f32 {
        &mut self.g
    }

    pub fn b_mut(&mut self) -> &mut f32 {
        &mut self.2
    }
    
    pub fn a_mut(&mut self) -> &mut f32 {
        &mut self.3
    }

    pub fn s_mut(&mut self) -> &mut f32 {
        &mut self.0
    }

    pub fn t_mut(&mut self) -> &mut f32 {
        &mut self.1
    }

    pub fn p_mut(&mut self) -> &mut f32 {
        &mut self.2
    }

    pub fn q_mut(&mut self) -> &mut f32 {
        &mut self.3
    }
}

impl std::ops::Index<usize> for Vec4 {
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

impl std::ops::IndexMut<usize> for Vec4 {
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

// operations
impl std::ops::Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(mut self, rhs: f32) -> Self::Output {
        let mut v = Vec3::default();
        let u = self;
        let f = rhs;

        v.0 = u.0 + f;
        v.1 = u.1 + f;
        v.2 = u.2 + f;

        v
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(mut self, rhs: Vec3) -> Self::Output {
        let v = self;
        let u = rhs;
        let mut w = Vec3::default();

        w.0 = v.0 + u.0;
        w.1 = v.1 + u.1;
        w.2 = v.2 + u.2;

        w
    }
}








// utility
pub const RIGHT: Vec3 = Vec3(1., 0., 0.);
pub const LEFT: Vec3 = Vec3(-1., 0., 0.);
pub const FORWARD: Vec3 = Vec3(0., 1., 0.);
pub const BACKWARD: Vec3 = Vec3(0., -1., 0.);
pub const UP: Vec3 = Vec3(0., 0., 1.);
pub const DOWN: Vec3 = Vec3(0., 0., -1.);



impl Vector3 {
    // initialization
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    // utility
    pub fn dot(a: Vector3, b: Vector3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    /// Think of `a` as the thumb of your right hand and of `b` as the index finger of your right
    /// hand. The result will be the middle finger.
    pub fn cross(a: Vector3, b: Vector3) -> Self {
        Self {
            x: a.y * b.z - b.y * a.z,
            y: a.z * b.x - b.z * a.x,
            z: a.x * b.y - b.x * a.y,
        }
    }

    pub fn normalized(self) -> Self {
        let magnitude = self.magnitude();
        if magnitude < super::MIN_NORM {
            ZERO
        } else {
            Self {
                x: self.x / magnitude,
                y: self.y / magnitude,
                z: self.z / magnitude,
            }
        }
    }

    pub fn inverted(self) -> Self {
        -1. * self
    }

    pub fn magnitude_squared(self) -> f32 {
        Self::dot(self, self)
    }

    pub fn magnitude(self) -> f32 {
        super::sqrt(self.magnitude_squared())
    }
}

impl std::ops::Add<Vector3> for Vector3 {
    type Output = Self;
    fn add(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul<Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}
