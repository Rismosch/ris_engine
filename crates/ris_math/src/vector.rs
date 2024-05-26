use ash::vk;

use crate::matrix::Mat2;

//
// definition
//

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Vec2(pub f32, pub f32);

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Vec3(pub f32, pub f32, pub f32);

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Vec4(pub f32, pub f32, pub f32, pub f32);

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct VkBvec2(pub vk::Bool32, pub vk::Bool32);

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct VkBvec3(pub vk::Bool32, pub vk::Bool32, pub vk::Bool32);

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct VkBvec4(
    pub vk::Bool32,
    pub vk::Bool32,
    pub vk::Bool32,
    pub vk::Bool32,
);

pub fn bool_to_vk(value: bool) -> vk::Bool32 {
    if value {
        vk::TRUE
    } else {
        vk::FALSE
    }
}

pub fn vk_to_bool(value: vk::Bool32) -> bool {
    match value {
        vk::TRUE => true,
        vk::FALSE => false,
        x => panic!(
            "cannot convert vk::Bool32 to bool, because {} is not a defined value",
            x
        ),
    }
}

//
// constructors
//

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
        Self(value.0 .0, value.0 .1, value.1 .0, value.1 .1)
    }
}

impl VkBvec2 {
    pub fn init(value: vk::Bool32) -> Self {
        Self(value, value)
    }

    pub fn from(value0: bool, value1: bool) -> Self {
        Self(bool_to_vk(value0), bool_to_vk(value1))
    }
}

impl VkBvec3 {
    pub fn init(value: vk::Bool32) -> Self {
        Self(value, value, value)
    }

    pub fn from(value0: bool, value1: bool, value2: bool) -> Self {
        Self(bool_to_vk(value0), bool_to_vk(value1), bool_to_vk(value2))
    }
}

impl VkBvec4 {
    pub fn init(value: vk::Bool32) -> Self {
        Self(value, value, value, value)
    }

    pub fn from(value0: bool, value1: bool, value2: bool, value3: bool) -> Self {
        Self(
            bool_to_vk(value0),
            bool_to_vk(value1),
            bool_to_vk(value2),
            bool_to_vk(value3),
        )
    }
}

//
// components
//

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
        self.2 = z
    }

    pub fn set_r(&mut self, r: f32) {
        self.0 = r
    }

    pub fn set_g(&mut self, g: f32) {
        self.1 = g
    }

    pub fn set_b(&mut self, b: f32) {
        self.2 = b
    }

    pub fn set_s(&mut self, s: f32) {
        self.0 = s
    }

    pub fn set_t(&mut self, t: f32) {
        self.1 = t
    }

    pub fn set_p(&mut self, p: f32) {
        self.2 = p
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
        self.2 = z
    }

    pub fn set_w(&mut self, w: f32) {
        self.3 = w
    }

    pub fn set_r(&mut self, r: f32) {
        self.0 = r
    }

    pub fn set_g(&mut self, g: f32) {
        self.1 = g
    }

    pub fn set_b(&mut self, b: f32) {
        self.2 = b
    }

    pub fn set_a(&mut self, a: f32) {
        self.3 = a
    }

    pub fn set_s(&mut self, s: f32) {
        self.0 = s
    }

    pub fn set_t(&mut self, t: f32) {
        self.1 = t
    }

    pub fn set_p(&mut self, p: f32) {
        self.2 = p
    }

    pub fn set_q(&mut self, q: f32) {
        self.3 = q
    }
}

impl VkBvec2 {
    pub fn x(self) -> vk::Bool32 {
        self.0
    }

    pub fn y(self) -> vk::Bool32 {
        self.1
    }

    pub fn r(self) -> vk::Bool32 {
        self.0
    }

    pub fn g(self) -> vk::Bool32 {
        self.1
    }

    pub fn s(self) -> vk::Bool32 {
        self.0
    }

    pub fn t(self) -> vk::Bool32 {
        self.1
    }

    pub fn set_x(&mut self, x: vk::Bool32) {
        self.0 = x
    }

    pub fn set_y(&mut self, y: vk::Bool32) {
        self.1 = y
    }

    pub fn set_r(&mut self, r: vk::Bool32) {
        self.0 = r
    }

    pub fn set_g(&mut self, g: vk::Bool32) {
        self.1 = g
    }

    pub fn set_s(&mut self, s: vk::Bool32) {
        self.0 = s
    }

