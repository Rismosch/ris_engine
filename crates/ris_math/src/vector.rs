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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Bvec2(pub bool, pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Bvec3(pub bool, pub bool, pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Bvec4(pub bool, pub bool, pub bool, pub bool);

//
// constructors
//

impl Vec2 {
    pub fn init(value: f32) -> Self {
        Self(value, value)
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(value: [f32; 2]) -> Self {
        Self(value[0], value[1])
    }
}

impl From<Vec2> for [f32; 2] {
    fn from(value: Vec2) -> Self {
        [value.0, value.1]
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

    pub fn right() -> Vec3 {
        Vec3(1., 0., 0.)
    }

    pub fn left() -> Vec3 {
        Vec3(-1., 0., 0.)
    }

    pub fn forward() -> Vec3 {
        Vec3(0., 1., 0.)
    }

    pub fn backward() -> Vec3 {
        Vec3(0., -1., 0.)
    }

    pub fn up() -> Vec3 {
        Vec3(0., 0., 1.)
    }

    pub fn down() -> Vec3 {
        Vec3(0., 0., -1.)
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(value: [f32; 3]) -> Self {
        Self(value[0], value[1], value[2])
    }
}

impl From<Vec3> for [f32; 3] {
    fn from(value: Vec3) -> Self {
        [value.0, value.1, value.2]
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

impl From<[f32; 4]> for Vec4 {
    fn from(value: [f32; 4]) -> Self {
        Self(value[0], value[1], value[2], value[3])
    }
}

impl From<Vec4> for [f32; 4] {
    fn from(value: Vec4) -> Self {
        [value.0, value.1, value.2, value.3]
    }
}

impl Bvec2 {
    pub fn init(value: bool) -> Self {
        Self(value, value)
    }
}

impl Bvec3 {
    pub fn init(value: bool) -> Self {
        Self(value, value, value)
    }
}

impl Bvec4 {
    pub fn init(value: bool) -> Self {
        Self(value, value, value, value)
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

impl Bvec2 {
    pub fn x(self) -> bool {
        self.0
    }

    pub fn y(self) -> bool {
        self.1
    }

    pub fn r(self) -> bool {
        self.0
    }

    pub fn g(self) -> bool {
        self.1
    }

    pub fn s(self) -> bool {
        self.0
    }

    pub fn t(self) -> bool {
        self.1
    }

    pub fn set_x(&mut self, x: bool) {
        self.0 = x
    }

    pub fn set_y(&mut self, y: bool) {
        self.1 = y
    }

    pub fn set_r(&mut self, r: bool) {
        self.0 = r
    }

    pub fn set_g(&mut self, g: bool) {
        self.1 = g
    }

    pub fn set_s(&mut self, s: bool) {
        self.0 = s
    }

    pub fn set_t(&mut self, t: bool) {
        self.1 = t
    }
}

impl Bvec3 {
    pub fn x(self) -> bool {
        self.0
    }

    pub fn y(self) -> bool {
        self.1
    }

    pub fn z(self) -> bool {
        self.2
    }

    pub fn r(self) -> bool {
        self.0
    }

    pub fn g(self) -> bool {
        self.1
    }

    pub fn b(self) -> bool {
        self.2
    }

    pub fn s(self) -> bool {
        self.0
    }

    pub fn t(self) -> bool {
        self.1
    }

    pub fn p(self) -> bool {
        self.2
    }

    pub fn set_x(&mut self, x: bool) {
        self.0 = x
    }

    pub fn set_y(&mut self, y: bool) {
        self.1 = y
    }

    pub fn set_z(&mut self, z: bool) {
        self.2 = z
    }

    pub fn set_r(&mut self, r: bool) {
        self.0 = r
    }

    pub fn set_g(&mut self, g: bool) {
        self.1 = g
    }

    pub fn set_b(&mut self, b: bool) {
        self.2 = b
    }

    pub fn set_s(&mut self, s: bool) {
        self.0 = s
    }

    pub fn set_t(&mut self, t: bool) {
        self.1 = t
    }

    pub fn set_p(&mut self, p: bool) {
        self.2 = p
    }
}

impl Bvec4 {
    pub fn x(self) -> bool {
        self.0
    }

    pub fn y(self) -> bool {
        self.1
    }

    pub fn z(self) -> bool {
        self.2
    }

    pub fn w(self) -> bool {
        self.3
    }

    pub fn r(self) -> bool {
        self.0
    }

    pub fn g(self) -> bool {
        self.1
    }

    pub fn b(self) -> bool {
        self.2
    }

    pub fn a(self) -> bool {
        self.3
    }

    pub fn s(self) -> bool {
        self.0
    }

    pub fn t(self) -> bool {
        self.1
    }

    pub fn p(self) -> bool {
        self.2
    }

    pub fn q(self) -> bool {
        self.3
    }

    pub fn set_x(&mut self, x: bool) {
        self.0 = x
    }

    pub fn set_y(&mut self, y: bool) {
        self.1 = y
    }

    pub fn set_z(&mut self, z: bool) {
        self.2 = z
    }

    pub fn set_w(&mut self, w: bool) {
        self.3 = w
    }

    pub fn set_r(&mut self, r: bool) {
        self.0 = r
    }

    pub fn set_g(&mut self, g: bool) {
        self.1 = g
    }

    pub fn set_b(&mut self, b: bool) {
        self.2 = b
    }

    pub fn set_a(&mut self, a: bool) {
        self.3 = a
    }

    pub fn set_s(&mut self, s: bool) {
        self.0 = s
    }

    pub fn set_t(&mut self, t: bool) {
        self.1 = t
    }

    pub fn set_p(&mut self, p: bool) {
        self.2 = p
    }

    pub fn set_q(&mut self, q: bool) {
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

impl std::ops::Index<usize> for Bvec2 {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 2);

        match index {
            0 => &self.0,
            1 => &self.1,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Index<usize> for Bvec3 {
    type Output = bool;

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

impl std::ops::Index<usize> for Bvec4 {
    type Output = bool;

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

impl std::ops::IndexMut<usize> for Bvec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 2);

        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => unreachable!(),
        }
    }
}

impl std::ops::IndexMut<usize> for Bvec3 {
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

impl std::ops::IndexMut<usize> for Bvec4 {
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

impl std::ops::BitAnd<bool> for Bvec2 {
    type Output = Self;

    fn bitand(self, rhs: bool) -> Self::Output {
        Self(self.0 & rhs, self.1 & rhs)
    }
}

impl std::ops::BitAnd<bool> for Bvec3 {
    type Output = Self;

    fn bitand(self, rhs: bool) -> Self::Output {
        Self(self.0 & rhs, self.1 & rhs, self.2 & rhs)
    }
}

impl std::ops::BitAnd<bool> for Bvec4 {
    type Output = Self;

    fn bitand(self, rhs: bool) -> Self::Output {
        Self(self.0 & rhs, self.1 & rhs, self.2 & rhs, self.3 & rhs)
    }
}

impl std::ops::BitAnd<Bvec2> for bool {
    type Output = Bvec2;

    fn bitand(self, rhs: Bvec2) -> Self::Output {
        Bvec2(self & rhs.0, self & rhs.0)
    }
}

impl std::ops::BitAnd<Bvec3> for bool {
    type Output = Bvec3;

    fn bitand(self, rhs: Bvec3) -> Self::Output {
        Bvec3(self & rhs.0, self & rhs.1, self & rhs.2)
    }
}

impl std::ops::BitAnd<Bvec4> for bool {
    type Output = Bvec4;

    fn bitand(self, rhs: Bvec4) -> Self::Output {
        Bvec4(self & rhs.0, self & rhs.1, self & rhs.2, self & rhs.3)
    }
}

impl std::ops::BitAnd<Bvec2> for Bvec2 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0, self.1 & rhs.1)
    }
}

impl std::ops::BitAnd<Bvec3> for Bvec3 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0, self.1 & rhs.1, self.2 & rhs.2)
    }
}

impl std::ops::BitAnd<Self> for Bvec4 {
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

impl std::ops::BitAndAssign<bool> for Bvec2 {
    fn bitand_assign(&mut self, rhs: bool) {
        *self = *self & rhs;
    }
}

impl std::ops::BitAndAssign<bool> for Bvec3 {
    fn bitand_assign(&mut self, rhs: bool) {
        *self = *self & rhs;
    }
}

impl std::ops::BitAndAssign<bool> for Bvec4 {
    fn bitand_assign(&mut self, rhs: bool) {
        *self = *self & rhs;
    }
}

impl std::ops::BitAndAssign<Bvec2> for Bvec2 {
    fn bitand_assign(&mut self, rhs: Bvec2) {
        *self = *self & rhs;
    }
}

impl std::ops::BitAndAssign<Bvec3> for Bvec3 {
    fn bitand_assign(&mut self, rhs: Bvec3) {
        *self = *self & rhs;
    }
}

impl std::ops::BitAndAssign<Bvec4> for Bvec4 {
    fn bitand_assign(&mut self, rhs: Bvec4) {
        *self = *self & rhs;
    }
}

impl std::ops::BitOr<bool> for Bvec2 {
    type Output = Self;

    fn bitor(self, rhs: bool) -> Self::Output {
        Self(self.0 | rhs, self.1 | rhs)
    }
}

impl std::ops::BitOr<bool> for Bvec3 {
    type Output = Self;

    fn bitor(self, rhs: bool) -> Self::Output {
        Self(self.0 | rhs, self.1 | rhs, self.2 | rhs)
    }
}

impl std::ops::BitOr<bool> for Bvec4 {
    type Output = Self;

    fn bitor(self, rhs: bool) -> Self::Output {
        Self(self.0 | rhs, self.1 | rhs, self.2 | rhs, self.3 | rhs)
    }
}

impl std::ops::BitOr<Bvec2> for bool {
    type Output = Bvec2;

    fn bitor(self, rhs: Bvec2) -> Self::Output {
        Bvec2(self | rhs.0, self | rhs.1)
    }
}

impl std::ops::BitOr<Bvec3> for bool {
    type Output = Bvec3;

    fn bitor(self, rhs: Bvec3) -> Self::Output {
        Bvec3(self | rhs.0, self | rhs.1, self | rhs.2)
    }
}

impl std::ops::BitOr<Bvec4> for bool {
    type Output = Bvec4;

    fn bitor(self, rhs: Bvec4) -> Self::Output {
        Bvec4(self | rhs.0, self | rhs.1, self | rhs.2, self | rhs.3)
    }
}

impl std::ops::BitOr<Bvec2> for Bvec2 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0, self.1 | rhs.1)
    }
}

impl std::ops::BitOr<Bvec3> for Bvec3 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0, self.1 | rhs.1, self.2 | rhs.2)
    }
}

impl std::ops::BitOr<Self> for Bvec4 {
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

impl std::ops::BitOrAssign<bool> for Bvec2 {
    fn bitor_assign(&mut self, rhs: bool) {
        *self = *self | rhs;
    }
}

impl std::ops::BitOrAssign<bool> for Bvec3 {
    fn bitor_assign(&mut self, rhs: bool) {
        *self = *self | rhs;
    }
}

impl std::ops::BitOrAssign<bool> for Bvec4 {
    fn bitor_assign(&mut self, rhs: bool) {
        *self = *self | rhs;
    }
}

impl std::ops::BitOrAssign<Bvec2> for Bvec2 {
    fn bitor_assign(&mut self, rhs: Bvec2) {
        *self = *self | rhs;
    }
}

impl std::ops::BitOrAssign<Bvec3> for Bvec3 {
    fn bitor_assign(&mut self, rhs: Bvec3) {
        *self = *self | rhs;
    }
}

impl std::ops::BitOrAssign<Bvec4> for Bvec4 {
    fn bitor_assign(&mut self, rhs: Bvec4) {
        *self = *self | rhs;
    }
}

impl std::ops::BitXor<bool> for Bvec2 {
    type Output = Self;

    fn bitxor(self, rhs: bool) -> Self::Output {
        Self(self.0 ^ rhs, self.1 ^ rhs)
    }
}

impl std::ops::BitXor<bool> for Bvec3 {
    type Output = Self;

    fn bitxor(self, rhs: bool) -> Self::Output {
        Self(self.0 ^ rhs, self.1 ^ rhs, self.2 ^ rhs)
    }
}

impl std::ops::BitXor<bool> for Bvec4 {
    type Output = Self;

    fn bitxor(self, rhs: bool) -> Self::Output {
        Self(self.0 ^ rhs, self.1 ^ rhs, self.2 ^ rhs, self.3 ^ rhs)
    }
}

impl std::ops::BitXor<Bvec2> for bool {
    type Output = Bvec2;

    fn bitxor(self, rhs: Bvec2) -> Self::Output {
        Bvec2(self ^ rhs.0, self ^ rhs.1)
    }
}

impl std::ops::BitXor<Bvec3> for bool {
    type Output = Bvec3;

    fn bitxor(self, rhs: Bvec3) -> Self::Output {
        Bvec3(self ^ rhs.0, self ^ rhs.1, self ^ rhs.2)
    }
}

impl std::ops::BitXor<Bvec4> for bool {
    type Output = Bvec4;

    fn bitxor(self, rhs: Bvec4) -> Self::Output {
        Bvec4(self ^ rhs.0, self ^ rhs.1, self ^ rhs.2, self ^ rhs.3)
    }
}

impl std::ops::BitXor<Bvec2> for Bvec2 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0, self.1 ^ rhs.1)
    }
}

