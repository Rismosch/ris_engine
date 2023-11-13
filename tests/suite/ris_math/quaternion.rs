use ris_math::quaternion::Quaternion;
use ris_math::vector3::{self, Vector3};
use ris_rng::rng;
use ris_rng::rng::Rng;
use ris_util::testing;
use ris_util::testing::assert_feq;

#[test]
fn should_normalize_quaternion() {
    let rng = std::rc::Rc::new(std::cell::RefCell::new(Rng::new(rng::CONST_SEED)));
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
fn should_convert_angleaxis_to_quaternion_at_angle_0() {
    let mut rng = Rng::new(rng::CONST_SEED);
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
    let mut rng = Rng::new(rng::CONST_SEED);
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

#[test]
fn should_rotate_around_up() {
    let rotation = Quaternion::from_angle_axis(ris_math::PI_0_25, vector3::UP);
    let result = rotation.rotate(vector3::FORWARD);
    assert!(result.x < 0.);
    assert!(result.y > 0.);
    assert!(result.z == 0.);
}

#[test]
fn should_rotate_around_down() {
    let rotation = Quaternion::from_angle_axis(ris_math::PI_0_25, vector3::DOWN);
    let result = rotation.rotate(vector3::FORWARD);
    assert!(result.x > 0.);
    assert!(result.y > 0.);
    assert!(result.z == 0.);
}

#[test]
fn should_rotate_around_left() {
    let rotation = Quaternion::from_angle_axis(ris_math::PI_0_25, vector3::LEFT);
    let result = rotation.rotate(vector3::FORWARD);
    assert!(result.x == 0.);
    assert!(result.y > 0.);
    assert!(result.z < 0.);
}

#[test]
fn should_rotate_around_right() {
    let rotation = Quaternion::from_angle_axis(ris_math::PI_0_25, vector3::RIGHT);
    let result = rotation.rotate(vector3::FORWARD);
    assert!(result.x == 0.);
    assert!(result.y > 0.);
    assert!(result.z > 0.);
}

#[test]
fn should_rotate_around_forward() {
    let rotation = Quaternion::from_angle_axis(ris_math::PI_0_25, vector3::FORWARD);
    let result = rotation.rotate(vector3::FORWARD);
    assert!(result.x == 0.);
    assert!(result.y == 1.);
    assert!(result.z == 0.);
}

#[test]
fn should_rotate_around_backward() {
    let rotation = Quaternion::from_angle_axis(ris_math::PI_0_25, vector3::BACKWARD);
    let result = rotation.rotate(vector3::FORWARD);
    assert!(result.x == 0.);
    assert!(result.y == 1.);
    assert!(result.z == 0.);
}
