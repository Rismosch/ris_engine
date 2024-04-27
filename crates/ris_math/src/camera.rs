use super::affine;
use super::matrix::Mat4;
use super::quaternion::Quat;
use super::vector::Vec3;

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
            fovy: super::radians(60.),
            aspect_ratio: 16. / 9.,
            near: 0.1,
            far: 10.0,
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

        let default_rotation = Quat::from((0.5 * super::PI, Vec3::right()));
        let camera_rotation = self.rotation.conjugate();
        let rotation = default_rotation * camera_rotation;
        let translation = -1.0 * self.position;

        let translation_mat = affine::translation(translation);
        let rotation_mat = affine::rotation(rotation);

        rotation_mat * translation_mat
    }

    pub fn projection_matrix(&self) -> Mat4 {
        let tan_half_fovy = super::tan(self.fovy / 2.0);

        let mut mat = Mat4::init(0.0);
        mat.0 .0 = 1.0 / (self.aspect_ratio * tan_half_fovy);
        mat.1 .1 = 1.0 / tan_half_fovy;
        mat.2 .2 = self.far / (self.far - self.near);
        mat.2 .3 = 1.0;
        mat.3 .2 = -(self.far * self.near) / (self.far - self.near);

        mat
    }
}

