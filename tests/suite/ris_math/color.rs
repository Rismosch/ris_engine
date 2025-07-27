use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;

use ris_math::color;
use ris_math::color::ByteColor3;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;
use ris_util::assert_bytes_eq;
use ris_util::assert_feq;
use ris_util::testing;
use ris_util::testing::miri_choose;

#[test]
fn should_convert_rgb_to_lab() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));
    testing::repeat(miri_choose(1_000_000, 100), move |_| {
        let mut rng = rng.borrow_mut();

        let r = rng.next_f32_between(0., 1.);
        let g = rng.next_f32_between(0., 1.);
        let b = rng.next_f32_between(0., 1.);

        let rgb = color::Rgb(r, g, b);
        let lab: color::OkLab = rgb.into();
        let rgb_: color::Rgb = lab.into();
        let lab_: color::OkLab = rgb_.into();

        assert_feq!(rgb.r(), rgb_.r(), color::MIN_NORM);
        assert_feq!(rgb.g(), rgb_.g(), color::MIN_NORM);
        assert_feq!(rgb.b(), rgb_.b(), color::MIN_NORM);
        assert_feq!(lab.l(), lab_.l(), color::MIN_NORM);
        assert_feq!(lab.a(), lab_.a(), color::MIN_NORM);
        assert_feq!(lab.b(), lab_.b(), color::MIN_NORM);
    });
}

#[test]
fn should_convert_lab_to_lch() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));
    testing::repeat(miri_choose(1_000_000, 100), move |_| {
        let mut rng = rng.borrow_mut();

        let l = rng.next_f32_between(0., 1.);
        let a = rng.next_f32_between(-0.5, 0.5);
        let b = rng.next_f32_between(-0.5, 0.5);

        let lab = color::OkLab(l, a, b);
        let lch: color::OkLch = lab.into();
        let lab_: color::OkLab = lch.into();
        let lch_: color::OkLch = lab_.into();

        assert_feq!(lab.l(), lab_.l(), color::MIN_NORM);
        assert_feq!(lab.a(), lab_.a(), color::MIN_NORM);
        assert_feq!(lab.b(), lab_.b(), color::MIN_NORM);
        assert_feq!(lch.l(), lch_.l(), color::MIN_NORM);
        assert_feq!(lch.c(), lch_.c(), color::MIN_NORM);
        assert_chroma_eq(lch, lch_);
    });
}

#[test]
fn should_convert_rgb_to_lch() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));
    testing::repeat(miri_choose(1_000_000, 100), move |_| {
        let mut rng = rng.borrow_mut();

        let r = rng.next_f32_between(0., 1.);
        let g = rng.next_f32_between(0., 1.);
        let b = rng.next_f32_between(0., 1.);

        let rgb = color::Rgb(r, g, b);
        let lch: color::OkLch = rgb.into();
        let rgb_: color::Rgb = lch.into();
        let lch_: color::OkLch = rgb_.into();

        assert_feq!(rgb.r(), rgb_.r(), color::MIN_NORM);
        assert_feq!(rgb.g(), rgb_.g(), color::MIN_NORM);
        assert_feq!(rgb.b(), rgb_.b(), color::MIN_NORM);
        assert_feq!(lch.l(), lch_.l(), color::MIN_NORM);
        assert_feq!(lch.c(), lch_.c(), color::MIN_NORM);

        // if the color is white, hue becomes lost when converting lch -> rgb -> lch
        // thus we cannot possibly expect the hue to stay the same after conversion, and assert
        // only when the color is not white
        let t = 1.0 - color::MIN_NORM;
        if rgb.r() < t || rgb.g() < t || rgb.b() < t {
            assert_chroma_eq(lch, lch_);
        }
    });
}

#[test]
fn should_convert_rgb_to_bytes() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));
    testing::repeat(miri_choose(1_000_000, 100), move |_| {
        let mut rng = rng.borrow_mut();

        let r = rng.next_f32_between(0., 1.);
        let g = rng.next_f32_between(0., 1.);
        let b = rng.next_f32_between(0., 1.);

        let rgb = color::Rgb(r, g, b);
        let bytes = rgb.to_bytes();
        let rgb_ = color::Rgb::from_bytes(bytes);
        let bytes_ = rgb_.to_bytes();

        assert_feq!(rgb.r(), rgb_.r(), color::MIN_NORM);
        assert_feq!(rgb.g(), rgb_.g(), color::MIN_NORM);
        assert_feq!(rgb.b(), rgb_.b(), color::MIN_NORM);
        assert_bytes_eq!(bytes, bytes_);
    });
}

#[test]
fn should_clamp_when_converting_bytes_to_rgb() {
    let bytes = color::Rgb(-1.0, 2.0, 0.5).to_bytes();
    assert_bytes_eq!(bytes, [0, 255, 127]);
}

fn assert_chroma_eq(left: color::OkLch, right: color::OkLch) {
    let c_left = left.c();
    let c_right = right.c();

    let diff = f32::abs(c_left - c_right);
    if diff < color::MIN_NORM {
        // success, hue is identical
    } else {
        // if diff is 2 * pi, then it is the same hue, because hue is mod 2 * pi
        assert_feq!(diff, 2.0 * PI, color::MIN_NORM, "{:?} {:?}", left, right,);
    }
}
