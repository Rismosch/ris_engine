use crate::matrix::Mat3;
use crate::matrix::Mat4;
use crate::quaternion::Quat;
use crate::vector::Vec3;

/// returns a translation matrix
pub fn from_translation(t: Vec3) -> Mat4 {
    let Vec3(x, y, z) = t;

    let mut m = Mat4::init(1.0);
    m.3 .0 = x;
    m.3 .1 = y;
    m.3 .2 = z;

    m
}

/// converts a translation matrix to a translation vector
pub fn to_translation(m: Mat4) -> Vec3 {
    let x = m.3 .0;
    let y = m.3 .1;
    let z = m.3 .2;

    Vec3(x, y, z)
}

/// returns a rotation matrix
pub fn from_rotation(r: Quat) -> Mat3 {
    let Quat(x, y, z, w) = r;

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

    let mut m = Mat3::default();

    m.0 .0 = 1. - (yy + zz);
    m.1 .0 = xy - wz;
    m.2 .0 = xz + wy;

    m.0 .1 = xy + wz;
    m.1 .1 = 1. - (xx + zz);
    m.2 .1 = yz - wx;

    m.0 .2 = xz - wy;
    m.1 .2 = yz + wx;
    m.2 .2 = 1. - (xx + yy);

    m
}

/// converts a rotation matrix to a quaternion
pub fn to_rotation(m: Mat3) -> Quat {
    let tr = m.0 .0 + m.1 .1 + m.2 .2;
    if tr > 0.0 {
        let s = f32::sqrt(tr + 1.0);
        let w = s / 2.0;
        let s = 0.5 / s;
        let x = (m.1 .2 - m.2 .1) * s;
        let y = (m.2 .0 - m.0 .2) * s;
        let z = (m.0 .1 - m.1 .0) * s;
        Quat(x, y, z, w)
    } else {
        let nxt = [1, 2, 0];

        let mut i = 0;
        if m.1 .1 > m.0 .0 {
            i = 1;
        }
        if m.2 .2 > m[i][i] {
            i = 2;
        }
        let j = nxt[i];
        let k = nxt[j];

        let mut s = f32::sqrt(m[i][i] - m[j][j] - m[k][k] + 1.0);

        let mut q = Quat::default();
        q[i] = s * 0.5;

        if s != 0.0 {
            s = 0.5 / s;
        }

        q.3 = (m[j][k] - m[k][j]) * s;
        q[j] = (m[j][i] + m[i][j]) * s;
        q[k] = (m[k][i] + m[i][k]) * s;

        q
    }
}

// returns a rotation matrix, using direction and up
pub fn look_at(direction: Vec3, up: Vec3) -> Mat3 {
    let m2 = up;
    let right = direction.cross(m2);
    let m0 = right / f32::sqrt(f32::max(0.000_01, right.length_squared()));
    let m1 = m2.cross(m0);

    Mat3(m0, m1, m2)
}

/// returns a scale matrix
pub fn from_scale(s: Vec3) -> Mat3 {
    let Vec3(x, y, z) = s;

    let mut m = Mat3::init(1.0);
    m.0 .0 = x;
    m.1 .1 = y;
    m.2 .2 = z;

    m
}

/// converts a trs matrix to a scale
///
/// **NOTE:** if you want to extract a scale from a trs matrix, use `decompose_trs` instead
pub fn to_scale(m: Mat3) -> Vec3 {
    let x = m.0 .0;
    let y = m.1 .1;
    let z = m.2 .2;

    Vec3(x, y, z)
}

/// returns a translation-rotation-scale matrix
pub fn trs_compose(t: Vec3, r: Quat, s: f32) -> Mat4 {
    ris_error::throw_debug_assert!(s > 0.0, "non-positive scale is not supported");

    let t = from_translation(t);
    let r = Mat4::from(from_rotation(r));
    let s = Mat4::from(from_scale(Vec3::init(s)));

    t * r * s
}

/// decomposes a trandlation-rotation-scale matrix
pub fn trs_decompose(m: Mat4) -> (Vec3, Quat, f32) {
    // compute translation
    let translation = to_translation(m);

    // for the next steps we only care bout the top left 3x3 matrix
    let mut m = Mat3::from(m);

    // compute scale, assume scale is uniform
    let scale = m.0.length();

    // normalize columns
    m.0 = m.0.normalize();
    m.1 = m.1.normalize();
    m.2 = m.2.normalize();

    // at this point m is a pure rotation matrix, compute rotation
    let rotation = to_rotation(m);

    // return
    (translation, rotation, scale)
}

pub struct Decomposed {
    pub scale: Vec3,
    pub skew: Vec3,
    pub rotation: Quat,
    pub translation: Vec3,
}

/// fully decomposes a matrix. this is a more complete implementation than `trs_decompose`. but it
/// should be noted that this implementation is not 100% complete.
///
/// TODO: look into a full implementation once it is required. look into glm and Graphics Gems II
pub fn decompose_fully(m: Mat4) -> Decomposed {
    // compute translation
    let translation = to_translation(m);

    // for the next steps we only care bout the top left 3x3 matrix
    let mut m = Mat3::from(m);

    // compute scale x and normalize 1st column
    let sx = m.0.length();
    m.0 = m.0.normalize();

    // compute shear xy and make 2nd column orthogonal to 1st
    let mut sxy = m.0.dot(m.1);
    m.1 -= sxy * m.0;

    // compute scale y and normalize 2nd column
    let sy = m.1.length();
    m.1 = m.1.normalize();
    sxy /= sy;

    // compute shear xz and yz, and make 3rd column orthogonal to the 1st and 2nd
    let mut sxz = m.0.dot(m.2);
    m.2 -= sxz * m.0;
    let mut syz = m.1.dot(m.2);
    m.2 -= syz * m.1;

    // compute scale z and normalize 3rd column
    let sz = m.2.length();
    m.2 = m.2.normalize();
    sxz /= sz;
    syz /= sz;

    // at this point m is a pure rotation matrix, compute rotation
    let rotation = to_rotation(m);

    // return
    let skew = Vec3(syz, sxz, sxy);
    let scale = Vec3(sx, sy, sz);

    Decomposed {
        scale,
        skew,
        rotation,
        translation,
    }
}
