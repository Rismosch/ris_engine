use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use ris_math::affine;
use ris_math::matrix::Mat4;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;
use ris_util::assert_feq;
use ris_util::assert_quat_eq;
use ris_util::assert_vec3_eq;
use ris_util::testing;
use ris_util::testing::miri_choose;

#[test]
fn should_convert_translation() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 100), move |_| {
        let mut rng = rng.borrow_mut();

        let t = rng.next_pos_3();

        let m = affine::from_translation(t);
        let t_ = affine::to_translation(m);

        assert_vec3_eq!(t, t_);
    });
}

#[test]
fn should_convert_rotation() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 100), move |i| {
        let mut rng = rng.borrow_mut();

        let r = rng.next_rot();

        let m = affine::from_rotation(r);
        let r_ = affine::to_rotation(m);

        assert_quat_eq!(r, r_);
    });
}

#[test]
fn should_convert_scale() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 100), move |_| {
        let mut rng = rng.borrow_mut();

        let s = rng.next_pos_3();

        let m = affine::from_scale(s);
        let s_ = affine::to_scale(m);

        assert_vec3_eq!(s, s_);
    });
}

#[test]
fn should_convert_trs() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 100), move |_| {
        let mut rng = rng.borrow_mut();

        let t = rng.next_pos_3();
        let r = rng.next_rot();
        let s = rng.next_f();

        let m = affine::trs(t, r, s);
        let (t_, r_, s_) = affine::decompose_trs(m);

        assert_vec3_eq!(t, t_);
        assert_quat_eq!(r, r_);
        assert_feq!(s, s_);
    });
}

#[test]
#[should_panic]
fn should_panic_when_scale_is_negative() {
    let t = Vec3::default();
    let r = Quat::default();
    let s = -1.0;

    let _ = affine::trs(t, r, s);
}

#[test]
#[should_panic]
fn should_panic_when_scale_is_zero() {
    let t = Vec3::default();
    let r = Quat::default();
    let s = 0.0;

    let _ = affine::trs(t, r, s);
}
