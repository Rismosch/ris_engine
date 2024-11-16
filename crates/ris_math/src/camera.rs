use std::f32::consts::PI;

use crate::affine;
use crate::matrix::Mat4;
use crate::quaternion::Quat;
use crate::vector::Vec3;

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vec3,
    pub rotation: Quat,

    pub fovy: f32,
    pub aspect_ratio: f32, // width / height
    pub near: f32,
    pub far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Default::default(),
            rotation: Default::default(),
            fovy: 60f32.to_radians(),
            aspect_ratio: 16. / 9.,
            near: 0.1,
            far: 100.0,
        }
    }
}

impl Camera {
    pub fn view_matrix(&self) -> Mat4 {
        // my coordinate system is
        //  x => right
        //  y => forward
        //  z => upward
        //
        // vulkans coordinate system is
        //  x => right
        //  y => down
        //  z => forward
        //
        //  a single rotation is sufficiant to translate one to the other

        let default_rotation = Quat::from((0.5 * PI, Vec3::right()));
        let camera_rotation = self.rotation.conjugate();
        let rotation = default_rotation * camera_rotation;
        let translation = -1.0 * self.position;

        let translation_mat = affine::from_translation(translation);
        let rotation_mat = Mat4::from(affine::from_rotation(rotation));

        rotation_mat * translation_mat
    }

    pub fn projection_matrix(&self) -> Mat4 {
        let n = self.near;
        let f = self.far;
        let e = 1.0 / f32::tan(self.fovy * 0.5);

        let mut mat = Mat4::init(0.0);
        mat.0 .0 = e / self.aspect_ratio;
        mat.1 .1 = e;
        mat.2 .2 = f / (f - n);
        mat.2 .3 = 1.0;
        mat.3 .2 = -(f * n) / (f - n);

        mat
    }
}