impl std::ops::BitXor<Bvec3> for Bvec3 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0, self.1 ^ rhs.1, self.2 ^ rhs.2)
    }
}

impl std::ops::BitXor<Bvec4> for Bvec4 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(
            self.0 ^ rhs.0,
            self.1 ^ rhs.1,
            self.2 ^ rhs.2,
            self.3 ^ rhs.3,
        )
    }
}

impl std::ops::BitXorAssign<bool> for Bvec2 {
    fn bitxor_assign(&mut self, rhs: bool) {
        *self = *self ^ rhs;
    }
}

impl std::ops::BitXorAssign<bool> for Bvec3 {
    fn bitxor_assign(&mut self, rhs: bool) {
        *self = *self ^ rhs;
    }
}

impl std::ops::BitXorAssign<bool> for Bvec4 {
    fn bitxor_assign(&mut self, rhs: bool) {
        *self = *self ^ rhs;
    }
}

impl std::ops::BitXorAssign<Bvec2> for Bvec2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl std::ops::BitXorAssign<Bvec3> for Bvec3 {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl std::ops::BitXorAssign<Bvec4> for Bvec4 {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
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

impl std::ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

impl std::ops::Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

impl std::ops::Not for Bvec2 {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0, !self.1)
    }
}

impl std::ops::Not for Bvec3 {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0, !self.1, !self.2)
    }
}

