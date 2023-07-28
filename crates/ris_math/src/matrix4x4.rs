use vulkano::buffer::BufferContents;

use crate::quaternion::Quaternion;
use crate::vector3;
use crate::vector3::Vector3;

// m00 m01 m02 m03
// m10 m11 m12 m13
// m20 m21 m22 m23
// m30 m31 m32 m33
#[derive(Debug, Copy, Clone, Default, BufferContents)]
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

type Vector4 = [f32; 4];

impl Matrix4x4 {
    // standard matrix stuff
    pub fn identity() -> Self {
        Self {
            m00: 1.,
            m01: 0.,
            m02: 0.,
            m03: 0.,
            m10: 0.,
            m11: 1.,
            m12: 0.,
            m13: 0.,
            m20: 0.,
            m21: 0.,
            m22: 1.,
            m23: 0.,
            m30: 0.,
            m31: 0.,
            m32: 0.,
            m33: 1.,
        }
    }

    pub fn get(self, m: usize, n: usize) -> f32 {
        assert!(m < 4);
        assert!(n < 4);

        match (m, n) {
            (0, 0) => self.m00,
            (0, 1) => self.m01,
            (0, 2) => self.m02,
            (0, 3) => self.m03,
            (1, 0) => self.m10,
            (1, 1) => self.m11,
            (1, 2) => self.m12,
            (1, 3) => self.m13,
            (2, 0) => self.m20,
            (2, 1) => self.m21,
            (2, 2) => self.m22,
            (2, 3) => self.m23,
            (3, 0) => self.m30,
            (3, 1) => self.m31,
            (3, 2) => self.m32,
            (3, 3) => self.m33,
            _ => unreachable!(),
        }
    }

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

    pub fn multiply(a: Self, b: Self) -> Self {
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

    pub fn multiply_vector4(m: Matrix4x4, v: Vector4) -> Vector4 {
        [
            m.m00 * v[0] + m.m01 * v[1] + m.m02 * v[2] + m.m03 * v[3],
            m.m10 * v[0] + m.m11 * v[1] + m.m12 * v[2] + m.m13 * v[3],
            m.m20 * v[0] + m.m21 * v[1] + m.m22 * v[2] + m.m23 * v[3],
            m.m30 * v[0] + m.m31 * v[1] + m.m32 * v[2] + m.m33 * v[3],
        ]
    }

    pub fn multiply_vector3(m: Matrix4x4, v: Vector3, w: f32) -> Vector3 {
        let v4 = [v.x, v.y, v.z, w];
        let result = Self::multiply_vector4(m, v4);
        Vector3 {
            x: result[0],
            y: result[1],
            z: result[2],
        }
    }

    // 3d transformation stuff
    pub fn transformation(rotation: Quaternion, translation: Vector3) -> Self {
        let x2 = rotation.x + rotation.x;
        let y2 = rotation.y + rotation.y;
        let z2 = rotation.z + rotation.z;
        let xx = rotation.x * x2;
        let xy = rotation.x * y2;
        let xz = rotation.x * z2;
        let yy = rotation.y * y2;
        let yz = rotation.y * z2;
        let zz = rotation.z * z2;
        let wx = rotation.w * x2;
        let wy = rotation.w * y2;
        let wz = rotation.w * z2;

        Self {
            m00: 1. - (yy + zz),
            m01: xy - wz,
            m02: xz + wy,
            m03: translation.x,

            m10: xy + wz,
            m11: 1. - (xx + zz),
            m12: yz - wx,
            m13: translation.y,

            m20: xz - wy,
            m21: yz + wx,
            m22: 1. - (xx + yy),
            m23: translation.z,

            m30: 0.,
            m31: 0.,
            m32: 0.,
            m33: 1.,
        }
    }

    pub fn view(rotation: Quaternion, position: Vector3) -> Self {
        let r = rotation.rotate(vector3::RIGHT);
        let u = rotation.rotate(vector3::UP);
        let d = rotation.rotate(vector3::FORWARD);
        let c = position;

        Self {
            m00: r.x,
            m01: u.x,
            m02: d.x,
            m03: -c.x,
            m10: r.y,
            m11: u.y,
            m12: d.y,
            m13: -c.y,
            m20: r.z,
            m21: u.z,
            m22: d.z,
            m23: -c.z,
            m30: 0.,
            m31: 0.,
            m32: 0.,
            m33: 1.,
        }

        //let xx2 = r.x * r.x * 2.;
        //let yy2 = r.y * r.y * 2.;
        //let zz2 = r.z * r.z * 2.;
        //let xy2 = r.x * r.y * 2.;
        //let xz2 = r.x * r.z * 2.;
        //let xw2 = r.x * r.w * 2.;
        //let yz2 = r.y * r.z * 2.;
        //let yw2 = r.y * r.w * 2.;
        //let zw2 = r.z * r.w * 2.;
        //Self {
        //    m00: 1. - yy2 - zz2,
        //    m01: xy2 - zw2,
        //    m02: xz2 + yw2,
        //    m03: 0.,
        //    m10: xy2 + zw2,
        //    m11: 1. - xx2 - zz2,
        //    m12: yz2 - xw2,
        //    m13: 0.,
        //    m20: xz2 - yw2,
        //    m21: yz2 + xw2,
        //    m22: 1. - xx2 - yy2,
        //    m23: 0.,
        //    m30: 0.,
        //    m31: 0.,
        //    m32: 0.,
        //    m33: 1.,
        //}

        //let right = rotation.rotate(vector3::RIGHT);
        //let up = rotation.rotate(vector3::UP);
        //let forward = rotation.rotate(vector3::FORWARD);
        //Self {
        //    m00: right.x,
        //    m01: up.x,
        //    m02: forward.x,
        //    //m03: - right.x * position.x - right.y * position.y - right.z * position.z,
        //    m03: -position.x,
        //    m10: right.y,
        //    m11: up.y,
        //    m12: forward.y,
        //    //m13: - up.x * position.x - up.y * position.y - up.z * position.z,
        //    m13: -position.y,
        //    m20: right.z,
        //    m21: up.z,
        //    m22: forward.z,
        //    //m23: - forward.x * position.x - forward.y * position.y - forward.z * position.z,
        //    m23: -position.z,
        //    m30: 0.,
        //    m31: 0.,
        //    m32: 0.,
        //    m33: 1.,
        //}
    }

    pub fn perspective_projection(fovy: f32, aspect_ratio: f32, near: f32, far: f32) -> Self
    {
        let focal_length = 1. / super::tan(fovy / 2.);
        let x = focal_length / aspect_ratio;
        let y = focal_length;
        let a = near / (far - near);
        let b = far * a;

        Self{
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
            m32: -1.,
            m33: 0.,
        }
    }

    pub fn rotate(self, v: Vector3) -> Vector3 {
        Self::multiply_vector3(self, v, 0.)
    }

    pub fn rotate_and_transform(self, v: Vector3) -> Vector3 {
        Self::multiply_vector3(self, v, 1.)
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
