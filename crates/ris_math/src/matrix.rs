use vulkano::buffer::BufferContents;

use crate::vector::Vec2;
use crate::vector::Vec3;
use crate::vector::Vec4;

// definition

// the first number in the type is the number of columns, the second is the number of rows. if
// there is only one number, the matrix is square.

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Mat2x2(pub Vec2, pub Vec2);

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Mat2x3(pub Vec3, pub Vec3);

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Mat2x4(pub Vec4, pub Vec4);

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Mat3x2(pub Vec2, pub Vec2, pub Vec2);

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Mat3x3(pub Vec3, pub Vec3, pub Vec3);

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Mat3x4(pub Vec4, pub Vec4, pub Vec4);

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Mat4x2(pub Vec2, pub Vec2, pub Vec2, pub Vec2);

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Mat4x3(pub Vec3, pub Vec3, pub Vec3, pub Vec3);

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Mat4x4(pub Vec4, pub Vec4, pub Vec4, pub Vec4);

pub type Mat2 = Mat2x2;
pub type Mat3 = Mat3x3;
pub type Mat4 = Mat4x4;

// constructors
impl Mat2 {
    pub fn init(value: f32) -> Self {
        Self(Vec2(value, 0),Vec2(0, value))
    }
}

impl Mat3 {
    pub fn init(value: f32) -> Self {
        Self(Vec3(value, 0, 0), Vec3(0, value, 0), Vec3(0, 0, value))
    }
}

impl Mat4 {
    pub fn init(value: f32) -> Self {
        Self(Vec4(value, 0, 0, 0),Vec4(0, value, 0, 0),Vec4(0, 0, value, 0),Vec4(0, 0, 0, value))
    }
}

// components
impl std::ops::Index<usize> for Mat2x2 {
    type Output = Vec2;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 2);

        match index {
            0 => &self.0,
            1 => &self.1,
            _ => unreachable!(),
        }
    }
}

impl std::ops::IndexMut<usize> for Mat2x2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 2);

        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Index<usize> for Mat2x3 {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 2);

        match index {
            0 => &self.0,
            1 => &self.1,
            _ => unreachable!(),
        }
    }
}

impl std::ops::IndexMut<usize> for Mat2x3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 2);

        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Index<usize> for Mat2x4 {
    type Output = Vec4;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 2);

        match index {
            0 => &self.0,
            1 => &self.1,
            _ => unreachable!(),
        }
    }
}

