use crate::affine;
use crate::matrix::Mat3;
use crate::quaternion::Quat;
use crate::vector::Vec3;

/// FOR DISPLAY PURPOSES ONLY. EULER ANGLES ARE A BAD WAY TO REPRESENT ROTATIONS, AND I WONT
/// IMPLEMENT A SINGLE UTILITY FUNCTION FOR THEM. PERIOD.
///
/// rotation sequence: XYZ
/// angles in degrees
pub fn from(value: Quat) -> Vec3 {
    let m = affine::from_rotation(value);

    let t1 = f32::atan2(m.2 .1, m.2 .2);
    let c2 = f32::sqrt(m.0 .0 * m.0 .0 + m.1 .0 * m.1 .0);
    let t2 = f32::atan2(-m.2 .0, c2);
    let s1 = f32::sin(t1);
    let c1 = f32::cos(t1);
    let t3 = f32::atan2(s1 * m.0 .2 - c1 * m.0 .1, c1 * m.1 .1 - s1 * m.1 .2);

    Vec3(-t1, -t2, -t3).degrees()
}

/// FOR DISPLAY PURPOSES ONLY. EULER ANGLES ARE A BAD WAY TO REPRESENT ROTATIONS, AND I WONT
/// IMPLEMENT A SINGLE UTILITY FUNCTION FOR THEM. PERIOD.
///
/// rotation sequence: XYZ
/// angles in degrees
pub fn to_quat(value: Vec3) -> Quat {
    let e = value.radians();

    let c1 = f32::cos(-e.0);
    let c2 = f32::cos(-e.1);
    let c3 = f32::cos(-e.2);
    let s1 = f32::sin(-e.0);
    let s2 = f32::sin(-e.1);
    let s3 = f32::sin(-e.2);

    let mut m = Mat3::default();
    m.0 .0 = c2 * c3;
    m.0 .1 = -c1 * s3 + s1 * s2 * c3;
    m.0 .2 = s1 * s3 + c1 * s2 * c3;
    m.1 .0 = c2 * s3;
    m.1 .1 = c1 * c3 + s1 * s2 * s3;
    m.1 .2 = -s1 * c3 + c1 * s2 * s3;
    m.2 .0 = -s2;
    m.2 .1 = s1 * c2;
    m.2 .2 = c1 * c2;

    affine::to_rotation(m)
}
