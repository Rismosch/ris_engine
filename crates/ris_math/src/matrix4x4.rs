use crate::quaternion::Quaternion;

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
    pub fn from_quaternion(q: Quaternion) -> Self {
        let x2 = q.x + q.x;
        let y2 = q.y + q.y;
        let z2 = q.z + q.z;
        let xx = q.x * x2;
        let xy = q.x * y2;
        let xz = q.x * z2;
        let yy = q.y * y2;
        let yz = q.y * z2;
        let zz = q.z * z2;
        let wx = q.w * x2;
        let wy = q.w * y2;
        let wz = q.w * z2;

        Self {
            m00: 1. - (yy + zz),
            m01: xy - wz,
            m02: xz + wy,
            m03: 0.,
            m10: xy + wz,
            m11: 1. - (xx + zz),
            m12: yz - wx,
            m13: 0.,
            m20: xz - wy,
            m21: yz + wx,
            m22: 1. - (xx + yy),
            m23: 0.,
            m30: 0.,
            m31: 0.,
            m32: 0.,
            m33: 0.,
        }
    }
}
