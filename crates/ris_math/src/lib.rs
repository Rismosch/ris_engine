//pub mod color;
pub mod matrix;
//pub mod quaternion;
pub mod vector;

pub const MIN_NORM: f32 = 0.000_001f32;

//
// angle and trigonometry functions
// function parameters specified as _angle_ are assumed to be in units of radians
//

/// ratio of a circle's circumference to its diameter
pub const PI: f32 = std::f32::consts::PI;

/// converts degrees to radians
pub fn radians(degrees: f32) -> f32 {
    degrees * PI / 180.
}
/// converts radians to degrees
pub fn degrees(radians: f32) -> f32 {
    radians * 180. / PI
}
/// standard trigonomic sine function
pub fn sin(angle: f32) -> f32 {
    f32::sin(angle)
}
/// standard trigonomic cosine function
pub fn cos(angle: f32) -> f32 {
    f32::cos(angle)
}
/// standard trigonomic tangent
pub fn tan(angle: f32) -> f32 {
    f32::tan(angle)
}
/// arc sine. returns an angle whose sine is x. the range of values returned by this function is
/// [-PI/2,PI/2] or NaN if x is outside the range [-1,1]
pub fn asin(x: f32) -> f32 {
    f32::asin(x)
}
/// arc cosine. returns an angle whose cosine is x. the range of values returned by this function
/// is `[0,PI]` or NaN if x is outside range [-1,1]
pub fn acos(x: f32) -> f32 {
    f32::acos(x)
}
/// arc tangent. returns an angle whose tangent is y/x. the signs of x and y are used to determine
/// what quadrant the angle is in
/// - x = 0, y = 0: 0
/// - x >= 0: arctan(y/x) -> [-PI/2,PI/2]
/// - y >= 0: arctan(y/x) + PI -> (PI/2, PI]
/// - y < 0: arctan(y/x) - PI -> (-PI. -PI/2)
pub fn atan2(y: f32, x: f32) -> f32 {
    f32::atan2(y, x)
}
/// arc tangent. returns an angle whose tangent is y_over_x. the range of values returned by this
/// function is `[-PI/2, PI/2]`
pub fn atan(y_over_x: f32) -> f32 {
    f32::atan(y_over_x)
}
/// returns the hyperbolic sine function `(e^x - e^(-x)) / 2`
pub fn sinh(x: f32) -> f32 {
    f32::sinh(x)
}
/// returns the hyperbolic cosine function `(e^x + e^(-x)) / 2`
pub fn cosh(x: f32) -> f32 {
    f32::cosh(x)
}
/// returns the hyperbolic tangent function `sinh(x) / cosh(x)`
pub fn tanh(x: f32) -> f32 {
    f32::tanh(x)
}
/// arc hyperbolic sinde; returns inverse of sinh
pub fn asinh(x: f32) -> f32 {
    f32::asinh(x)
}
/// arc hyperbolic cosine; returns the non-negative inverse of cosh
pub fn acosh(x: f32) -> f32 {
    f32::acosh(x)
}
/// arc hyperbolic tangent; returns the inverse of tanh
pub fn atanh(x: f32) -> f32 {
    f32::atanh(x)
}

//
// exponential functions
//

/// returns x raised to the y power, i.e., x^y
pub fn pow(x: f32, y: f32) -> f32 {
    f32::powf(x, y)
}
/// returns the natural exponentiation of x, i.e., e^x
pub fn exp(x: f32) -> f32 {
    f32::exp(x)
}
/// returns the natural logarithm of x, i.e., returns the value y, which satisfies the equation x =
/// e^y
pub fn log(x: f32) -> f32 {
    f32::ln(x)
}
/// returns 2 raised to the x powers, i.e., 2^x
pub fn exp2(x: f32) -> f32 {
    f32::exp2(x)
}
/// returns the base 2 logarithm of x, i.e., returns the value y which satisfies the equation x =
/// 2^y
pub fn log2(x: f32) -> f32 {
    f32::log2(x)
}
/// returns the square root of x
pub fn sqrt(x: f32) -> f32 {
    f32::sqrt(x)
}
/// returns the cube root (or third root) of x
pub fn cbrt(x: f32) -> f32 {
    f32::cbrt(x)
}
/// returns the inverse square root, i.e., 1 / sqrt(x)
pub fn inversesqrt(x: f32) -> f32 {
    1. / sqrt(x)
}

//
// common functions
//

/// returns x is x >= 0, otherwise it returns -x
pub fn abs(x: f32) -> f32 {
    f32::abs(x)
}
/// returns 1.0 if x > 0, 0.0 of x = 0, or -1.0 if x < 0
pub fn sign(x: f32) -> f32 {
    if x == 0. {
        0.
    } else if x > 0. {
        1.
    } else {
        -1.
    }
}
/// returns a value equal to the nearest integer that is less than or equal to x
pub fn floor(x: f32) -> f32 {
    f32::floor(x)
}
/// returns a value equal to the nearest integer that is greater than or equal to x
pub fn ceil(x: f32) -> f32 {
    f32::ceil(x)
}
/// returns a value equal to the nearest integer to x whose absolute value is not larger than the
/// absolute value of x. in plain english: it rounds towards zero. for positive numbers it acts
/// like floor, for negative numbers it acts like ceil
pub fn trunc(x: f32) -> f32 {
    f32::trunc(x)
}
/// returns a value equal to the nearest integer to x.  the fraction 0.5 will round away from 0.0
pub fn round(x: f32) -> f32 {
    f32::round(x)
}
/// returns the fractional part of x, i.e., x - floor(x)
pub fn fract(x: f32) -> f32 {
    x - floor(x)
}
/// modulus. returns x - y * floor(x/y)
pub fn modulo(x: f32, y: f32) -> f32 {
    x - y * floor(x/y)
}
/// returns y if y < x, otherwise it returns x
pub fn min(x: f32, y: f32) -> f32 {
    if y < x {
        y
    } else {
        x
    }
}
/// returns y if x < y, otherwise it returns x
pub fn max(x: f32, y: f32) -> f32 {
    if x < y {
        y
    } else {
        x
    }
}
/// returns min(max(x, min_val), max_val)
pub fn clamp(x: f32, min_val: f32, max_val: f32) -> f32 {
    min(max(x, min_val), max_val)
}
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
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0., 1.);
    t * t * (3. - 2. * t)
}
/// returns 0.0 if x <= edge0 and 1.0 if x >= edge1 and performs smooth Hermite interpolation
/// between 0 and 1, when edge0 < x < edge1. this is is a smoother version of smoothstep, as
/// smoothstep is only C1 continuous, while smoothstep is C2 continuous
pub fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0., 1.);
    t * t * t * (t * (6. * t - 15.) + 10.)
}
/// returns true if x holds a NaN. returns false otherwise
pub fn is_nan(x: f32) -> bool {
    f32::is_nan(x)
}
/// returns true if x holds a positive infinity or negative infinity. returns false otherwise
pub fn is_inf(x: f32) -> bool {
    f32::is_infinite(x)
}

