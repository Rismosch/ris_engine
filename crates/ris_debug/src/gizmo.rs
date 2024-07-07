use std::sync::Mutex;

use ris_error::Extensions;
use ris_error::RisResult;
use ris_math::color::Rgb;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

static GIZMOS: Mutex<Option<Gizmos>> = Mutex::new(None);

pub struct GizmoGuard;

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

/// # Safety
///
/// Gizmo is a singleton. Initialize only once.
pub unsafe fn init() -> RisResult<GizmoGuard> {
    let mut gizmo = GIZMOS.lock()?;
    *gizmo = Some(Gizmos {
        to_draw: Vec::new(),
    });

    Ok(GizmoGuard)
}

struct Gizmos {
    to_draw: Vec<GizmoVertex>,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct GizmoVertex {
    pub pos: Vec3,
    pub color: Rgb,
}

pub fn new_frame() -> RisResult<()> {
    if let Some(ref mut gizmos) = *GIZMOS.lock()? {
        gizmos.to_draw.clear();
    }

    Ok(())
}

pub fn segment(start: Vec3, end: Vec3, color: Rgb) -> RisResult<()> {
    let Some(ref mut gizmos) = *GIZMOS.lock()? else {
        return Ok(());
    };

    gizmos.add_segment(start, end, color);

    Ok(())
}

pub fn point(position: Vec3, color: Option<Rgb>) -> RisResult<()> {
    let Some(ref mut gizmos) = *GIZMOS.lock()? else {
        return Ok(());
    };

    panic!("not implemented")
}

pub fn view_point(position: Vec3, rotation: Quat, color: Option<Rgb>) -> RisResult<()> {
    let Some(ref mut gizmos) = *GIZMOS.lock()? else {
        return Ok(());
    };

    panic!("not implemented")
}

pub fn aabb(min: Vec3, max: Vec3, color: Option<Rgb>) -> RisResult<()> {
    let Some(ref mut gizmos) = *GIZMOS.lock()? else {
        return Ok(());
    };

    let red = color.unwrap_or(ris_math::color::RGB_RED);
    let green = color.unwrap_or(ris_math::color::RGB_GREEN);
    let blue = color.unwrap_or(ris_math::color::RGB_BLUE);
    let cyan = color.unwrap_or(ris_math::color::RGB_CYAN);
    let magenta = color.unwrap_or(ris_math::color::RGB_MAGENTA);
    let yellow = color.unwrap_or(ris_math::color::RGB_YELLOW);

    let v0 = Vec3(min.x(), min.y(), min.z());
    let v1 = Vec3(max.x(), min.y(), min.z());
    let v2 = Vec3(min.x(), max.y(), min.z());
    let v3 = Vec3(max.x(), max.y(), min.z());
    let v4 = Vec3(min.x(), min.y(), max.z());
    let v5 = Vec3(max.x(), min.y(), max.z());
    let v6 = Vec3(min.x(), max.y(), max.z());
    let v7 = Vec3(max.x(), max.y(), max.z());

    gizmos.add_segment(v1, v5, red);
    gizmos.add_segment(v3, v7, red);
    gizmos.add_segment(v2, v3, green);
    gizmos.add_segment(v6, v7, green);
    gizmos.add_segment(v4, v6, blue);
    gizmos.add_segment(v5, v7, blue);
    gizmos.add_segment(v0, v4, cyan);
    gizmos.add_segment(v2, v6, cyan);
    gizmos.add_segment(v0, v1, magenta);
    gizmos.add_segment(v4, v5, magenta);
    gizmos.add_segment(v0, v2, yellow);
    gizmos.add_segment(v1, v3, yellow);

    Ok(())
}

pub fn obb(center: Vec3, half_scale: Vec3, rotation: Quat, color: Option<Rgb>) -> RisResult<()> {
    let Some(ref mut gizmos) = *GIZMOS.lock()? else {
        return Ok(());
    };

    Ok(())
}

pub fn text(position: Vec3, text: &str) -> RisResult<()> {
    let Some(ref mut gizmos) = *GIZMOS.lock()? else {
        return Ok(());
    };

    Ok(())
}

pub fn draw() -> RisResult<Vec<GizmoVertex>> {
    let vertices = match *GIZMOS.lock()? {
        Some(ref mut gizmos) => gizmos.to_draw.clone(),
        None => Vec::new(),
    };

    Ok(vertices)
}

impl Gizmos {
    fn add_segment(&mut self, start: Vec3, end: Vec3, color: Rgb) {
        let v0 = GizmoVertex {
            pos: start,
            color,
        };
        let v1 = GizmoVertex {
            pos: end,
            color,
        };

        self.to_draw.push(v0);
        self.to_draw.push(v1);
    }
}
