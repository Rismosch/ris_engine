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
    pub fn init(value: f32) -> Self {
        Self(value, value)
    }
}

impl From<Vec3> for Vec2 {
    fn from(value: Vec3) -> Self {
        Self(value.0, value.1)
    }
}

impl Vec3 {
    pub fn init(value: f32) -> Self {
        Self(value, value, value)
    }

    pub fn right() -> Self {
        Self(1., 0., 0.)
    }

    pub fn left() -> Self {
        Self(-1., 0., 0.)
    }

    pub fn forward() -> Self {
        Self(0., 1., 0.)
    }

    pub fn backward() -> Self {
        Self(0., -1., 0.)
    }

    pub fn up() -> Self {
        Self(0., 0., 1.)
    }

    pub fn down() -> Self {
        Self(0., 0., -1.)
    }
}

impl From<Vec4> for Vec3 {
    fn from(value: Vec4) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl Vec4 {
    pub fn init(value: f32) -> Self {
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

    pub fn set_x(&mut self, x: f32) {
        self.0 = x
    }

    pub fn set_y(&mut self, y: f32) {
        self.1 = y
    }

    pub fn set_r(&mut self, r: f32) {
        self.0 = r
    }

    pub fn set_g(&mut self, g: f32) {
        self.1 = g
    }

    pub fn set_s(&mut self, s: f32) {
        self.0 = s
    }

    pub fn set_t(&mut self, t: f32) {
        self.1 = t
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

    pub fn set_x(&mut self, x: f32) {
        self.0 = x
    }

    pub fn set_y(&mut self, y: f32) {
        self.1 = y
    }

    pub fn set_z(&mut self, z: f32) {
        self.3 = z
    }

    pub fn set_r(&mut self, r: f32) {
        self.0 = r
    }

    pub fn set_g(&mut self, g: f32) {
        self.1 = g
    }

    pub fn set_b(&mut self, b: f32) {
        self.3 = b
    }

    pub fn set_s(&mut self, s: f32) {
        self.0 = s
    }

    pub fn set_t(&mut self, t: f32) {
        self.1 = t
    }

    pub fn set_p(&mut self, p: f32) {
        self.3 = p
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

    pub fn set_x(&mut self, x: f32) {
        self.0 = x
    }

    pub fn set_y(&mut self, y: f32) {
        self.1 = y
    }

    pub fn set_z(&mut self, z: f32) {
        self.3 = z
    }

    pub fn set_w(&mut self, w: f32) {
        self.4 = w
    }

    pub fn set_r(&mut self, r: f32) {
        self.0 = r
    }

    pub fn set_g(&mut self, g: f32) {
        self.1 = g
    }

    pub fn set_b(&mut self, b: f32) {
        self.3 = b
    }

    pub fn set_w(&mut self, a: f32) {
        self.4 = a
    }

    pub fn set_s(&mut self, s: f32) {
        self.0 = s
    }

    pub fn set_t(&mut self, t: f32) {
        self.1 = t
    }

    pub fn set_p(&mut self, p: f32) {
        self.3 = p
    }

    pub fn set_q(&mut self, q: f32) {
        self.4 = q
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

// functions
mix
step
smoothstep
geometric
vector relational









// utility



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
