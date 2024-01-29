use crate::matrix::Mat4x4;
use crate::quaternion::Quat;
use crate::vector::Vec3;

pub struct Space;

impl Space {
    pub fn translation(translation: Vec3) -> Mat4x4 {
        let mut mat = Mat4x4::init(1.);
        mat.3.0 = translation.0;
        mat.3.1 = translation.1;
        mat.3.2 = translation.2;

        mat
    }

    pub fn rotation(rotation: Quat) -> Mat4x4 {
        let Quat(x, y, z, w) = rotation;
        let mut mat = Mat4x4::init(1.);

        let sqx = x * x;
        let sqy = y * y;
        let sqz = z * z;
        let sqw = w * w;

        mat.0.0 = sqx - sqy - sqz + sqw;
        mat.1.1 = -sqx + sqy - sqz + sqw;
        mat.2.2 = -sqx - sqy + sqz + sqw;

        let temp1 = x * y;
        let temp2 = z * w;
        mat.1.0 = 2. * (temp1 - temp2);
        mat.0.1 = 2. * (temp1 + temp2);

        let temp1 = x * z;
        let temp2 = y * w;
        mat.2.0 = 2. * (temp1 + temp2);
        mat.0.2 = 2. * (temp1 - temp2);

        let temp1 = y * z;
        let temp2 = x * w;
        mat.2.1 = 2. * (temp1 - temp2);
        mat.1.2 = 2. * (temp1 + temp2);

        mat
    }

    pub fn view(camera_position: Vec3, camera_rotation: Quat) -> Mat4x4 {
        // my coordinate system is x => right, y => forward and z => upward.
        // Vulkans coordinat system is x => right, y => down and z => forward.
        // both are right handed coordinate systems, therefore all relationships are equal.
        // only a single default rotation is necessary, to convert my system to vulkan.
        let default_rotation = Quat::from((0.5 * crate::PI, Vec3::right()));
        let camera_rotation = camera_rotation.conjugate();
        let rotation = default_rotation * camera_rotation;
        let translation = -1. * camera_position;

        let translation_mat = Self::translation(translation);
        let rotation_mat = Self::rotation(rotation);

        rotation_mat * translation_mat
    }

    pub fn proj(fovy: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4x4 {
        let focal_length = 1. / crate::tan(fovy / 2.);
        let x = focal_length / aspect_ratio;
        let y = focal_length;
        let a = near / (far - near);
        let b = far * a;

        let mut mat = Mat4x4::init(0.0);
        mat.0.0 = x;
        mat.1.1 = y;
        mat.2.2 = a;
        mat.3.2 = b;
        mat.2.3 = 1.;

        mat
    }
}
