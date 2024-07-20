use std::f32::consts::PI;

// returns 1.0 if x = true, otherwise it returns 0.0
pub fn bool_to_f32(x: bool) -> f32 {
    x as usize as f32
}

/// returns x if a = true, and y is a = false. useful for branchless programming
pub fn choose(x: f32, y: f32, a: bool) -> f32 {
    crate::common::mix(y, x, bool_to_f32(a))
}

/// returns (sin(angle), cos(angle))
///
/// max error: ~ 0.00202
///
/// inspired by Kaze Emanuar: <https://youtu.be/xFKFoGiGlXQ?si=qY72yYASbnN5LS0l&t=680>
///
/// # Panics
///
/// panics if angle < 0 or angle > 2 * PI
pub fn sincos(angle: f32) -> (f32, f32) {
    debug_assert!(angle >= 0.);
    debug_assert!(angle <= 2. * PI);

    let sin_part1 = bhaskara(angle - 0.5 * PI);
    let sin_part2 = -bhaskara(angle - 1.5 * PI);

    let flipsign = angle > 0.5 * PI && angle < 1.5 * PI;
    let sign = choose(-1., 1., flipsign);

    let sin = choose(sin_part1, sin_part2, angle < PI);
    let cos = sign * f32::sqrt(1. - sin * sin);

    (sin, cos)
}

/// returns (pi^2 - 4x^2) / (pi^2 + x^2). used by fast_sincos
pub fn bhaskara(x: f32) -> f32 {
    let pi2 = PI * PI;
    let xx = x * x;
    let xx4 = xx + xx + xx + xx;
    (pi2 - xx4) / (pi2 + xx)
}

const ONE_AS_INT: i32 = 0x3f80_0000;
const SCALE_UP: f32 = 0x00800000 as f32;
const SCALE_DOWN: f32 = 1.0 / SCALE_UP;

/// uses the bytes of x to initialize an i32. used by fast functions
pub fn as_int(x: f32) -> i32 {
    #[cfg(target_endian = "big")]
    {
        i32::from_be_bytes(x.to_be_bytes())
    }

    #[cfg(target_endian = "little")]
    {
        i32::from_le_bytes(x.to_le_bytes())
    }
}
/// uses the bytes of x to initialize an f32. used by fast functions
pub fn as_float(x: i32) -> f32 {
    #[cfg(target_endian = "big")]
    {
        f32::from_be_bytes(x.to_be_bytes())
    }

    #[cfg(target_endian = "little")]
    {
        f32::from_le_bytes(x.to_le_bytes())
    }
}

#[deprecated(note = "benchmarking proofed bitmagic is not faster than std")]
/// returns abs(x)
///
/// inspired by Creel: <https://youtu.be/ReTetN51r7A?si=hSNzsPFMN_Pe5kgj&t=201>
pub fn abs(x: f32) -> f32 {
    as_float(as_int(x) & 0x7FFF_FFFF)
}

#[deprecated(note = "benchmarking proofed bitmagic is not faster than std")]
/// returns -x
///
/// inspired by Creel: <https://youtu.be/ReTetN51r7A?si=hSNzsPFMN_Pe5kgj&t=201>
///
pub fn neg(x: f32) -> f32 {
    as_float(as_int(x) ^ 0x8000_0000u32 as i32)
}

/// returns log2(x)
///
/// max error: ~ 0.09
/// most accurate around powers of 2
///
/// inspired by Creel: <https://youtu.be/ReTetN51r7A?si=hSNzsPFMN_Pe5kgj&t=293>
///
/// # Panics
///
/// panics if x <= 0
pub fn log2(x: f32) -> f32 {
    debug_assert!(x > 0.);
    (as_int(x) - ONE_AS_INT) as f32 * SCALE_DOWN
}

/// returns exp2(x)
///
/// very accurate for integers and negative floats, very inaccurate otherwise
///
/// inspired by Creel: <https://youtu.be/ReTetN51r7A?si=20RYsxxHEF5ZYZGc&t=474>
pub fn exp2(x: f32) -> f32 {
    as_float(((x * SCALE_UP) as i32).wrapping_add(ONE_AS_INT))
}


/// returns x raised to the y power, i.e., x^y
///
/// max error, if x < 1 and y < 1: ~ 0.04304
/// decently accurate when both x and y are near positive 0, **very** innacurate otherwise.
///
/// inspired by Creel: <https://youtu.be/ReTetN51r7A?si=vmcdBoVu1vAxR1hj&t=518>
pub fn pow(x: f32, y: f32) -> f32 {
    as_float(((y * (as_int(x).wrapping_sub(ONE_AS_INT)) as f32) as i32).wrapping_add(ONE_AS_INT))
}
 
#[deprecated(note = "benchmarking proofed that newton step makes this substentially slower than std. removing the newton step substentially decreased accuracy")]
/// returns sqrt(x)
///
/// max error: ~ 0.03925;
/// decently accurate, approaching max error every second power of 2, e.g 2, 8, 32, 128, 512, etc.
///
/// inspired by Creel: <https://youtu.be/ReTetN51r7A?si=vmcdBoVu1vAxR1hj&t=518>
///
/// # Panics
///
/// panics if x < 0
pub fn sqrt(x: f32) -> f32 {
    debug_assert!(x >= 0.);

    let mut y = as_float((as_int(x) >> 1) + (ONE_AS_INT >> 1));

    y = (y * y + x) / (2. * y); // newton step, repeating increases accuracy

    y
}

/// returns 1 / sqrt(x)
///
/// max error, if x < 1: ~ 0.43313
/// max error, if x > 1: ~ 0.00153
/// the further away x from 0, the more accurate this function becomes
///
/// inspired by Creel: <https://youtu.be/ReTetN51r7A?si=CX-5iUHIqXBeuxBT&t=986>
///
/// # Panics
///
/// panics if x < 0
pub fn inversesqrt(mut x: f32) -> f32 {
    debug_assert!(x >= 0.);

    let xhalf = 0.5 * x;
    let mut i = as_int(x);
    i = 0x5f375a86 - (i >> 1);
    x = as_float(i);

    x = x * (1.5 - xhalf * x * x); // newton step, repeating increases accuracy

    x
}