    pub fn set_t(&mut self, t: vk::Bool32) {
        self.1 = t
    }
}

impl VkBvec3 {
    pub fn x(self) -> vk::Bool32 {
        self.0
    }

    pub fn y(self) -> vk::Bool32 {
        self.1
    }

    pub fn z(self) -> vk::Bool32 {
        self.2
    }

    pub fn r(self) -> vk::Bool32 {
        self.0
    }

    pub fn g(self) -> vk::Bool32 {
        self.1
    }

    pub fn b(self) -> vk::Bool32 {
        self.2
    }

    pub fn s(self) -> vk::Bool32 {
        self.0
    }

    pub fn t(self) -> vk::Bool32 {
        self.1
    }

    pub fn p(self) -> vk::Bool32 {
        self.2
    }

    pub fn set_x(&mut self, x: vk::Bool32) {
        self.0 = x
    }

    pub fn set_y(&mut self, y: vk::Bool32) {
        self.1 = y
    }

    pub fn set_z(&mut self, z: vk::Bool32) {
        self.2 = z
    }

    pub fn set_r(&mut self, r: vk::Bool32) {
        self.0 = r
    }

    pub fn set_g(&mut self, g: vk::Bool32) {
        self.1 = g
    }

    pub fn set_b(&mut self, b: vk::Bool32) {
        self.2 = b
    }

    pub fn set_s(&mut self, s: vk::Bool32) {
        self.0 = s
    }

    pub fn set_t(&mut self, t: vk::Bool32) {
        self.1 = t
    }

    pub fn set_p(&mut self, p: vk::Bool32) {
        self.2 = p
    }
}

impl VkBvec4 {
    pub fn x(self) -> vk::Bool32 {
        self.0
    }

    pub fn y(self) -> vk::Bool32 {
        self.1
    }

    pub fn z(self) -> vk::Bool32 {
        self.2
    }

    pub fn w(self) -> vk::Bool32 {
        self.3
    }

    pub fn r(self) -> vk::Bool32 {
        self.0
    }

    pub fn g(self) -> vk::Bool32 {
        self.1
    }

    pub fn b(self) -> vk::Bool32 {
        self.2
    }

    pub fn a(self) -> vk::Bool32 {
        self.3
    }

    pub fn s(self) -> vk::Bool32 {
        self.0
    }

    pub fn t(self) -> vk::Bool32 {
        self.1
    }

    pub fn p(self) -> vk::Bool32 {
        self.2
    }

    pub fn q(self) -> vk::Bool32 {
        self.3
    }

    pub fn set_x(&mut self, x: vk::Bool32) {
        self.0 = x
    }

    pub fn set_y(&mut self, y: vk::Bool32) {
        self.1 = y
    }

    pub fn set_z(&mut self, z: vk::Bool32) {
        self.2 = z
    }

    pub fn set_w(&mut self, w: vk::Bool32) {
        self.3 = w
    }

    pub fn set_r(&mut self, r: vk::Bool32) {
        self.0 = r
    }

    pub fn set_g(&mut self, g: vk::Bool32) {
        self.1 = g
    }

    pub fn set_b(&mut self, b: vk::Bool32) {
        self.2 = b
    }

    pub fn set_a(&mut self, a: vk::Bool32) {
        self.3 = a
    }

    pub fn set_s(&mut self, s: vk::Bool32) {
        self.0 = s
    }

    pub fn set_t(&mut self, t: vk::Bool32) {
        self.1 = t
    }

    pub fn set_p(&mut self, p: vk::Bool32) {
        self.2 = p
    }

    pub fn set_q(&mut self, q: vk::Bool32) {
        self.3 = q
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

impl std::ops::Index<usize> for VkBvec2 {
    type Output = vk::Bool32;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 2);

        match index {
            0 => &self.0,
            1 => &self.1,
            _ => unreachable!(),
        }
    }
}

impl std::ops::IndexMut<usize> for VkBvec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 2);

        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Index<usize> for VkBvec3 {
    type Output = vk::Bool32;

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

impl std::ops::IndexMut<usize> for VkBvec3 {
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

impl std::ops::Index<usize> for VkBvec4 {
    type Output = vk::Bool32;

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

impl std::ops::IndexMut<usize> for VkBvec4 {
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
// operations
//

impl std::ops::Add<f32> for Vec2 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self(self.0 + rhs, self.1 + rhs)
    }
}

impl std::ops::Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl std::ops::Add<f32> for Vec4 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self(self.0 + rhs, self.1 + rhs, self.2 + rhs, self.3 + rhs)
    }
}