impl std::ops::Not for Bvec4 {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0, !self.1, !self.2, !self.3)
    }
}

impl std::ops::Rem<f32> for Vec2 {
    type Output = Self;

    fn rem(self, rhs: f32) -> Self::Output {
        Self(self.0 % rhs, self.1 % rhs)
    }
}

impl std::ops::Rem<f32> for Vec3 {
    type Output = Self;

    fn rem(self, rhs: f32) -> Self::Output {
        Self(self.0 % rhs, self.1 % rhs, self.2 % rhs)
    }
}

impl std::ops::Rem<f32> for Vec4 {
    type Output = Self;

    fn rem(self, rhs: f32) -> Self::Output {
        Self(self.0 % rhs, self.1 % rhs, self.2 % rhs, self.3 % rhs)
    }
}

impl std::ops::Rem<Vec2> for f32 {
    type Output = Vec2;

    fn rem(self, rhs: Vec2) -> Self::Output {
        Vec2(self % rhs.0, self % rhs.1)
    }
}

impl std::ops::Rem<Vec3> for f32 {
    type Output = Vec3;

    fn rem(self, rhs: Vec3) -> Self::Output {
        Vec3(self % rhs.0, self % rhs.1, self % rhs.2)
    }
}

impl std::ops::Rem<Vec4> for f32 {
    type Output = Vec4;

