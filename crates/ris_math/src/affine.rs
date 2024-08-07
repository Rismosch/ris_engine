use crate::matrix::Mat4;
use crate::quaternion::Quat;
use crate::vector::Vec3;

pub fn translation(translation: Vec3) -> Mat4 {
    let Vec3(x, y, z) = translation;

    let mut mat = Mat4::init(1.0);
    mat.3 .0 = x;
    mat.3 .1 = y;
    mat.3 .2 = z;

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

    let mut mat = Mat4::init(1.0);

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

pub fn scale(scale: Vec3) -> Mat4 {
    let Vec3(x, y, z) = scale;

    let mut mat = Mat4::init(1.0);
    mat.0.0 = x;
    mat.1.1 = y;
    mat.2.2 = z;

    mat
}

pub fn trs(t: Vec3, r: Quat, s: Vec3) -> Mat4 {
    let t = translation(t);
    let r = rotation(r);
    let s = scale(s);

    t * r * s
}

pub fn get_translation(mat: Mat4) -> Vec3 {
    let x = mat.3 .0;
    let y = mat.3 .1;
    let z = mat.3 .2;

    Vec3(x, y, z)
}

pub fn get_rotation(mat: Mat4) -> Quat {
    let nxt = [1, 2, 0];

    let tr = mat.0.0 + mat.1.1 + mat.2.2;

    if tr > 0.0 {
        let s = f32::sqrt(tr + 1.0);
        let w = s / 2.0;
        let s = 0.5 / s;
        let x = (mat.1.2 - mat.2.1) * s;
        let y = (mat.2.0 - mat.0.2) * s;
        let z = (mat.0.1 - mat.1.0) * s;
        Quat(x,y,z,w)
    } else {
        let mut i = 0;
        if mat.1.1 > mat.0.0 {
            i = 1;
        }
        if mat.2.2 > mat[i][i] {
            i = 2;
        }
        let j = nxt[i];
        let k = nxt[j];

        let mut s = f32::sqrt(mat[i][i] - mat[j][j] - mat[k][k] + 1.0);

        let mut q = Quat::default();
        q[i] = s * 0.5;

        if s != 0.0 {
            s = 0.5 / s;
        }

        q.3 = (mat[j][k] - mat[k][j]) * s;
        q[j] = (mat[j][i] + mat[i][j]) * s;
        q[k] = (mat[k][i] + mat[i][k]) * s;

        q
    }
}

pub fn get_scale(mat: Mat4) -> Vec3 {
    let x = mat.0.0;
    let y = mat.1.1;
    let z = mat.2.2;

    Vec3(x, y, z)
}

pub fn get_trs(mat: Mat4) -> (Vec3, Quat, Vec3) {
    (Vec3::default(), Quat::default(), Vec3::default())
}