impl std::ops::Add<Vec2> for f32 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2(self + rhs.0, self + rhs.1)
    }
}

impl std::ops::Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self + rhs.0, self + rhs.1, self + rhs.2)
    }
}

impl std::ops::Add<Vec4> for f32 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Self::Output {
        Vec4(self + rhs.0, self + rhs.1, self + rhs.2, self + rhs.3)
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Add<Vec4> for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl std::ops::Sub<f32> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self(self.0 - rhs, self.1 - rhs)
    }
}

impl std::ops::Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self(self.0 - rhs, self.1 - rhs, self.2 - rhs)
    }
}

impl std::ops::Sub<f32> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self(self.0 - rhs, self.1 - rhs, self.2 - rhs, self.3 - rhs)
    }
}

impl std::ops::Sub<Vec2> for f32 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2(self - rhs.0, self - rhs.1)
    }
}

impl std::ops::Sub<Vec3> for f32 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self - rhs.0, self - rhs.1, self - rhs.2)
    }
}

impl std::ops::Sub<Vec4> for f32 {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Self::Output {
        Vec4(self - rhs.0, self - rhs.1, self - rhs.2, self - rhs.3)
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::Sub<Vec4> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2(self * rhs.0, self * rhs.1)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl std::ops::Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4(self * rhs.0, self * rhs.1, self * rhs.2, self * rhs.3)
    }
}

impl std::ops::Mul<Vec2> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl std::ops::Mul<Vec4> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
            self.3 * rhs.3,
        )
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl std::ops::Div<f32> for Vec4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}

impl std::ops::Div<Vec2> for f32 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Self::Output {
        Vec2(self / rhs.0, self / rhs.1)
    }
}

impl std::ops::Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3(self / rhs.0, self / rhs.1, self / rhs.2)
    }
}
impl std::ops::Div<Vec4> for f32 {
    type Output = Vec4;

    fn div(self, rhs: Vec4) -> Self::Output {
        Vec4(self / rhs.0, self / rhs.1, self / rhs.2, self / rhs.3)
    }
}

impl std::ops::Div<Vec2> for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl std::ops::Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl std::ops::Div<Vec4> for Vec4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(
            self.0 / rhs.0,
            self.1 / rhs.1,
            self.2 / rhs.2,
            self.3 / rhs.3,
        )
    }
}

impl std::ops::AddAssign<f32> for Vec2 {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl std::ops::AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl std::ops::AddAssign<f32> for Vec4 {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl std::ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::AddAssign<Vec4> for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign<f32> for Vec2 {
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl std::ops::SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl std::ops::SubAssign<f32> for Vec4 {
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl std::ops::SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::SubAssign<Vec4> for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl std::ops::MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl std::ops::MulAssign<Vec2> for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::MulAssign<Vec4> for Vec4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl std::ops::DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl std::ops::DivAssign<Vec2> for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl std::ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl std::ops::DivAssign<Vec4> for Vec4 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.
    }
}

impl std::ops::Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.
    }
}

impl std::ops::BitOr<vk::Bool32> for VkBvec2 {
    type Output = Self;

    fn bitor(self, rhs: vk::Bool32) -> Self::Output {
        Self(self.0 | rhs, self.1 | rhs)
    }
}

impl std::ops::BitOr<vk::Bool32> for VkBvec3 {
    type Output = Self;

    fn bitor(self, rhs: vk::Bool32) -> Self::Output {
        Self(self.0 | rhs, self.1 | rhs, self.2 | rhs)
    }
}

impl std::ops::BitOr<vk::Bool32> for VkBvec4 {
    type Output = Self;

    fn bitor(self, rhs: vk::Bool32) -> Self::Output {
        Self(self.0 | rhs, self.1 | rhs, self.2 | rhs, self.3 | rhs)
    }
}

impl std::ops::BitOr<VkBvec2> for VkBvec2 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0, self.1 | rhs.1)
    }
}

impl std::ops::BitOr<VkBvec3> for VkBvec3 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0, self.1 | rhs.1, self.2 | rhs.2)
    }
}

impl std::ops::BitOr<Self> for VkBvec4 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(
            self.0 | rhs.0,
            self.1 | rhs.1,
            self.2 | rhs.2,
            self.3 | rhs.3,
        )
    }
}

