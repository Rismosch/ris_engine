use std::f32::consts::PI;

use ris_math::color::Rgb;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

use crate::ui_helper::UiHelperDrawData;
use crate::ui_helper::UiHelperModule;

pub struct GizmoModule {
    angle: f32,
    draw_line: bool,
    draw_point: bool,
    draw_view_point: bool,
    draw_aabb: bool,
    draw_oob: bool,
    draw_text: bool,
}

impl GizmoModule {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            angle: 0.0,
            draw_line: false,
            draw_point: false,
            draw_view_point: false,
            draw_aabb: false,
            draw_oob: false,
            draw_text: false,
        })
    }
}

impl UiHelperModule for GizmoModule {
    fn name(&self) -> &'static str {
        "gizmos"
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> ris_error::RisResult<()> {
        let UiHelperDrawData { ui, frame, .. } = data;

        self.angle += 0.5 * frame.average_seconds() * PI;

        ui.checkbox("draw line", &mut self.draw_line);
        ui.checkbox("draw point", &mut self.draw_point);
        ui.checkbox("draw view point", &mut self.draw_view_point);
        ui.checkbox("draw aabb", &mut self.draw_aabb);
        ui.checkbox("draw oob", &mut self.draw_oob);
        ui.checkbox("draw text", &mut self.draw_text);

        if self.draw_line {
            let color_rotation = Quat::from((self.angle, ris_math::vector::VEC3_ONE));
            let color_dir = color_rotation.rotate(ris_math::vector::VEC3_RIGHT);
            let color = Rgb::from(color_dir.normalize());
            ris_debug::gizmo::segment(
                -1.0 * ris_math::vector::VEC3_ONE,
                ris_math::vector::VEC3_ONE,
                color,
            )?;
        }

        if self.draw_point {
            for i in 0..10 {
                for j in 0..10 {
                    for k in 0..10 {
                        ris_debug::gizmo::point(Vec3(i as f32, j as f32, k as f32), None)?;
                    }
                }
            }
        }

        if self.draw_view_point {
            for i in 0..10 {
                for j in 0..10 {
                    for k in 0..10 {
                        ris_debug::gizmo::view_point(
                            Vec3(i as f32, j as f32, k as f32),
                            Quat::from((0.25 * PI, ris_math::vector::VEC3_ONE)),
                            None,
                        )?;
                    }
                }
            }
        }

        if self.draw_aabb {
            ris_debug::gizmo::aabb(
                -1.0 * ris_math::vector::VEC3_ONE,
                ris_math::vector::VEC3_ONE,
                None,
            )?;
        }

        if self.draw_oob {
            ris_debug::gizmo::obb(
                ris_math::vector::VEC3_ZERO,
                ris_math::vector::VEC3_ONE,
                Quat::from((self.angle, ris_math::vector::VEC3_ONE)),
                None,
            )?;
        }


        if self.draw_text {
            ris_debug::gizmo::text(ris_math::vector::VEC3_RIGHT, "right")?;
            ris_debug::gizmo::text(ris_math::vector::VEC3_LEFT, "left")?;
            ris_debug::gizmo::text(ris_math::vector::VEC3_FORWARD, "forward")?;
            ris_debug::gizmo::text(ris_math::vector::VEC3_BACKWARD, "backward")?;
            ris_debug::gizmo::text(ris_math::vector::VEC3_UP, "up")?;
            ris_debug::gizmo::text(ris_math::vector::VEC3_DOWN, "down")?;
        }

        Ok(())
    }
}
