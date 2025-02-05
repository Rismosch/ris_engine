#[cfg(feature = "gizmos_enabled")]
use std::sync::Mutex;

use ris_error::RisResult;
use ris_math::camera::Camera;
use ris_math::color::Rgb;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

#[cfg(feature = "gizmos_enabled")]
// geometry shader outputs max 128 vertices. with 4 vertices per glyph, this puts a hard limit on
// how long a text can be
const MAX_TEXT_LEN: usize = 32;

#[cfg(feature = "gizmos_enabled")]
static GIZMOS: Mutex<Option<Gizmos>> = Mutex::new(None);

pub struct GizmoGuard;

#[cfg(feature = "gizmos_enabled")]
impl Drop for GizmoGuard {
    fn drop(&mut self) {
        match GIZMOS.lock() {
            Err(e) => ris_log::error!("error while dropping gimzo: {}", e),
            Ok(mut gizmo) => {
                *gizmo = None;
            }
        }
    }
}

pub fn init() -> RisResult<GizmoGuard> {

    #[cfg(feature = "gizmos_enabled")]
    {
        let mut gizmo = GIZMOS.lock()?;
        *gizmo = Some(Gizmos {
            shapes: Vec::new(),
            text: Vec::new(),
        });
    }

    Ok(GizmoGuard)
}

#[cfg(feature = "gizmos_enabled")]
enum GizmoShape {
    Segment {
        start: Vec3,
        end: Vec3,
        color: Rgb,
    },
    Point {
        position: Vec3,
        color: Option<Rgb>,
    },
    ViewPoint {
        position: Vec3,
        rotation: Quat,
        color: Option<Rgb>,
    },
    Aabb {
        min: Vec3,
        max: Vec3,
        color: Option<Rgb>,
    },
    Obb {
        center: Vec3,
        half_scale: Vec3,
        rotation: Quat,
        color: Option<Rgb>,
    },
}

#[cfg(feature = "gizmos_enabled")]
struct GizmoText {
    position: Vec3,
    bytes: Vec<u8>,
}