impl std::ops::BitAnd<vk::Bool32> for VkBvec2 {
    type Output = Self;

    fn bitand(self, rhs: vk::Bool32) -> Self::Output {
        Self(self.0 & rhs, self.1 & rhs)
    }
}

impl std::ops::BitAnd<vk::Bool32> for VkBvec3 {
    type Output = Self;

    fn bitand(self, rhs: vk::Bool32) -> Self::Output {
        Self(self.0 & rhs, self.1 & rhs, self.2 & rhs)
    }
}

impl std::ops::BitAnd<vk::Bool32> for VkBvec4 {
    type Output = Self;

    fn bitand(self, rhs: vk::Bool32) -> Self::Output {
        Self(self.0 & rhs, self.1 & rhs, self.2 & rhs, self.3 & rhs)
    }
}

impl std::ops::BitAnd<VkBvec2> for VkBvec2 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0, self.1 & rhs.1)
    }
}

impl std::ops::BitAnd<VkBvec3> for VkBvec3 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0, self.1 & rhs.1, self.2 & rhs.2)
    }
}

impl std::ops::BitAnd<Self> for VkBvec4 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(
            self.0 & rhs.0,
            self.1 & rhs.1,
            self.2 & rhs.2,
            self.3 & rhs.3,
        )
    }
}

//
// common functions
//

impl Vec2 {
    pub fn abs(self) -> Self {
        Self(crate::f32::abs(self.0), crate::f32::abs(self.1))
    }

    pub fn sign(self) -> Self {
        Self(crate::f32::sign(self.0), crate::f32::sign(self.1))
    }

    pub fn floor(self) -> Self {
        Self(crate::f32::floor(self.0), crate::f32::floor(self.1))
    }

    pub fn ceil(self) -> Self {
        Self(crate::f32::ceil(self.0), crate::f32::ceil(self.1))
    }

    pub fn trunc(self) -> Self {
        Self(crate::f32::trunc(self.0), crate::f32::trunc(self.1))
    }

    pub fn round(self) -> Self {
        Self(crate::f32::round(self.0), crate::f32::round(self.1))
    }

    pub fn fract(self) -> Self {
        Self(crate::f32::fract(self.0), crate::f32::fract(self.1))
    }

    pub fn modulo(self, rhs: Self) -> Self {
        Self(
            crate::f32::modulo(self.0, rhs.0),
            crate::f32::modulo(self.1, rhs.1),
        )
    }

    pub fn min(x: Self, y: Self) -> Self {
        Self(crate::f32::min(x.0, y.0), crate::f32::min(x.1, y.1))
    }

    pub fn max(x: Self, y: Self) -> Self {
        Self(crate::f32::max(x.0, y.0), crate::f32::max(x.1, y.1))
    }

    pub fn clamp(self, min_val: Self, max_val: Self) -> Self {
        Self(
            crate::f32::clamp(self.0, min_val.0, max_val.0),
            crate::f32::clamp(self.1, min_val.1, max_val.1),
        )
    }

    pub fn mix(x: Self, y: Self, a: Self) -> Self {
        Self(
            crate::f32::mix(x.0, y.0, a.0),
            crate::f32::mix(x.1, y.1, a.1),
        )
    }

    pub fn step(edge: Self, x: Self) -> Self {
        Self(crate::f32::step(edge.0, x.0), crate::f32::step(edge.1, x.1))
    }

    pub fn smoothstep(edge0: Self, edge1: Self, x: Self) -> Self {
        Self(
            crate::f32::smoothstep(edge0.0, edge1.0, x.0),
            crate::f32::smoothstep(edge0.1, edge1.1, x.1),
        )
    }

    pub fn smootherstep(edge0: Self, edge1: Self, x: Self) -> Self {
        Self(
            crate::f32::smootherstep(edge0.0, edge1.0, x.0),
            crate::f32::smootherstep(edge0.1, edge1.1, x.1),
        )
    }

    pub fn is_nan(self) -> VkBvec2 {
        VkBvec2::from(crate::f32::is_nan(self.0), crate::f32::is_nan(self.1))
    }

    pub fn is_inf(self) -> VkBvec2 {
        VkBvec2::from(crate::f32::is_inf(self.0), crate::f32::is_inf(self.1))
    }
}

