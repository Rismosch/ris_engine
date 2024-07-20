/// returns the linear blend of x and y, i.e., x * (1 - a) + y * a
pub fn mix(x: f32, y: f32, a: f32) -> f32 {
    x * (1. - a) + y * a
}

/// returns 0.0 if x < edge, otherwise it returns 1.0
pub fn step(edge: f32, x: f32) -> f32 {
    if x < edge {
        0.0
    } else {
        1.0
    }
}

/// returns 0.0 if x <= edge0 and 1.0 if x >= edge1 and performs smooth Hermite interpolation
/// between 0 and 1, when edge0 < x < edge1. this is useful in cases where you would want a
/// threshold function with a smooth transition
pub fn smoothstep(edge0: f32, edge1: f32, mut x: f32) -> f32 {
    x = (x - edge0) / (edge1 - edge0);
    x = x.clamp(0., 1.);
    x * x * (3. - 2. * x)
}

/// returns 0.0 if x <= edge0 and 1.0 if x >= edge1 and performs smooth Hermite interpolation
/// between 0 and 1, when edge0 < x < edge1. this is a smoother version of smoothstep, as
/// smoothstep is only C1 continuous, while smoothstep is C2 continuous
pub fn smootherstep(edge0: f32, edge1: f32, mut x: f32) -> f32 {
    x = (x - edge0) / (edge1 - edge0);
    x = x.clamp(0., 1.);
    x * x * x * (x * (6. * x - 15.) + 10.)
}