impl std::ops::IndexMut<usize> for Mat2x4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 2);

        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Index<usize> for Mat3x2 {
    type Output = Vec2;

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

impl std::ops::IndexMut<usize> for Mat3x2 {
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

impl std::ops::Index<usize> for Mat3x3 {
    type Output = Vec3;

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

impl std::ops::IndexMut<usize> for Mat3x3 {
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

impl std::ops::Index<usize> for Mat3x4 {
    type Output = Vec4;

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

impl std::ops::IndexMut<usize> for Mat3x4 {
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

impl std::ops::Index<usize> for Mat4x2 {
    type Output = Vec2;

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

impl std::ops::IndexMut<usize> for Mat4x2 {
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

impl std::ops::Index<usize> for Mat4x3 {
    type Output = Vec3;

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

impl std::ops::IndexMut<usize> for Mat4x3 {
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

impl std::ops::Index<usize> for Mat4x4 {
    type Output = Vec4;

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

impl std::ops::IndexMut<usize> for Mat4x4 {
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
impl std::ops::Mul<Mat3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Mat3) -> Self::Output {
        let v = self;
        let mut u = Vec3::default();
        let m = rhs;

        u.0 = Vec3::dot(v, m.0);
        u.1 = Vec3::dot(v, m.1);
        u.2 = Vec3::dot(v, m.2);

        u
    }
}

impl std::ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let v = rhs;
        let mut u = Vec3::default();
        let m = self;

        u.0 = m.0.x() * v.x() + m.1.x() * v.y() + m.2.x() * v.z();
        u.1 = m.0.y() * v.x() + m.1.y() * v.y() + m.2.y() * v.z();
        u.2 = m.0.z() * v.x() + m.1.z() * v.y() + m.2.z() * v.z();

        u
    }
}

impl std::ops::Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Self::Output {
        let m = self;
        let n = rhs;
        let mut r = Mat3::default();

        r.0.x = m.0.x() * n.0.x() + m.1.x() * n.0.y() + m.2.x() * n.0.z();
        r.1.x = m.0.x() * n.1.x() + m.1.x() * n.1.y() + m.2.x() * n.1.z();
        r.2.x = m.0.x() * n.2.x() + m.1.x() * n.2.y() + m.2.x() * n.2.z();

        r.0.y = m.0.y() * n.0.x() + m.1.y() * n.0.y() + m.2.y() * n.0.z();
        r.1.y = m.0.y() * n.1.x() + m.1.y() * n.1.y() + m.2.y() * n.1.z();
        r.2.y = m.0.y() * n.2.x() + m.1.y() * n.2.y() + m.2.y() * n.2.z();

        r.0.z = m.0.z() * n.0.x() + m.1.z() * n.0.y() + m.2.z() * n.0.z();
        r.1.z = m.0.z() * n.1.x() + m.1.z() * n.1.y() + m.2.z() * n.1.z();
        r.2.z = m.0.z() * n.2.x() + m.1.z() * n.2.y() + m.2.z() * n.2.z();

        r
    }
}














// 

// m00 m10 m20 m30
// m01 m11 m21 m31
// m02 m12 m22 m32
// m03 m13 m23 m33
#[derive(Debug, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Matrix4x4 {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m03: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m30: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
}

impl std::fmt::Display for Matrix4x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:>8} {:>8} {:>8} {:>8}", self.m00, self.m10, self.m20, self.m30)?;
        writeln!(f, "{:>8} {:>8} {:>8} {:>8}", self.m01, self.m11, self.m21, self.m31)?;
        writeln!(f, "{:>8} {:>8} {:>8} {:>8}", self.m02, self.m12, self.m22, self.m32)?;
        writeln!(f, "{:>8} {:>8} {:>8} {:>8}", self.m03, self.m13, self.m23, self.m33)
    }
}

impl Matrix4x4 {
    // constructors
    pub fn new(scalar: f32) -> Self {
        Self {
            m00: scalar,
            m01: 0.,
            m02: 0.,
            m03: 0.,
            m10: 0.,
            m11: scalar,
            m12: 0.,
            m13: 0.,
            m20: 0.,
            m21: 0.,
            m22: scalar,
            m23: 0.,
            m30: 0.,
            m31: 0.,
            m32: 0.,
            m33: scalar,
        }
    }

    pub fn translation(v: Vector3) -> Self {
        Self {
            m00: 1.,
            m01: 0.,
            m02: 0.,
            m03: v.x,
            m10: 0.,
            m11: 1.,
            m12: 0.,
            m13: v.y,
            m20: 0.,
            m21: 0.,
            m22: 1.,
            m23: v.z,
            m30: 0.,
            m31: 0.,
            m32: 0.,
            m33: 1.,
        }
    }

    pub fn rotation(q: Quaternion) -> Self {
        let sqw = q.w * q.w;
        let sqx = q.x * q.x;
        let sqy = q.y * q.y;
        let sqz = q.z * q.z;

        let m00 = sqx - sqy - sqz + sqw;
        let m11 = -sqx + sqy - sqz + sqw;
        let m22 = -sqx - sqy + sqz + sqw;

        let temp1 = q.x * q.y;
        let temp2 = q.z * q.w;
        let m10 = 2. * (temp1 + temp2);
        let m01 = 2. * (temp1 - temp2);

        let temp1 = q.x * q.z;
        let temp2 = q.y * q.w;
        let m20 = 2. * (temp1 - temp2);
        let m02 = 2. * (temp1 + temp2);

        let temp1 = q.y * q.z;
        let temp2 = q.x * q.w;
        let m21 = 2. * (temp1 + temp2);
        let m12 = 2. * (temp1 - temp2);

        Self {
            m00,
            m01,
            m02,
            m03: 0.,
            m10,
            m11,
            m12,
            m13: 0.,
            m20,
            m21,
            m22,
            m23: 0.,
            m30: 0.,
            m31: 0.,
            m32: 0.,
            m33: 1.,
        }
    }

    // standard matrix stuff
    pub fn transposed(self) -> Self {
        Self {
            m00: self.m00,
            m01: self.m10,
            m02: self.m20,
            m03: self.m30,
            m10: self.m01,
            m11: self.m11,
            m12: self.m21,
            m13: self.m31,
            m20: self.m02,
            m21: self.m12,
            m22: self.m22,
            m23: self.m32,
            m30: self.m03,
            m31: self.m13,
            m32: self.m23,
            m33: self.m33,
        }
    }

    pub fn multiply_vector4(m: Matrix4x4, v: Vector4) -> Vector4 {
        [
            m.m00 * v[0] + m.m01 * v[1] + m.m02 * v[2] + m.m03 * v[3],
            m.m10 * v[0] + m.m11 * v[1] + m.m12 * v[2] + m.m13 * v[3],
            m.m20 * v[0] + m.m21 * v[1] + m.m22 * v[2] + m.m23 * v[3],
            m.m30 * v[0] + m.m31 * v[1] + m.m32 * v[2] + m.m33 * v[3],
        ]
    }

    // 3d transformation stuff
    pub fn view(camera_position: Vector3, camera_roration: Quaternion) -> Self {
        // My coordinate system is x => right, y => forward and z => upward.
        // Vulkans coordinat system is x => right, y => down and z => forward.
        // Both are right handed coordinate systems, therefore all relationships are equal.
        // Only a single default rotation is necessary, to convert my system to vulkan.
        let default_rotation = Quaternion::from_angle_axis(super::PI_0_5, vector3::RIGHT);
        let camera_rotation = camera_roration.conjugate();
        let rotation = default_rotation * camera_rotation;
        let translation = camera_position.inverted();

        let translation_matrix = Matrix4x4::translation(translation);
        let rotation_matrix = Matrix4x4::rotation(rotation);

        rotation_matrix * translation_matrix
    }

    pub fn perspective_projection(fovy: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let focal_length = 1. / super::tan(fovy / 2.);
        let x = focal_length / aspect_ratio;
        let y = focal_length;
        let a = near / (far - near);
        let b = far * a;

        Self {
            m00: x,
            m01: 0.,
            m02: 0.,
            m03: 0.,
            m10: 0.,
            m11: y,
            m12: 0.,
            m13: 0.,
            m20: 0.,
            m21: 0.,
            m22: a,
            m23: b,
            m30: 0.,
            m31: 0.,
            m32: 1.,
            m33: 0.,
        }
    }

    pub fn rotate(self, v: Vector3) -> Vector3 {
        let v4 = [v.x, v.y, v.z, 0.];
        let result = Self::multiply_vector4(self, v4);
        Vector3 {
            x: result[0],
            y: result[1],
            z: result[2],
        }
    }

    pub fn rotate_and_transform(self, v: Vector3) -> Vector3 {
        let v4 = [v.x, v.y, v.z, 1.];
        let result = Self::multiply_vector4(self, v4);
        Vector3 {
            x: result[0],
            y: result[1],
            z: result[2],
        }
    }

    pub fn invert(self) -> Self {
        Self {
            m00: self.m00,
            m01: self.m10,
            m02: self.m20,
            m03: -self.m03,
            m10: self.m01,
            m11: self.m11,
            m12: self.m21,
            m13: -self.m13,
            m20: self.m02,
            m21: self.m12,
            m22: self.m22,
            m23: -self.m23,
            m30: self.m30,
            m31: self.m31,
            m32: self.m32,
            m33: self.m33,
        }
    }
}

impl std::ops::Index<usize> for Matrix4x4 {
    type Output = Column4;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 4);

        match index {
            0 => &Column4(self.m00, self.m01, self.m02, self.m03),
            1 => &Column4(self.m10, self.m11, self.m12, self.m13),
            2 => &Column4(self.m20, self.m21, self.m22, self.m23),
            3 => &Column4(self.m30, self.m31, self.m32, self.m33),
            _ => unreachable!(),
        }
    }
}

impl std::ops::Index<usize> for Column4 {
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

impl std::ops::Mul<Matrix4x4> for Matrix4x4 {
    type Output = Self;
    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        let b = self;
        let a = rhs;
        Self {
            m00: a.m00 * b.m00 + a.m01 * b.m10 + a.m02 * b.m20 + a.m03 * b.m30,
            m01: a.m00 * b.m01 + a.m01 * b.m11 + a.m02 * b.m21 + a.m03 * b.m31,
            m02: a.m00 * b.m02 + a.m01 * b.m12 + a.m02 * b.m22 + a.m03 * b.m32,
            m03: a.m00 * b.m03 + a.m01 * b.m13 + a.m02 * b.m23 + a.m03 * b.m33,
            m10: a.m10 * b.m00 + a.m11 * b.m10 + a.m12 * b.m20 + a.m13 * b.m30,
            m11: a.m10 * b.m01 + a.m11 * b.m11 + a.m12 * b.m21 + a.m13 * b.m31,
            m12: a.m10 * b.m02 + a.m11 * b.m12 + a.m12 * b.m22 + a.m13 * b.m32,
            m13: a.m10 * b.m03 + a.m11 * b.m13 + a.m12 * b.m23 + a.m13 * b.m33,
            m20: a.m20 * b.m00 + a.m21 * b.m10 + a.m22 * b.m20 + a.m23 * b.m30,
            m21: a.m20 * b.m01 + a.m21 * b.m11 + a.m22 * b.m21 + a.m23 * b.m31,
            m22: a.m20 * b.m02 + a.m21 * b.m12 + a.m22 * b.m22 + a.m23 * b.m32,
            m23: a.m20 * b.m03 + a.m21 * b.m13 + a.m22 * b.m23 + a.m23 * b.m33,
            m30: a.m30 * b.m00 + a.m31 * b.m10 + a.m32 * b.m20 + a.m33 * b.m30,
            m31: a.m30 * b.m01 + a.m31 * b.m11 + a.m32 * b.m21 + a.m33 * b.m31,
            m32: a.m30 * b.m02 + a.m31 * b.m12 + a.m32 * b.m22 + a.m33 * b.m32,
            m33: a.m30 * b.m03 + a.m31 * b.m13 + a.m32 * b.m23 + a.m33 * b.m33,
        }
    }
}