impl Vec3 {
    pub fn abs(self) -> Self {
        Self(
            crate::f32::abs(self.0),
            crate::f32::abs(self.1),
            crate::f32::abs(self.2),
        )
    }

    pub fn sign(self) -> Self {
        Self(
            crate::f32::sign(self.0),
            crate::f32::sign(self.1),
            crate::f32::sign(self.2),
        )
    }

    pub fn floor(self) -> Self {
        Self(
            crate::f32::floor(self.0),
            crate::f32::floor(self.1),
            crate::f32::floor(self.2),
        )
    }

    pub fn ceil(self) -> Self {
        Self(
            crate::f32::ceil(self.0),
            crate::f32::ceil(self.1),
            crate::f32::ceil(self.2),
        )
    }

    pub fn trunc(self) -> Self {
        Self(
            crate::f32::trunc(self.0),
            crate::f32::trunc(self.1),
            crate::f32::trunc(self.2),
        )
    }

    pub fn round(self) -> Self {
        Self(
            crate::f32::round(self.0),
            crate::f32::round(self.1),
            crate::f32::round(self.2),
        )
    }

    pub fn fract(self) -> Self {
        Self(
            crate::f32::fract(self.0),
            crate::f32::fract(self.1),
            crate::f32::fract(self.2),
        )
    }

    pub fn modulo(self, rhs: Self) -> Self {
        Self(
            crate::f32::modulo(self.0, rhs.0),
            crate::f32::modulo(self.1, rhs.1),
            crate::f32::modulo(self.2, rhs.2),
        )
    }

    pub fn min(x: Self, y: Self) -> Self {
        Self(
            crate::f32::min(x.0, y.0),
            crate::f32::min(x.1, y.1),
            crate::f32::min(x.2, y.2),
        )
    }

    pub fn max(x: Self, y: Self) -> Self {
        Self(
            crate::f32::max(x.0, y.0),
            crate::f32::max(x.1, y.1),
            crate::f32::max(x.2, y.2),
        )
    }

    pub fn clamp(self, min_val: Self, max_val: Self) -> Self {
        Self(
            crate::f32::clamp(self.0, min_val.0, max_val.0),
            crate::f32::clamp(self.1, min_val.1, max_val.1),
            crate::f32::clamp(self.2, min_val.2, max_val.2),
        )
    }

    pub fn mix(x: Self, y: Self, a: Self) -> Self {
        Self(
            crate::f32::mix(x.0, y.0, a.0),
            crate::f32::mix(x.1, y.1, a.1),
            crate::f32::mix(x.2, y.2, a.2),
        )
    }

    pub fn step(edge: Self, x: Self) -> Self {
        Self(
            crate::f32::step(edge.0, x.0),
            crate::f32::step(edge.1, x.1),
            crate::f32::step(edge.2, x.2),
        )
    }

    pub fn smoothstep(edge0: Self, edge1: Self, x: Self) -> Self {
        Self(
            crate::f32::smoothstep(edge0.0, edge1.0, x.0),
            crate::f32::smoothstep(edge0.1, edge1.1, x.1),
            crate::f32::smoothstep(edge0.2, edge1.2, x.2),
        )
    }

    pub fn smootherstep(edge0: Self, edge1: Self, x: Self) -> Self {
        Self(
            crate::f32::smootherstep(edge0.0, edge1.0, x.0),
            crate::f32::smootherstep(edge0.1, edge1.1, x.1),
            crate::f32::smootherstep(edge0.2, edge1.2, x.2),
        )
    }

    pub fn is_nan(self) -> VkBvec3 {
        VkBvec3::from(
            crate::f32::is_nan(self.0),
            crate::f32::is_nan(self.1),
            crate::f32::is_nan(self.2),
        )
    }

    pub fn is_inf(self) -> VkBvec3 {
        VkBvec3::from(
            crate::f32::is_inf(self.0),
            crate::f32::is_inf(self.1),
            crate::f32::is_inf(self.2),
        )
    }
}

impl Vec4 {
    pub fn abs(self) -> Self {
        Self(
            crate::f32::abs(self.0),
            crate::f32::abs(self.1),
            crate::f32::abs(self.2),
            crate::f32::abs(self.3),
        )
    }

    pub fn sign(self) -> Self {
        Self(
            crate::f32::sign(self.0),
            crate::f32::sign(self.1),
            crate::f32::sign(self.2),
            crate::f32::sign(self.3),
        )
    }