    fn rem(self, rhs: Vec4) -> Self::Output {
        Vec4(self % rhs.0, self % rhs.1, self % rhs.2, self % rhs.3)
    }
}

impl std::ops::Rem<Vec2> for Vec2 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0, self.1 % rhs.1)
    }
}

impl std::ops::Rem<Vec3> for Vec3 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0, self.1 % rhs.1, self.2 % rhs.2)
    }
}

impl std::ops::Rem<Vec4> for Vec4 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(
            self.0 % rhs.0,
            self.1 % rhs.1,
            self.2 % rhs.2,
            self.3 % rhs.3,
        )
    }
}

impl std::ops::RemAssign<f32> for Vec2 {
    fn rem_assign(&mut self, rhs: f32) {
        *self = *self % rhs;
    }
}

impl std::ops::RemAssign<f32> for Vec3 {
    fn rem_assign(&mut self, rhs: f32) {
        *self = *self % rhs;
    }
}

impl std::ops::RemAssign<f32> for Vec4 {
    fn rem_assign(&mut self, rhs: f32) {
        *self = *self % rhs;
    }
}

impl std::ops::RemAssign<Vec2> for Vec2 {
    fn rem_assign(&mut self, rhs: Vec2) {
        *self = *self % rhs;
    }
}

impl std::ops::RemAssign<Vec3> for Vec3 {
    fn rem_assign(&mut self, rhs: Vec3) {
        *self = *self % rhs;
    }
}

