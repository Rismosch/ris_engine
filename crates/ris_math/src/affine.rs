use crate::matrix::Mat4;
use crate::quaternion::Quat;
use crate::vector::Vec3;

pub fn translation(translation: Vec3) -> Mat4 {
    let mut mat = Mat4::init(1.);
    mat.3 .0 = translation.0;
    mat.3 .1 = translation.1;
    mat.3 .2 = translation.2;

    mat
}

pub fn rotation(rotation: Quat) -> Mat4 {
    let Quat(x, y, z, w) = rotation;

    let x2 = x + x;
    let y2 = y + y;
    let z2 = z + z;
    let xx = x * x2;
    let xy = x * y2;
    let xz = x * z2;
    let yy = y * y2;
    let yz = y * z2;
    let zz = z * z2;
    let wx = w * x2;
    let wy = w * y2;
    let wz = w * z2;

    let mut mat = Mat4::init(1.);

    mat.0 .0 = 1. - (yy + zz);
    mat.1 .0 = xy - wz;
    mat.2 .0 = xz + wy;

    mat.0 .1 = xy + wz;
    mat.1 .1 = 1. - (xx + zz);
    mat.2 .1 = yz - wx;

    mat.0 .2 = xz - wy;
    mat.1 .2 = yz + wx;
    mat.2 .2 = 1. - (xx + yy);

    mat
}

// reflection
// shear
// scale

