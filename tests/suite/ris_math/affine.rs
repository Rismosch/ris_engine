use std::cell::RefCell;
use std::rc::Rc;

use ris_math::affine;
use ris_math::vector::Vec3;
use ris_math::vector::Vec4;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;
use ris_util::assert_feq;
use ris_util::assert_quat_feq;
use ris_util::assert_vec3_feq;
use ris_util::testing;
use ris_util::testing::miri_choose;

#[test]
fn should_convert_translation() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 100), move |_| {
        let mut rng = rng.borrow_mut();

        let t = rng.next_pos_3();

        let m = affine::from_translation(t);
        let t_ = affine::to_translation(m);

        assert_vec3_feq!(t, t_);
    });
}

#[test]
fn should_convert_rotation() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 100), move |_| {
        let mut rng = rng.borrow_mut();

        let r = rng.next_rot();

        let m = affine::from_rotation(r);
        let r_ = affine::to_rotation(m);

        assert_quat_feq!(r, r_);
    });
}

#[test]
fn should_convert_scale() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 100), move |_| {
        let mut rng = rng.borrow_mut();

        let s = rng.next_pos_3();

        let m = affine::from_scale(s);
        let s_ = affine::to_scale(m);

        assert_vec3_feq!(s, s_);
    });
}

#[test]
fn should_convert_trs() {
    let seed = Seed::new();
    //let seed = Seed(189219585137655393239067829489766350996);
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(500_000, 50), move |_| {
        let mut rng = rng.borrow_mut();

        let t = rng.next_pos_3();
        let r = rng.next_rot();
        let s = rng.next_pos_3();

        let m = affine::trs(t, r, s);
        let affine::DecomposedTrs{
            translation: t_,
            rotation: r_,
            scale: s_,
            skew: _,
        } = affine::decompose_trs(m);
        let m_ = affine::trs(t_, r_, s_);

        // a negative scale might flip the coordinate system, because of
        // that we might get different trs values out of decomposition 
        // that we got in. this is fine however, as different 
        // transformation may lead to the same results. thus this test 
        // only tests if the decomposed trs produces the same 
        // transformation, not if it has the same values
        for _ in 0..2 {
            let p = rng.next_pos_3();
            let p = Vec4(p.0, p.1, p.2, 0.0);

            let q = m * p;
            let q_ = m_ * p;

            let q = Vec3(q.0, q.1, q.2);
            let q_ = Vec3(q_.0, q_.1, q_.2);

            // any scale coordinate being 0 leads to a division by 0
            // internally. this is a special edgecase which was not
            // handled for simplicity reasons
            let has_zero_scale = s.equal(Vec3::init(0.0)).any();
            if has_zero_scale {
                assert!(q_.is_nan().all());
            } else {
                assert_vec3_feq!(q, q_,);
            }
        }
    });

}

