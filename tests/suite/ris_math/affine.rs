use std::cell::RefCell;
use std::rc::Rc;

use ris_math::affine;
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

    testing::repeat(miri_choose(1_000_000, 100), move |_| {
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
        let s = rng.next_dir_3();

        let m = affine::trs_compose(t, r, s);
        let (t_, r_, s_) = affine::trs_decompose(m);

        assert_vec3_eq!(t, t_);
        assert_quat_eq!(r, r_);
        assert_vec3_eq!(s, s_);
    });
}