    pub fn floor(self) -> Self {
        Self(
            crate::f32::floor(self.0),
            crate::f32::floor(self.1),
            crate::f32::floor(self.2),
            crate::f32::floor(self.3),
        )
    }

    pub fn ceil(self) -> Self {
        Self(
            crate::f32::ceil(self.0),
            crate::f32::ceil(self.1),
            crate::f32::ceil(self.2),
            crate::f32::ceil(self.3),
        )
    }

    pub fn trunc(self) -> Self {
        Self(
            crate::f32::trunc(self.0),
            crate::f32::trunc(self.1),
            crate::f32::trunc(self.2),
            crate::f32::trunc(self.3),
        )
    }

    pub fn round(self) -> Self {
        Self(
            crate::f32::round(self.0),
            crate::f32::round(self.1),
            crate::f32::round(self.2),
            crate::f32::round(self.3),
        )
    }

    pub fn fract(self) -> Self {
        Self(
            crate::f32::fract(self.0),
            crate::f32::fract(self.1),
            crate::f32::fract(self.2),
            crate::f32::fract(self.3),
        )
    }

    pub fn modulo(self, rhs: Self) -> Self {
        Self(
            crate::f32::modulo(self.0, rhs.0),
            crate::f32::modulo(self.1, rhs.1),
            crate::f32::modulo(self.2, rhs.2),
            crate::f32::modulo(self.3, rhs.3),
        )
    }

    pub fn min(x: Self, y: Self) -> Self {
        Self(
            crate::f32::min(x.0, y.0),
            crate::f32::min(x.1, y.1),
            crate::f32::min(x.2, y.2),
            crate::f32::min(x.3, y.3),
        )
    }

    pub fn max(x: Self, y: Self) -> Self {
        Self(
            crate::f32::max(x.0, y.0),
            crate::f32::max(x.1, y.1),
            crate::f32::max(x.2, y.2),
            crate::f32::max(x.3, y.3),
        )
    }

    pub fn clamp(self, min_val: Self, max_val: Self) -> Self {
        Self(
            crate::f32::clamp(self.0, min_val.0, max_val.0),
            crate::f32::clamp(self.1, min_val.1, max_val.1),
            crate::f32::clamp(self.2, min_val.2, max_val.2),
            crate::f32::clamp(self.3, min_val.3, max_val.3),
        )
    }

    pub fn mix(x: Self, y: Self, a: Self) -> Self {
        Self(
            crate::f32::mix(x.0, y.0, a.0),
            crate::f32::mix(x.1, y.1, a.1),
            crate::f32::mix(x.2, y.2, a.2),
            crate::f32::mix(x.3, y.3, a.3),
        )
    }

    pub fn step(edge: Self, x: Self) -> Self {
        Self(
            crate::f32::step(edge.0, x.0),
            crate::f32::step(edge.1, x.1),
            crate::f32::step(edge.2, x.2),
            crate::f32::step(edge.3, x.3),
        )
    }

    pub fn smoothstep(edge0: Self, edge1: Self, x: Self) -> Self {
        Self(
            crate::f32::smoothstep(edge0.0, edge1.0, x.0),
            crate::f32::smoothstep(edge0.1, edge1.1, x.1),
            crate::f32::smoothstep(edge0.2, edge1.2, x.2),
            crate::f32::smoothstep(edge0.3, edge1.3, x.3),
        )
    }

    pub fn smootherstep(edge0: Self, edge1: Self, x: Self) -> Self {
        Self(
            crate::f32::smootherstep(edge0.0, edge1.0, x.0),
            crate::f32::smootherstep(edge0.1, edge1.1, x.1),
            crate::f32::smootherstep(edge0.2, edge1.2, x.2),
            crate::f32::smootherstep(edge0.3, edge1.3, x.3),
        )
    }

    pub fn is_nan(self) -> VkBvec4 {
        VkBvec4::from(
            crate::f32::is_nan(self.0),
            crate::f32::is_nan(self.1),
            crate::f32::is_nan(self.2),
            crate::f32::is_nan(self.3),
        )
    }

    pub fn is_inf(self) -> VkBvec4 {
        VkBvec4::from(
            crate::f32::is_inf(self.0),
            crate::f32::is_inf(self.1),
            crate::f32::is_inf(self.2),
            crate::f32::is_inf(self.3),
        )
    }
}

//
// geometric functions
//

