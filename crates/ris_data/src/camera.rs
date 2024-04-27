use ris_math::affine;
use ris_math::matrix::Mat4;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

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
            fovy: ris_math::radians(60.),
            aspect_ratio: 16. / 9.,
            near: 0.1,
            far: 10.0,
        }
    }
}

impl Camera {
    pub fn view_matrix(&self) -> Mat4 {
        // my coordinate system is x => right, y => forward and z => upward.
        // Vulkans coordinat system is x => right, y => down and z => forward.
        // both are right handed coordinate systems, therefore all relationships are equal.
        // only a single default rotation is necessary, to convert my system to vulkan.
        let default_rotation = Quat::from((0.5 * ris_math::PI, Vec3::right()));
        let camera_rotation = self.rotation.conjugate();
        let rotation = default_rotation * camera_rotation;
        let translation = -1. * self.position;

        let translation_mat = affine::translation(translation);
        let rotation_mat = affine::rotation(rotation);

        rotation_mat * translation_mat
    }

    pub fn projection_matrix(&self) -> Mat4 {
        let focal_length = 1. / ris_math::tan(self.fovy / 2.);
        let x = focal_length / self.aspect_ratio;
        let y = focal_length;
        let a = self.near / (self.far - self.near);
        let b = self.far * a;

        let mut mat = Mat4::init(0.0);
        mat.0 .0 = x;
        mat.1 .1 = y;
        mat.2 .2 = a;
        mat.3 .2 = b;
        mat.2 .3 = 1.;

        mat
    }
}

