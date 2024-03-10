pub mod color;
pub mod matrix;
pub mod quaternion;

use ris_util::testing::miri_choose;

#[test]
fn should_compute_fastsincos() {
    let max_error = 0.00202;

    let count = 1 << miri_choose(16, 4);
    for i in 0..count {
        let f = std::f32::consts::PI * (i as f32) / (count as f32);

        let std_sin = f32::sin(f);
        let std_cos = f32::cos(f);

        let (sin, cos) = ris_math::fastsincos(f);

        ris_util::assert_feq!(std_sin, sin, max_error, "value: {}", f);
        ris_util::assert_feq!(std_cos, cos, max_error, "value: {}", f);
    }
}

#[test]
fn should_compute_abs() {
    let count = 1 << miri_choose(16, 4);
    for i in 0..count {
        let f = 1000. * ((i + 1) as f32) / (count as f32);

        let std = -f;
        let fast = ris_math::fast_negate(f);

        assert_eq!(std, fast);
    }
}

#[test]
fn should_compute_negative() {
    let count = 1 << miri_choose(16, 4);
    for i in 0..count {
        let f = 1000. * ((i + 1) as f32) / (count as f32);

        let std = f32::abs(f);
        let fast = ris_math::fast_abs(f);

        assert_eq!(std, fast);
    }
}

#[test]
fn should_compute_fastlog2() {
    let max_error = 0.09;

    let count = 1 << miri_choose(16, 4);
    for i in 0..count {
        let f = 1000. * ((i + 1) as f32) / (count as f32);

        let std = f32::log2(f);
        let fast = ris_math::fastlog2(f);

        ris_util::assert_feq!(std, fast, max_error, "value: {}", f);
    }
}

#[test]
fn should_compute_fastlog2_around_powers_of_2() {
    for i in 0..32 {
        let f = f32::powi(2., i) / 16.;

        let std = f32::log2(f);
        let fast = ris_math::fastlog2(f);

        ris_util::assert_feq!(std, fast);
    }
}

#[test]
fn should_compute_fastexp2() {
    for i in miri_choose(-127..128, -16..16) {
        let f = i as f32;

        let std = f32::exp2(f);
        let fast = ris_math::fastexp2(f);

        ris_util::assert_feq!(std, fast, ris_math::MIN_NORM, "value: {}", f);
    }
}

// too inaccurate to properly test
//#[test]
//fn should_compute_fastpow() {
//    let max_error = 0.09;
//
//    let count = 1 << 8;
//    for i in 1..count {
//        for j in 1..count {
//
//            let f1 = 10. * (i as f32) / (count as f32);
//            let f2 = 10. * (j as f32) / (count as f32);
//
//            let std = f32::powf(f1, f2);
//            let fast = ris_math::fastpow(f1, f2);
//
//            //ris_util::assert_feq!(std, fast, max_error, "value: {}", f);
//        }
//    }
//}

#[test]
fn should_compute_fastsqrt() {
    let max_error = 0.03925;

    let count = 1 << miri_choose(16, 4);
    for i in 0..count {
        let f = 1000. * (i as f32) / (count as f32);

        let std = f32::sqrt(f);
        let fast = ris_math::fastsqrt(f);

        ris_util::assert_feq!(std, fast, max_error, "value: {}", f);
    }
}

#[test]
fn should_compute_fastinversesqrt() {
    let count = 1 << miri_choose(16, 4);
    for i in 0..count {
        let f = 1000. * ((i + 1) as f32) / (count as f32);

        let std = 1. / f32::sqrt(f);
        let fast = ris_math::fastinversesqrt(f);

        // the error is greater, the closer f is to 0
        let max_error = if f < 1. { 0.43313 } else { 0.00153 };

        ris_util::assert_feq!(std, fast, max_error, "value: {}", f);
    }
}
