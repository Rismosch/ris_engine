use std::f32::consts::PI;

use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;
use ris_util::assert_feq;
use ris_util::testing;
use ris_util::testing::miri_choose;

#[test]
fn should_normalize_quaternion() {
    let rng = std::rc::Rc::new(std::cell::RefCell::new(Rng::new(Seed::new())));
    testing::repeat(miri_choose(1_000_000, 100), move |_| {
        let quaternion = rng.borrow_mut().next_rot();

        let normalized_quaternion = quaternion.normalize();
        let expected_magnitude = 1.;
        let actual_magnitude = normalized_quaternion.length();

        assert_feq!(expected_magnitude, actual_magnitude);
    });
}

#[test]
fn should_convert_angleaxis_to_quaternion_at_angle_0() {
    let mut rng = Rng::new(Seed::new());
    let angle = 0.0;
    let axis = rng.next_dir_3();

    let quaternion = Quat::from((angle, axis));
    let (angle_copy, axis_copy) = quaternion.into();

    assert_feq!(angle, angle_copy);
    assert_feq!(axis_copy.x(), 1.);
    assert_feq!(axis_copy.y(), 0.);
    assert_feq!(axis_copy.z(), 0.);
}

#[test]
fn should_convert_angleaxis_to_quaternion_at_angle_2pi() {
    let mut rng = Rng::new(Seed::new());
    let angle = 2.0 * PI;
    let axis = rng.next_dir_3();

    let quaternion = Quat::from((angle, axis));
    let (angle_copy, axis_copy) = quaternion.into();

    assert_feq!(angle, angle_copy);
    assert_feq!(axis_copy.x(), 1.);
    assert_feq!(axis_copy.y(), 0.);
    assert_feq!(axis_copy.z(), 0.);
}

#[test]
fn should_rotate_around_up() {
    let rotation = Quat::from((0.25 * PI, Vec3::up()));
    let result = rotation.rotate(Vec3::forward());
    assert!(result.x() < 0.);
    assert!(result.y() > 0.);
    assert!(result.z() == 0.);
}

#[test]
fn should_rotate_around_down() {
    let rotation = Quat::from((0.25 * PI, Vec3::down()));
    let result = rotation.rotate(Vec3::forward());
    assert!(result.x() > 0.);
    assert!(result.y() > 0.);
    assert!(result.z() == 0.);
}

#[test]
fn should_rotate_around_left() {
    let rotation = Quat::from((0.25 * PI, Vec3::left()));
    let result = rotation.rotate(Vec3::forward());
    assert!(result.x() == 0.);
    assert!(result.y() > 0.);
    assert!(result.z() < 0.);
}

#[test]
fn should_rotate_around_right() {
    let rotation = Quat::from((0.25 * PI, Vec3::right()));
    let result = rotation.rotate(Vec3::forward());
    assert!(result.x() == 0.);
    assert!(result.y() > 0.);
    assert!(result.z() > 0.);
}

#[test]
fn should_rotate_around_forward() {
    let rotation = Quat::from((0.25 * PI, Vec3::forward()));
    let result = rotation.rotate(Vec3::forward());
    assert!(result.x() == 0.);
    assert!(result.y() == 1.);
    assert!(result.z() == 0.);
}

#[test]
fn should_rotate_around_backward() {
    let rotation = Quat::from((0.25 * PI, Vec3::backward()));
    let result = rotation.rotate(Vec3::forward());
    assert!(result.x() == 0.);
    assert!(result.y() == 1.);
    assert!(result.z() == 0.);
}
