use ris_math::matrix4x4::Matrix4x4;
use ris_math::quaternion::Quaternion;
use ris_math::vector3::Vector3;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;
use ris_util::assert_feq;
use ris_util::testing;
use ris_util::testing::miri_choose;

#[test]
fn should_rotate_like_quaternion() {
    let rng = std::rc::Rc::new(std::cell::RefCell::new(Rng::new(Seed::new().unwrap())));
    testing::repeat(miri_choose(1_000_000, 100), move |_| {
        let w = rng.borrow_mut().next_f();
        let x = rng.borrow_mut().next_f();
        let y = rng.borrow_mut().next_f();
        let z = rng.borrow_mut().next_f();
        let quaternion = Quaternion { w, x, y, z }.normalized();
        let matrix = Matrix4x4::rotation(quaternion);

        let x = rng.borrow_mut().next_f();
        let y = rng.borrow_mut().next_f();
        let z = rng.borrow_mut().next_f();
        let vector = Vector3 { x, y, z }.normalized();

        let rotated_by_quaternion = quaternion.rotate(vector);
        let rotated_by_matrix = matrix.rotate(vector);

        assert_feq!(
            rotated_by_quaternion.x,
            rotated_by_matrix.x,
            ris_math::MIN_NORM
        );
        assert_feq!(
            rotated_by_quaternion.y,
            rotated_by_matrix.y,
            ris_math::MIN_NORM
        );
        assert_feq!(
            rotated_by_quaternion.z,
            rotated_by_matrix.z,
            ris_math::MIN_NORM
        );
    });
}
