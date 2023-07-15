pub mod matrix4x4;
pub mod quaternion;
pub mod vector3;

pub const MIN_NORM: f32 = 0.000_001f32;
pub const DEG2RAD: f32 = std::f32::consts::PI * 2. / 360.;
pub const RAD2DEG: f32 = 1. / DEG2RAD;

pub fn f_eq(a: f32, b: f32) -> bool {
    abs(a - b) < MIN_NORM
}

pub fn sin(f: f32) -> f32 {
    f32::sin(f)
}
pub fn cos(f: f32) -> f32 {
    f32::cos(f)
}
pub fn tan(f: f32) -> f32 {
    f32::tan(f)
}
pub fn asin(f: f32) -> f32 {
    f32::asin(f)
}
pub fn acos(f: f32) -> f32 {
    f32::acos(f)
}
pub fn atan(f: f32) -> f32 {
    f32::atan(f)
}
pub fn atan2(x: f32, y: f32) -> f32 {
    f32::atan2(y, x)
}

pub fn sqrt(f: f32) -> f32 {
    f32::sqrt(f)
}

pub fn abs(f: f32) -> f32 {
    f32::abs(f)
}
pub fn sign(f: f32) -> f32 {
    if f >= 0.0 {
        1.
    } else {
        -1.
    }
}
pub fn min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}
pub fn max(a: f32, b: f32) -> f32 {
    if a < b {
        b
    } else {
        a
    }
}

pub fn clamp(f: f32, min: f32, max: f32) -> f32 {
    if f < min {
        min
    } else if f > max {
        max
    } else {
        f
    }
}

pub fn lerp(f: f32, a: f32, b: f32) -> f32 {
    a + f * (b - a)
}

pub fn smoothstep(f: f32, min: f32, max: f32) -> f32 {
    let x = clamp((f - min) / (max - min), 0., 1.);
    -2. * x * x * x + 3. * x * x
}

pub fn smootherstep(f: f32, min: f32, max: f32) -> f32 {
    let x = clamp((f - min) / (max - min), 0., 1.);
    6. * x * x * x * x * x - 15. * x * x * x * x + 10. * x * x * x
}

pub fn inverse_smoothstep(f: f32) -> f32 {
    0.5 - sin(asin(1. - 2. * f) / 3.)
}
