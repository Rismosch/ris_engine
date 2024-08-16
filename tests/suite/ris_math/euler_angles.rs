use ris_rng::rng::Rng;
use ris_rng::rng::Seed;
use ris_util::assert_quat_eq;
use ris_util::testing;
use ris_util::testing::miri_choose;

#[test]
fn should_convert() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = std::rc::Rc::new(std::cell::RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 100), move |_| {
        let mut rng = rng.borrow_mut();

        let q = rng.next_rot();
        let euler = ris_math::euler_angles::from(q);
        let q_ = ris_math::euler_angles::to_quat(euler);

        assert_quat_eq!(q, q_);
    });
}