#[cfg(feature = "gizmos_enabled")]
struct Gizmos {
    shapes: Vec<GizmoShape>,
    text: Vec<GizmoText>,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct GizmoSegmentVertex {
    pub pos: Vec3,
    pub color: Rgb,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct GizmoTextVertex {
    pub pos: Vec3,
    pub text_addr: u32,
    pub text_len: u32,
}

pub fn new_frame() -> RisResult<()> {

    #[cfg(feature = "gizmos_enabled")]
    {
        let Some(ref mut gizmos) = *GIZMOS.lock()? else {
            return Ok(());
        };

        gizmos.shapes.clear();
        gizmos.text.clear();
    }

    Ok(())
}

pub fn segment(start: Vec3, end: Vec3, color: Rgb) -> RisResult<()> {
    #[cfg(feature = "gizmos_enabled")]
    {
        let Some(ref mut gizmos) = *GIZMOS.lock()? else {
            return Ok(());
        };

        let shape = GizmoShape::Segment { start, end, color };
        gizmos.shapes.push(shape);
    }

    #[cfg(not(feature = "gizmos_enabled"))]
    {
        let _ = start;
        let _ = end;
        let _ = color;
    }
    Ok(())
}

pub fn point(position: Vec3, color: Option<Rgb>) -> RisResult<()> {
    #[cfg(feature = "gizmos_enabled")]
    {
        let Some(ref mut gizmos) = *GIZMOS.lock()? else {
            return Ok(());
        };

        let shape = GizmoShape::Point { position, color };
        gizmos.shapes.push(shape);
    }
    #[cfg(not(feature = "gizmos_enabled"))]
    {
        let _ = position;
        let _ = color;
    }
    Ok(())
}

pub fn view_point(position: Vec3, rotation: Quat, color: Option<Rgb>) -> RisResult<()> {
    #[cfg(feature = "gizmos_enabled")]
    {
        let Some(ref mut gizmos) = *GIZMOS.lock()? else {
            return Ok(());
        };

        let shape = GizmoShape::ViewPoint {
            position,
            rotation,
            color,
        };
        gizmos.shapes.push(shape);
    }
    #[cfg(not(feature = "gizmos_enabled"))]
    {
        let _ = position;
        let _ = rotation;
        let _ = color;
    }
    Ok(())
}

pub fn aabb(min: Vec3, max: Vec3, color: Option<Rgb>) -> RisResult<()> {
    #[cfg(feature = "gizmos_enabled")]
    {
        let Some(ref mut gizmos) = *GIZMOS.lock()? else {
            return Ok(());
        };

        let min = Vec3(
            f32::min(min.0, max.0),
            f32::min(min.1, max.1),
            f32::min(min.2, max.2),
        );
        let max = Vec3(
            f32::max(min.0, max.0),
            f32::max(min.1, max.1),
            f32::max(min.2, max.2),
        );

        let shape = GizmoShape::Aabb { min, max, color };
        gizmos.shapes.push(shape);
    }
    #[cfg(not(feature = "gizmos_enabled"))]
    {
        let _ = min;
        let _ = max;
        let _ = color;
    }
    Ok(())
}

pub fn obb(center: Vec3, half_scale: Vec3, rotation: Quat, color: Option<Rgb>) -> RisResult<()> {
    #[cfg(feature = "gizmos_enabled")]
    {
        let Some(ref mut gizmos) = *GIZMOS.lock()? else {
            return Ok(());
        };

        let half_scale = Vec3(half_scale.0, half_scale.1, half_scale.2).abs();

        let shape = GizmoShape::Obb {
            center,
            half_scale,
            rotation,
            color,
        };

        gizmos.shapes.push(shape);
    }

    #[cfg(not(feature = "gizmos_enabled"))]
    {
        let _ = center;
        let _ = half_scale;
        let _ = rotation;
        let _ = color;
    }

    Ok(())
}

pub fn text(position: Vec3, text: &str) -> RisResult<()> {

    #[cfg(feature = "gizmos_enabled")]
    {
        let Some(ref mut gizmos) = *GIZMOS.lock()? else {
            return Ok(());
        };

        let mut bytes = text.as_bytes();
        if bytes.len() > MAX_TEXT_LEN {
            bytes = &bytes[0..MAX_TEXT_LEN]
        }

        let gizmo_text = GizmoText {
            position,
            bytes: bytes.to_vec(),
        };

        gizmos.text.push(gizmo_text);
    }

    #[cfg(not(feature = "gizmos_enabled"))]
    {
        let _ = position;
        let _ = text;
    }
    Ok(())
}

pub fn draw_segments(camera: &Camera) -> RisResult<Vec<GizmoSegmentVertex>> {

    #[cfg(feature = "gizmos_enabled")]
    {
        let Some(ref mut gizmos) = *GIZMOS.lock()? else {
            return Ok(Vec::new());
        };

        let mut segments = Vec::new();

        for shape in gizmos.shapes.iter() {
            match *shape {
                GizmoShape::Segment { start, end, color } => {
                    add_segment(camera, &mut segments, start, end, color);
                }
                GizmoShape::Point { position, color } => {
                    const MAGIC_SCALE: f32 = 0.03;

                    let camera_distance = camera.position.distance(position);
                    let scale = MAGIC_SCALE * camera_distance;

                    let red = color.unwrap_or(Rgb::red());
                    let green = color.unwrap_or(Rgb::green());
                    let blue = color.unwrap_or(Rgb::blue());
                    let cyan = color.unwrap_or(Rgb::cyan());
                    let magenta = color.unwrap_or(Rgb::magenta());
                    let yellow = color.unwrap_or(Rgb::yellow());

                    let v0 = position;
                    let v1 = position + scale * Vec3::right();
                    let v2 = position + scale * Vec3::left();
                    let v3 = position + scale * Vec3::forward();
                    let v4 = position + scale * Vec3::backward();
                    let v5 = position + scale * Vec3::up();
                    let v6 = position + scale * Vec3::down();

                    add_segment(camera, &mut segments, v0, v1, red);
                    add_segment(camera, &mut segments, v0, v2, cyan);
                    add_segment(camera, &mut segments, v0, v3, green);
                    add_segment(camera, &mut segments, v0, v4, magenta);
                    add_segment(camera, &mut segments, v0, v5, blue);
                    add_segment(camera, &mut segments, v0, v6, yellow);
                }
                GizmoShape::ViewPoint {
                    position,
                    rotation,
                    color,
                } => {
                    let red = color.unwrap_or(Rgb::red());
                    let green = color.unwrap_or(Rgb::green());
                    let blue = color.unwrap_or(Rgb::blue());

                    let v0 = position;
                    let v1 = position + rotation.rotate(Vec3::right());
                    let v2 = position + rotation.rotate(Vec3::forward());
                    let v3 = position + rotation.rotate(Vec3::up());

                    add_segment(camera, &mut segments, v0, v1, red);
                    add_segment(camera, &mut segments, v0, v2, green);
                    add_segment(camera, &mut segments, v0, v3, blue);
                }
                GizmoShape::Aabb { min, max, color } => {
                    let red = color.unwrap_or(Rgb::red());
                    let green = color.unwrap_or(Rgb::green());
                    let blue = color.unwrap_or(Rgb::blue());
                    let cyan = color.unwrap_or(Rgb::cyan());
                    let magenta = color.unwrap_or(Rgb::magenta());
                    let yellow = color.unwrap_or(Rgb::yellow());

                    let v0 = Vec3(min.x(), min.y(), min.z());
                    let v1 = Vec3(max.x(), min.y(), min.z());
                    let v2 = Vec3(min.x(), max.y(), min.z());
                    let v3 = Vec3(max.x(), max.y(), min.z());
                    let v4 = Vec3(min.x(), min.y(), max.z());
                    let v5 = Vec3(max.x(), min.y(), max.z());
                    let v6 = Vec3(min.x(), max.y(), max.z());
                    let v7 = Vec3(max.x(), max.y(), max.z());

                    add_segment(camera, &mut segments, v1, v3, red);
                    add_segment(camera, &mut segments, v5, v7, red);
                    add_segment(camera, &mut segments, v2, v6, green);
                    add_segment(camera, &mut segments, v3, v7, green);
                    add_segment(camera, &mut segments, v4, v5, blue);
                    add_segment(camera, &mut segments, v6, v7, blue);
                    add_segment(camera, &mut segments, v0, v2, cyan);
                    add_segment(camera, &mut segments, v4, v6, cyan);
                    add_segment(camera, &mut segments, v0, v4, magenta);
                    add_segment(camera, &mut segments, v1, v5, magenta);
                    add_segment(camera, &mut segments, v0, v1, yellow);
                    add_segment(camera, &mut segments, v2, v3, yellow);
                }
                GizmoShape::Obb {
                    center,
                    half_scale,
                    rotation,
                    color,
                } => {
                    let red = color.unwrap_or(Rgb::red());
                    let green = color.unwrap_or(Rgb::green());
                    let blue = color.unwrap_or(Rgb::blue());
                    let cyan = color.unwrap_or(Rgb::cyan());
                    let magenta = color.unwrap_or(Rgb::magenta());
                    let yellow = color.unwrap_or(Rgb::yellow());

                    let x = half_scale.x() * rotation.rotate(Vec3::right());
                    let y = half_scale.y() * rotation.rotate(Vec3::forward());
                    let z = half_scale.z() * rotation.rotate(Vec3::up());

                    let v0 = center - x - y - z;
                    let v1 = center + x - y - z;
                    let v2 = center - x + y - z;
                    let v3 = center + x + y - z;
                    let v4 = center - x - y + z;
                    let v5 = center + x - y + z;
                    let v6 = center - x + y + z;
                    let v7 = center + x + y + z;

                    add_segment(camera, &mut segments, v1, v3, red);
                    add_segment(camera, &mut segments, v5, v7, red);
                    add_segment(camera, &mut segments, v2, v6, green);
                    add_segment(camera, &mut segments, v3, v7, green);
                    add_segment(camera, &mut segments, v4, v5, blue);
                    add_segment(camera, &mut segments, v6, v7, blue);
                    add_segment(camera, &mut segments, v0, v2, cyan);
                    add_segment(camera, &mut segments, v4, v6, cyan);
                    add_segment(camera, &mut segments, v0, v4, magenta);
                    add_segment(camera, &mut segments, v1, v5, magenta);
                    add_segment(camera, &mut segments, v0, v1, yellow);
                    add_segment(camera, &mut segments, v2, v3, yellow);
                }
            }
        }

        segments.sort_by(|left, right| right.0.total_cmp(&left.0));

        let vertices = segments.into_iter().flat_map(|x| [x.1, x.2]).collect();

        Ok(vertices)
    }


    #[cfg(not(feature = "gizmos_enabled"))]
    {
        let _ = camera;
        Ok(Vec::with_capacity(0))
    }
}

pub fn draw_text() -> RisResult<(Vec<GizmoTextVertex>, Vec<u8>)> {

    #[cfg(feature = "gizmos_enabled")]
    {
        let Some(ref mut gizmos) = *GIZMOS.lock()? else {
            return Ok((Vec::new(), Vec::new()));
        };

        let mut vertices = Vec::new();
        let mut texture = Vec::new();

        let mut text_addr = 0;
        for GizmoText { position, bytes } in gizmos.text.iter_mut() {
            let bytes_len: u32 = bytes.len().try_into()?;
            let vertex = GizmoTextVertex {
                pos: *position,
                text_addr,
                text_len: bytes_len,
            };

            vertices.push(vertex);
            texture.append(bytes);

            text_addr += bytes_len;
        }

        let modulo = texture.len() % 4;
        if modulo != 0 {
            let padding = 4 - modulo;
            texture.resize(texture.len() + padding, 0);
        }

        Ok((vertices, texture))
    }

    #[cfg(not(feature = "gizmos_enabled"))]
    {
        Ok((Vec::with_capacity(0), Vec::with_capacity(0)))
    }
}

#[cfg(feature = "gizmos_enabled")]
fn add_segment(
    camera: &Camera,
    segments: &mut Vec<(f32, GizmoSegmentVertex, GizmoSegmentVertex)>,
    start: Vec3,
    end: Vec3,
    color: Rgb,
) {
    let middle = (start + end) / 2.0;
    let distance = middle.distance_squared(camera.position);
    let v0 = GizmoSegmentVertex { pos: start, color };
    let v1 = GizmoSegmentVertex { pos: end, color };

    segments.push((distance, v0, v1));
}
