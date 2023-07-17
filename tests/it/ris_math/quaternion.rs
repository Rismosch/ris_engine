use ris_math::matrix4x4::Matrix4x4;
use ris_math::quaternion::Quaternion;
use ris_math::vector3::{self, Vector3};
use ris_rng::rng::Rng;
use ris_util::testing;
use ris_util::testing::assert_feq;

#[test]
fn should_normalize_quaternion() {
    let rng = std::rc::Rc::new(std::cell::RefCell::new(Rng::new().unwrap()));
    testing::repeat(1_000_000, move || {
        let w = rng.borrow_mut().next_f();
        let x = rng.borrow_mut().next_f();
        let y = rng.borrow_mut().next_f();
        let z = rng.borrow_mut().next_f();

        let quaternion = Quaternion { w, x, y, z };

        let normalized_quaternion = quaternion.normalized();
        let expected_magnitude = 1.;
        let actual_magnitude = normalized_quaternion.magnitude();

        assert_feq(expected_magnitude, actual_magnitude, ris_math::MIN_NORM);
    });
}

#[test]
fn should_convert_quaternion_to_matrix_and_back() {
    let rng = std::rc::Rc::new(std::cell::RefCell::new(Rng::new().unwrap()));
    testing::repeat(1_000_000, move || {
        let w = rng.borrow_mut().next_f();
        let x = rng.borrow_mut().next_f();
        let y = rng.borrow_mut().next_f();
        let z = rng.borrow_mut().next_f();

        let quaternion = Quaternion { w, x, y, z }.normalized();

        let matrix = Matrix4x4::transformation(quaternion, vector3::ZERO);
        let copy = Quaternion::from_matrix(matrix);

        assert_feq(quaternion.w, copy.w, ris_math::MIN_NORM);
        assert_feq(quaternion.x, copy.x, ris_math::MIN_NORM);
        assert_feq(quaternion.y, copy.y, ris_math::MIN_NORM);
        assert_feq(quaternion.z, copy.z, ris_math::MIN_NORM);
    });
}

#[test]
fn should_convert_angleaxis_to_quaternion_and_back() {
    let rng = std::rc::Rc::new(std::cell::RefCell::new(Rng::new().unwrap()));
    testing::repeat(1_000_000, move || {
        let angle = rng.borrow_mut().range_f(0.1, ris_math::PI_2 - 0.1);
        let x = rng.borrow_mut().next_f();
        let y = rng.borrow_mut().next_f();
        let z = rng.borrow_mut().next_f();
        let axis = Vector3 { x, y, z }.normalized();

        let quaternion = Quaternion::from_angle_axis(angle, axis);
        let (angle_copy, axis_copy) = quaternion.to_angle_axis();

        assert_feq(angle, angle_copy, 0.000_1);
        assert_feq(axis.x, axis_copy.x, 0.000_1);
        assert_feq(axis.y, axis_copy.y, 0.000_1);
        assert_feq(axis.z, axis_copy.z, 0.000_1);
    });
}

#[test]
fn should_convert_angleaxis_to_quaternion_at_angle_0() {
    let mut rng = Rng::new().unwrap();
    let angle = 0.;
    let x = rng.next_f();
    let y = rng.next_f();
    let z = rng.next_f();
    let axis = Vector3 { x, y, z }.normalized();

    let quaternion = Quaternion::from_angle_axis(angle, axis);
    let (angle_copy, axis_copy) = quaternion.to_angle_axis();

    assert_feq(angle, angle_copy, ris_math::MIN_NORM);
    assert_feq(axis_copy.x, 1., ris_math::MIN_NORM);
    assert_feq(axis_copy.y, 0., ris_math::MIN_NORM);
    assert_feq(axis_copy.z, 0., ris_math::MIN_NORM);
}

#[test]
fn should_convert_angleaxis_to_quaternion_at_angle_2pi() {
    let mut rng = Rng::new().unwrap();
    let angle = ris_math::PI_2;
    let x = rng.next_f();
    let y = rng.next_f();
    let z = rng.next_f();
    let axis = Vector3 { x, y, z }.normalized();

    let quaternion = Quaternion::from_angle_axis(angle, axis);
    let (angle_copy, axis_copy) = quaternion.to_angle_axis();

    assert_feq(angle, angle_copy, ris_math::MIN_NORM);
    assert_feq(axis_copy.x, 1., ris_math::MIN_NORM);
    assert_feq(axis_copy.y, 0., ris_math::MIN_NORM);
    assert_feq(axis_copy.z, 0., ris_math::MIN_NORM);
}