impl std::ops::RemAssign<Vec4> for Vec4 {
    fn rem_assign(&mut self, rhs: Vec4) {
        *self = *self % rhs;
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

//
// common functions
//

impl Vec2 {
    pub fn radians(self) -> Self {
        Self(self.0.to_radians(), self.1.to_radians())
    }

    pub fn degrees(self) -> Self {
        Self(self.0.to_degrees(), self.1.to_degrees())
    }

    pub fn abs(self) -> Self {
        Self(self.0.abs(), self.1.abs())
    }

    pub fn sign(self) -> Self {
        Self(f32::signum(self.0), f32::signum(self.1))
    }

    pub fn floor(self) -> Self {
        Self(f32::floor(self.0), f32::floor(self.1))
    }

    pub fn ceil(self) -> Self {
        Self(f32::ceil(self.0), f32::ceil(self.1))
    }

    pub fn trunc(self) -> Self {
        Self(f32::trunc(self.0), f32::trunc(self.1))
    }

    pub fn round(self) -> Self {
        Self(f32::round(self.0), f32::round(self.1))
    }

    pub fn fract(self) -> Self {
        Self(f32::fract(self.0), f32::fract(self.1))
    }

    pub fn min(x: Self, y: Self) -> Self {
        Self(f32::min(x.0, y.0), f32::min(x.1, y.1))
    }

    pub fn max(x: Self, y: Self) -> Self {
        Self(f32::max(x.0, y.0), f32::max(x.1, y.1))
    }

    pub fn clamp(self, min_val: Self, max_val: Self) -> Self {
        Self(
            f32::clamp(self.0, min_val.0, max_val.0),
            f32::clamp(self.1, min_val.1, max_val.1),
        )
    }

    pub fn mix(x: Self, y: Self, a: Self) -> Self {
        Self(crate::mix(x.0, y.0, a.0), crate::mix(x.1, y.1, a.1))
    }

    pub fn step(edge: Self, x: Self) -> Self {
        Self(crate::step(edge.0, x.0), crate::step(edge.1, x.1))
    }

    pub fn smoothstep(edge0: Self, edge1: Self, x: Self) -> Self {
        Self(
            crate::smoothstep(edge0.0, edge1.0, x.0),
            crate::smoothstep(edge0.1, edge1.1, x.1),
        )
    }

    pub fn smootherstep(edge0: Self, edge1: Self, x: Self) -> Self {
        Self(
            crate::smootherstep(edge0.0, edge1.0, x.0),
            crate::smootherstep(edge0.1, edge1.1, x.1),
        )
    }

    pub fn is_nan(self) -> Bvec2 {
        Bvec2(f32::is_nan(self.0), f32::is_nan(self.1))
    }

    pub fn is_infinite(self) -> Bvec2 {
        Bvec2(f32::is_infinite(self.0), f32::is_infinite(self.1))
    }
}

impl Vec3 {
    pub fn radians(self) -> Self {
        Self(
            self.0.to_radians(),
            self.1.to_radians(),
            self.2.to_radians(),
        )
    }

    pub fn degrees(self) -> Self {
        Self(
            self.0.to_degrees(),
            self.1.to_degrees(),
            self.2.to_degrees(),
        )
    }

    pub fn abs(self) -> Self {
        Self(self.0.abs(), self.1.abs(), self.2.abs())
    }

    pub fn sign(self) -> Self {
        Self(
            f32::signum(self.0),
            f32::signum(self.1),
            f32::signum(self.2),
        )
    }

    pub fn floor(self) -> Self {
        Self(f32::floor(self.0), f32::floor(self.1), f32::floor(self.2))
    }

    pub fn ceil(self) -> Self {
        Self(f32::ceil(self.0), f32::ceil(self.1), f32::ceil(self.2))
    }

    pub fn trunc(self) -> Self {
        Self(f32::trunc(self.0), f32::trunc(self.1), f32::trunc(self.2))
    }

    pub fn round(self) -> Self {
        Self(f32::round(self.0), f32::round(self.1), f32::round(self.2))
    }

    pub fn fract(self) -> Self {
        Self(f32::fract(self.0), f32::fract(self.1), f32::fract(self.2))
    }

    pub fn min(x: Self, y: Self) -> Self {
        Self(f32::min(x.0, y.0), f32::min(x.1, y.1), f32::min(x.2, y.2))
    }

    pub fn max(x: Self, y: Self) -> Self {
        Self(f32::max(x.0, y.0), f32::max(x.1, y.1), f32::max(x.2, y.2))
    }

    pub fn clamp(self, min_val: Self, max_val: Self) -> Self {
        Self(
            f32::clamp(self.0, min_val.0, max_val.0),
            f32::clamp(self.1, min_val.1, max_val.1),
            f32::clamp(self.2, min_val.2, max_val.2),
        )
    }

    pub fn mix(x: Self, y: Self, a: Self) -> Self {
        Self(
            crate::mix(x.0, y.0, a.0),
            crate::mix(x.1, y.1, a.1),
            crate::mix(x.2, y.2, a.2),
        )
    }

    pub fn step(edge: Self, x: Self) -> Self {
        Self(
            crate::step(edge.0, x.0),
            crate::step(edge.1, x.1),
            crate::step(edge.2, x.2),
        )
    }

    pub fn smoothstep(edge0: Self, edge1: Self, x: Self) -> Self {
        Self(
            crate::smoothstep(edge0.0, edge1.0, x.0),
            crate::smoothstep(edge0.1, edge1.1, x.1),
            crate::smoothstep(edge0.2, edge1.2, x.2),
        )
    }

    pub fn smootherstep(edge0: Self, edge1: Self, x: Self) -> Self {
        Self(
            crate::smootherstep(edge0.0, edge1.0, x.0),
            crate::smootherstep(edge0.1, edge1.1, x.1),
            crate::smootherstep(edge0.2, edge1.2, x.2),
        )
    }

    pub fn is_nan(self) -> Bvec3 {
        Bvec3(
            f32::is_nan(self.0),
            f32::is_nan(self.1),
            f32::is_nan(self.2),
        )
    }

    pub fn is_infinite(self) -> Bvec3 {
        Bvec3(
            f32::is_infinite(self.0),
            f32::is_infinite(self.1),
            f32::is_infinite(self.2),
        )
    }
}

impl Vec4 {
    pub fn radians(self) -> Self {
        Self(
            self.0.to_radians(),
            self.1.to_radians(),
            self.2.to_radians(),
            self.3.to_radians(),
        )
    }

    pub fn degrees(self) -> Self {
        Self(
            self.0.to_degrees(),
            self.1.to_degrees(),
            self.2.to_degrees(),
            self.3.to_degrees(),
        )
    }

    pub fn abs(self) -> Self {
        Self(self.0.abs(), self.1.abs(), self.2.abs(), self.3.abs())
    }

    pub fn sign(self) -> Self {
        Self(
            f32::signum(self.0),
            f32::signum(self.1),
            f32::signum(self.2),
            f32::signum(self.3),
        )
    }

    pub fn floor(self) -> Self {
        Self(
            f32::floor(self.0),
            f32::floor(self.1),
            f32::floor(self.2),
            f32::floor(self.3),
        )
    }

    pub fn ceil(self) -> Self {
        Self(
            f32::ceil(self.0),
            f32::ceil(self.1),
            f32::ceil(self.2),
            f32::ceil(self.3),
        )
    }

    pub fn trunc(self) -> Self {
        Self(
            f32::trunc(self.0),
            f32::trunc(self.1),
            f32::trunc(self.2),
            f32::trunc(self.3),
        )
    }

    pub fn round(self) -> Self {
        Self(
            f32::round(self.0),
            f32::round(self.1),
            f32::round(self.2),
            f32::round(self.3),
        )
    }

    pub fn fract(self) -> Self {
        Self(
            f32::fract(self.0),
            f32::fract(self.1),
            f32::fract(self.2),
            f32::fract(self.3),
        )
    }

    pub fn min(x: Self, y: Self) -> Self {
        Self(
            f32::min(x.0, y.0),
            f32::min(x.1, y.1),
            f32::min(x.2, y.2),
            f32::min(x.3, y.3),
        )
    }

    pub fn max(x: Self, y: Self) -> Self {
        Self(
            f32::max(x.0, y.0),
            f32::max(x.1, y.1),
            f32::max(x.2, y.2),
            f32::max(x.3, y.3),
        )
    }

    pub fn clamp(self, min_val: Self, max_val: Self) -> Self {
        Self(
            f32::clamp(self.0, min_val.0, max_val.0),
            f32::clamp(self.1, min_val.1, max_val.1),
            f32::clamp(self.2, min_val.2, max_val.2),
            f32::clamp(self.3, min_val.3, max_val.3),
        )
    }

    pub fn mix(x: Self, y: Self, a: Self) -> Self {
        Self(
            crate::mix(x.0, y.0, a.0),
            crate::mix(x.1, y.1, a.1),
            crate::mix(x.2, y.2, a.2),
            crate::mix(x.3, y.3, a.3),
        )
    }

    pub fn step(edge: Self, x: Self) -> Self {
        Self(
            crate::step(edge.0, x.0),
            crate::step(edge.1, x.1),
            crate::step(edge.2, x.2),
            crate::step(edge.3, x.3),
        )
    }

    pub fn smoothstep(edge0: Self, edge1: Self, x: Self) -> Self {
        Self(
            crate::smoothstep(edge0.0, edge1.0, x.0),
            crate::smoothstep(edge0.1, edge1.1, x.1),
            crate::smoothstep(edge0.2, edge1.2, x.2),
            crate::smoothstep(edge0.3, edge1.3, x.3),
        )
    }

    pub fn smootherstep(edge0: Self, edge1: Self, x: Self) -> Self {
        Self(
            crate::smootherstep(edge0.0, edge1.0, x.0),
            crate::smootherstep(edge0.1, edge1.1, x.1),
            crate::smootherstep(edge0.2, edge1.2, x.2),
            crate::smootherstep(edge0.3, edge1.3, x.3),
        )
    }

    pub fn is_nan(self) -> Bvec4 {
        Bvec4(
            f32::is_nan(self.0),
            f32::is_nan(self.1),
            f32::is_nan(self.2),
            f32::is_nan(self.3),
        )
    }

    pub fn is_infinite(self) -> Bvec4 {
        Bvec4(
            f32::is_infinite(self.0),
            f32::is_infinite(self.1),
            f32::is_infinite(self.2),
            f32::is_infinite(self.3),
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
        f32::sqrt(self.length_squared())
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
            eta * i - (eta * n.dot(i) + f32::sqrt(k)) * n
        }
    }
}

impl Vec3 {
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        f32::sqrt(self.length_squared())
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
            eta * i - (eta * n.dot(i) + f32::sqrt(k)) * n
        }
    }
}