impl Vec2 {
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        crate::f32::sqrt(self.length_squared())
    }

    pub fn distance_squared(self, rhs: Self) -> f32 {
        (self - rhs).length_squared()
    }

    pub fn distance(self, rhs: Self) -> f32 {
        (self - rhs).length()
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }

    pub fn face_forward(self, i: Self, n_ref: Self) -> Self {
        if n_ref.dot(i) < 0. {
            self
        } else {
            -self
        }
    }

    /// for the incident vector self and surface orientation n, returns the reflection direction
    ///
    /// n must already be normalized in order to achieve the desired result.
    pub fn reflect(self, n: Self) -> Self {
        let i = self;
        i - 2. * n.dot(i) * n
    }

    /// for the incident vector self and surface normal n, and the ratio of indices of refraction
    /// eta, return the refraction vector
    ///
    /// the input parameters for the incident vector self and the surface normal n must already by
    /// normalized to get the desired results
    pub fn refract(self, n: Self, eta: f32) -> Self {
        let i = self;
        let k = 1. - eta * eta * (1. - n.dot(i) * n.dot(i));
        if k < 0. {
            Self::init(0.)
        } else {
            eta * i - (eta * n.dot(i) + crate::f32::sqrt(k)) * n
        }
    }
}

impl Vec3 {
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        crate::f32::sqrt(self.length_squared())
    }

    pub fn distance_squared(self, rhs: Self) -> f32 {
        (self - rhs).length_squared()
    }

    pub fn distance(self, rhs: Self) -> f32 {
        (self - rhs).length()
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self(
            self.1 * rhs.2 - rhs.1 * self.2,
            self.2 * rhs.0 - rhs.2 * self.0,
            self.0 * rhs.1 - rhs.0 * self.1,
        )
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }

    pub fn face_forward(self, i: Self, n_ref: Self) -> Self {
        if n_ref.dot(i) < 0. {
            self
        } else {
            -self
        }
    }

    /// for the incident vector self and surface orientation n, returns the reflection direction
    ///
    /// n must already be normalized in order to achieve the desired result.
    pub fn reflect(self, n: Self) -> Self {
        let i = self;
        i - 2. * n.dot(i) * n
    }

    /// for the incident vector self and surface normal n, and the ratio of indices of refraction
    /// eta, return the refraction vector
    ///
    /// the input parameters for the incident vector self and the surface normal n must already by
    /// normalized to get the desired results
    pub fn refract(self, n: Self, eta: f32) -> Self {
        let i = self;
        let k = 1. - eta * eta * (1. - n.dot(i) * n.dot(i));
        if k < 0. {
            Self::init(0.)
        } else {
            eta * i - (eta * n.dot(i) + crate::f32::sqrt(k)) * n
        }
    }
}

impl Vec4 {
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        crate::f32::sqrt(self.length_squared())
    }

    pub fn distance_squared(self, rhs: Self) -> f32 {
        (self - rhs).length_squared()
    }

    pub fn distance(self, rhs: Self) -> f32 {
        (self - rhs).length()
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2 + self.3 * rhs.3
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }

    pub fn face_forward(self, i: Self, n_ref: Self) -> Self {
        if n_ref.dot(i) < 0. {
            self
        } else {
            -self
        }
    }

    /// for the incident vector self and surface orientation n, returns the reflection direction
    ///
    /// n must already be normalized in order to achieve the desired result.
    pub fn reflect(self, n: Self) -> Self {
        let i = self;
        i - 2. * n.dot(i) * n
    }

    /// for the incident vector self and surface normal n, and the ratio of indices of refraction
    /// eta, return the refraction vector
    ///
    /// the input parameters for the incident vector self and the surface normal n must already by
    /// normalized to get the desired results
    pub fn refract(self, n: Self, eta: f32) -> Self {
        let i = self;
        let k = 1. - eta * eta * (1. - n.dot(i) * n.dot(i));
        if k < 0. {
            Self::init(0.)
        } else {
            eta * i - (eta * n.dot(i) + crate::f32::sqrt(k)) * n
        }
    }
}

//
// relational functions
//

impl Vec2 {
    pub fn less_than(self, rhs: Self) -> VkBvec2 {
        VkBvec2::from(self.0 < rhs.0, self.1 < rhs.1)
    }

    pub fn less_than_equal(self, rhs: Self) -> VkBvec2 {
        VkBvec2::from(self.0 <= rhs.0, self.1 <= rhs.1)
    }

