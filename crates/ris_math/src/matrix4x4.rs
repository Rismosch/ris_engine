use crate::quaternion::Quaternion;
use crate::vector3::Vector3;

// m00 m01 m02 m03
// m10 m11 m12 m13
// m20 m21 m22 m23
// m30 m31 m32 m33
#[derive(Debug, Copy, Clone, Default)]
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

impl Matrix4x4 {
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

    pub fn get(&self, m: usize, n: usize) -> f32 {
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


}