impl Vec4 {
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        f32::sqrt(self.length_squared())
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
            eta * i - (eta * n.dot(i) + f32::sqrt(k)) * n
        }
    }
}

//
// relational functions
//

impl Vec2 {
    pub fn less_than(self, rhs: Self) -> Bvec2 {
        Bvec2(self.0 < rhs.0, self.1 < rhs.1)
    }

    pub fn less_than_equal(self, rhs: Self) -> Bvec2 {
        Bvec2(self.0 <= rhs.0, self.1 <= rhs.1)
    }

    pub fn greater_than(self, rhs: Self) -> Bvec2 {
        Bvec2(self.0 > rhs.0, self.1 > rhs.1)
    }

    pub fn greater_than_equal(self, rhs: Self) -> Bvec2 {
        Bvec2(self.0 >= rhs.0, self.1 >= rhs.1)
    }

    pub fn equal(self, rhs: Self) -> Bvec2 {
        Bvec2(self.0 == rhs.0, self.1 == rhs.1)
    }

    pub fn fequal(self, rhs: Self, tolerance: f32) -> Bvec2 {
        Bvec2(
            f32::abs(self.0 - rhs.0) < tolerance,
            f32::abs(self.1 - rhs.1) < tolerance,
        )
    }

    pub fn not_equal(self, rhs: Self) -> Bvec2 {
        Bvec2(self.0 != rhs.0, self.1 != rhs.1)
    }