    pub fn greater_than(self, rhs: Self) -> VkBvec2 {
        VkBvec2::from(self.0 > rhs.0, self.1 > rhs.1)
    }

    pub fn greater_than_equal(self, rhs: Self) -> VkBvec2 {
        VkBvec2::from(self.0 >= rhs.0, self.1 >= rhs.1)
    }

    pub fn equal(self, rhs: Self) -> VkBvec2 {
        VkBvec2::from(self.0 == rhs.0, self.1 == rhs.1)
    }

    pub fn not_equal(self, rhs: Self) -> VkBvec2 {
        VkBvec2::from(self.0 != rhs.0, self.1 != rhs.1)
    }
}

impl Vec3 {
    pub fn less_than(self, rhs: Self) -> VkBvec3 {
        VkBvec3::from(self.0 < rhs.0, self.1 < rhs.1, self.2 < rhs.2)
    }

    pub fn less_than_equal(self, rhs: Self) -> VkBvec3 {
        VkBvec3::from(self.0 <= rhs.0, self.1 <= rhs.1, self.2 <= rhs.2)
    }

    pub fn greater_than(self, rhs: Self) -> VkBvec3 {
        VkBvec3::from(self.0 > rhs.0, self.1 > rhs.1, self.2 > rhs.2)
    }

    pub fn greater_than_equal(self, rhs: Self) -> VkBvec3 {
        VkBvec3::from(self.0 >= rhs.0, self.1 >= rhs.1, self.2 >= rhs.2)
    }

    pub fn equal(self, rhs: Self) -> VkBvec3 {
        VkBvec3::from(self.0 == rhs.0, self.1 == rhs.1, self.2 == rhs.2)
    }

    pub fn not_equal(self, rhs: Self) -> VkBvec3 {
        VkBvec3::from(self.0 != rhs.0, self.1 != rhs.1, self.2 != rhs.2)
    }
}

impl Vec4 {
    pub fn less_than(self, rhs: Self) -> VkBvec4 {
        VkBvec4::from(
            self.0 < rhs.0,
            self.1 < rhs.1,
            self.2 < rhs.2,
            self.3 < rhs.3,
        )
    }

    pub fn less_than_equal(self, rhs: Self) -> VkBvec4 {
        VkBvec4::from(
            self.0 <= rhs.0,
            self.1 <= rhs.1,
            self.2 <= rhs.2,
            self.3 <= rhs.3,
        )
    }

    pub fn greater_than(self, rhs: Self) -> VkBvec4 {
        VkBvec4::from(
            self.0 > rhs.0,
            self.1 > rhs.1,
            self.2 > rhs.2,
            self.3 > rhs.3,
        )
    }

    pub fn greater_than_equal(self, rhs: Self) -> VkBvec4 {
        VkBvec4::from(
            self.0 >= rhs.0,
            self.1 >= rhs.1,
            self.2 >= rhs.2,
            self.3 >= rhs.3,
        )
    }

    pub fn equal(self, rhs: Self) -> VkBvec4 {
        VkBvec4::from(
            self.0 == rhs.0,
            self.1 == rhs.1,
            self.2 == rhs.2,
            self.3 == rhs.3,
        )
    }

    pub fn not_equal(self, rhs: Self) -> VkBvec4 {
        VkBvec4::from(
            self.0 != rhs.0,
            self.1 != rhs.1,
            self.2 != rhs.2,
            self.3 != rhs.3,
        )
    }
}

impl VkBvec2 {
    pub fn any(self) -> vk::Bool32 {
        self.0 | self.1
    }

    pub fn all(self) -> vk::Bool32 {
        self.0 & self.1
    }
}

impl VkBvec3 {
    pub fn any(self) -> vk::Bool32 {
        self.0 | self.1 | self.2
    }

    pub fn all(self) -> vk::Bool32 {
        self.0 & self.1 & self.2
    }
}

impl VkBvec4 {
    pub fn any(self) -> vk::Bool32 {
        self.0 | self.1 | self.2 | self.3
    }

    pub fn all(self) -> vk::Bool32 {
        self.0 & self.1 | self.2 | self.3
    }
}

impl std::ops::Not for VkBvec2 {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0, !self.1)
    }
}

impl std::ops::Not for VkBvec3 {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0, !self.1, !self.2)
    }
}

impl std::ops::Not for VkBvec4 {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0, !self.1, !self.2, !self.3)
    }
}
