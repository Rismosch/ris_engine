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
use ris_util::testing;
use ris_util::testing::miri_choose;

#[test]
fn should_convert_translation() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 100), move |_| {
        let mut rng = rng.borrow_mut();

        let pos = rng.next_pos_3();

        let mat = affine::translation(pos);
        let pos_ = affine::get_translation(mat);

        assert_feq!(pos.0, pos_.0);
        assert_feq!(pos.1, pos_.1);
        assert_feq!(pos.2, pos_.2);
    });
}

#[test]
fn should_convert_rotation() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 100), move |_| {
        let mut rng = rng.borrow_mut();

        let rot = rng.next_rot();

        let mat = affine::rotation(rot);
        let rot_ = affine::get_rotation(mat);

        if f32::signum(rot.0) != f32::signum(rot_.0) {
            // a quaternion with each component negated represents the same rotation
            assert_feq!(rot.0, -rot_.0);
            assert_feq!(rot.1, -rot_.1);
            assert_feq!(rot.2, -rot_.2);
            assert_feq!(rot.3, -rot_.3);
        } else {
            assert_feq!(rot.0, rot_.0);
            assert_feq!(rot.1, rot_.1);
            assert_feq!(rot.2, rot_.2);
            assert_feq!(rot.3, rot_.3);
        }

    });
}

#[test]
fn should_convert_scale() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 100), move |_| {
        let mut rng = rng.borrow_mut();

        let scale = rng.next_pos_3();

        let mat = affine::scale(scale);
        let scale_ = affine::get_scale(mat);

        assert_feq!(scale.0, scale_.0);
        assert_feq!(scale.1, scale_.1);
        assert_feq!(scale.2, scale_.2);
    });
}

#[test]
fn should_convert_trs() {
    panic!()
}