    pub fn not_fequal(self, rhs: Self, tolerance: f32) -> Bvec2 {
        Bvec2(
            f32::abs(self.0 - rhs.0) >= tolerance,
            f32::abs(self.1 - rhs.1) >= tolerance,
        )
    }
}

impl Vec3 {
    pub fn less_than(self, rhs: Self) -> Bvec3 {
        Bvec3(self.0 < rhs.0, self.1 < rhs.1, self.2 < rhs.2)
    }

    pub fn less_than_equal(self, rhs: Self) -> Bvec3 {
        Bvec3(self.0 <= rhs.0, self.1 <= rhs.1, self.2 <= rhs.2)
    }

    pub fn greater_than(self, rhs: Self) -> Bvec3 {
        Bvec3(self.0 > rhs.0, self.1 > rhs.1, self.2 > rhs.2)
    }

    pub fn greater_than_equal(self, rhs: Self) -> Bvec3 {
        Bvec3(self.0 >= rhs.0, self.1 >= rhs.1, self.2 >= rhs.2)
    }

    pub fn equal(self, rhs: Self) -> Bvec3 {
        Bvec3(self.0 == rhs.0, self.1 == rhs.1, self.2 == rhs.2)
    }

    pub fn fequal(self, rhs: Self, tolerance: f32) -> Bvec3 {
        Bvec3(
            f32::abs(self.0 - rhs.0) < tolerance,
            f32::abs(self.1 - rhs.1) < tolerance,
            f32::abs(self.2 - rhs.2) < tolerance,
        )
    }

