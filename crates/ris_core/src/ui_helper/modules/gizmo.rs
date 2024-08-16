use std::f32::consts::PI;

use ris_data::info::app_info::AppInfo;
use ris_math::color::Rgb;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

use crate::ui_helper::IUiHelperModule;
use crate::ui_helper::SharedStateWeakPtr;
use crate::ui_helper::UiHelperDrawData;

pub struct GizmoModule {
    angle: f32,
    draw_line: bool,
    draw_point: bool,
    draw_view_point: bool,
    draw_aabb: bool,
    draw_obb: bool,
    draw_text: bool,
}

impl IUiHelperModule for GizmoModule {
    fn name() -> &'static str {
        "gizmo"
    }

    fn build(_shared_state: SharedStateWeakPtr) -> Box<dyn IUiHelperModule> {
        Box::new(Self {
            angle: 0.0,
            draw_line: false,
            draw_point: false,
            draw_view_point: false,
            draw_aabb: false,
            draw_obb: false,
            draw_text: false,
        })
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> ris_error::RisResult<()> {
        let UiHelperDrawData { ui, frame, .. } = data;

        self.angle += 0.5 * frame.average_seconds() * PI;

        ui.checkbox("draw line", &mut self.draw_line);
        ui.checkbox("draw point", &mut self.draw_point);
        ui.checkbox("draw view point", &mut self.draw_view_point);
        ui.checkbox("draw aabb", &mut self.draw_aabb);
        ui.checkbox("draw obb", &mut self.draw_obb);
        ui.checkbox("draw text", &mut self.draw_text);

        if self.draw_line {
            let color_rotation = Quat::from((self.angle, Vec3::init(1.0)));
            let color_dir = color_rotation.rotate(Vec3::right());
            let color = Rgb::from(color_dir.normalize());
            ris_debug::gizmo::segment(-1.0 * Vec3::init(1.0), Vec3::init(1.0), color)?;
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
                            Quat::from((0.25 * PI, Vec3::init(1.0))),
                            None,
                        )?;
                    }
                }
            }
        }

        if self.draw_aabb {
            ris_debug::gizmo::aabb(-1.0 * Vec3::init(1.0), Vec3::init(1.0), None)?;
        }

        if self.draw_obb {
            ris_debug::gizmo::obb(
                Vec3::init(0.0),
                Vec3::init(1.0),
                Quat::from((self.angle, Vec3::init(1.0))),
                None,
            )?;
        }

        if self.draw_text {
            ris_debug::gizmo::text(Vec3::right(), "right")?;
            ris_debug::gizmo::text(Vec3::left(), "left")?;
            ris_debug::gizmo::text(Vec3::forward(), "forward")?;
            ris_debug::gizmo::text(Vec3::backward(), "backward")?;
            ris_debug::gizmo::text(Vec3::up(), "up")?;
            ris_debug::gizmo::text(Vec3::down(), "down")?;
        }

        Ok(())
    }
}