    pub fn not_equal(self, rhs: Self) -> Bvec3 {
        Bvec3(self.0 != rhs.0, self.1 != rhs.1, self.2 != rhs.2)
    }

    pub fn not_fequal(self, rhs: Self, tolerance: f32) -> Bvec3 {
        Bvec3(
            f32::abs(self.0 - rhs.0) >= tolerance,
            f32::abs(self.1 - rhs.1) >= tolerance,
            f32::abs(self.2 - rhs.2) >= tolerance,
        )
    }
}

impl Vec4 {
    pub fn less_than(self, rhs: Self) -> Bvec4 {
        Bvec4(
            self.0 < rhs.0,
            self.1 < rhs.1,
            self.2 < rhs.2,
            self.3 < rhs.3,
        )
    }

    pub fn less_than_equal(self, rhs: Self) -> Bvec4 {
        Bvec4(
            self.0 <= rhs.0,
            self.1 <= rhs.1,
            self.2 <= rhs.2,
            self.3 <= rhs.3,
        )
    }

    pub fn greater_than(self, rhs: Self) -> Bvec4 {
        Bvec4(
            self.0 > rhs.0,
            self.1 > rhs.1,
            self.2 > rhs.2,
            self.3 > rhs.3,
        )
    }

    pub fn greater_than_equal(self, rhs: Self) -> Bvec4 {
        Bvec4(
            self.0 >= rhs.0,
            self.1 >= rhs.1,
            self.2 >= rhs.2,
            self.3 >= rhs.3,
        )
    }

    pub fn equal(self, rhs: Self) -> Bvec4 {
        Bvec4(
            self.0 == rhs.0,
            self.1 == rhs.1,
            self.2 == rhs.2,
            self.3 == rhs.3,
        )
    }

    pub fn fequal(self, rhs: Self, tolerance: f32) -> Bvec4 {
        Bvec4(
            f32::abs(self.0 - rhs.0) < tolerance,
            f32::abs(self.1 - rhs.1) < tolerance,
            f32::abs(self.2 - rhs.2) < tolerance,
            f32::abs(self.3 - rhs.3) < tolerance,
        )
    }

    pub fn not_equal(self, rhs: Self) -> Bvec4 {
        Bvec4(
            self.0 != rhs.0,
            self.1 != rhs.1,
            self.2 != rhs.2,
            self.3 != rhs.3,
        )
    }

    pub fn not_fequal(self, rhs: Self, tolerance: f32) -> Bvec4 {
        Bvec4(
            f32::abs(self.0 - rhs.0) >= tolerance,
            f32::abs(self.1 - rhs.1) >= tolerance,
            f32::abs(self.2 - rhs.2) >= tolerance,
            f32::abs(self.3 - rhs.3) >= tolerance,
        )
    }
}

impl std::cmp::PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.equal(*other).all()
    }
}

impl std::cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.equal(*other).all()
    }
}

impl std::cmp::PartialEq for Vec4 {
    fn eq(&self, other: &Self) -> bool {
        self.equal(*other).all()
    }
}

impl std::cmp::Eq for Vec2 {}
impl std::cmp::Eq for Vec3 {}
impl std::cmp::Eq for Vec4 {}

impl Bvec2 {
    pub fn any(self) -> bool {
        self.0 || self.1
    }

    pub fn all(self) -> bool {
        self.0 && self.1
    }
}

impl Bvec3 {
    pub fn any(self) -> bool {
        self.0 || self.1 || self.2
    }

    pub fn all(self) -> bool {
        self.0 && self.1 && self.2
    }
}

impl Bvec4 {
    pub fn any(self) -> bool {
        self.0 || self.1 || self.2 || self.3
    }

    pub fn all(self) -> bool {
        self.0 && self.1 && self.2 && self.3
    }
}
