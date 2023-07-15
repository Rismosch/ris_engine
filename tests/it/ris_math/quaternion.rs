use ris_math::matrix4x4::Matrix4x4;
use ris_math::quaternion::Quaternion;
use ris_math::vector3::Vector3;
use ris_rng::rng::Rng;
use ris_util::testing;

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

        assert!(ris_math::f_eq(expected_magnitude, actual_magnitude));
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

        let matrix = Matrix4x4::from_quaternion(quaternion);
        let copy = Quaternion::from_matrix(matrix);

        assert!(ris_math::f_eq(quaternion.w, copy.w));
        assert!(ris_math::f_eq(quaternion.x, copy.x));
        assert!(ris_math::f_eq(quaternion.y, copy.y));
        assert!(ris_math::f_eq(quaternion.z, copy.z));
    });
}

#[test]
fn should_convert_angleaxis_to_quaternion_and_back() {
    let rng = std::rc::Rc::new(std::cell::RefCell::new(Rng::new().unwrap()));
    testing::repeat(1_000_000, move || {
        let angle = rng.borrow_mut().range_f(0., ris_math::PI_2);
        let x = rng.borrow_mut().next_f();
        let y = rng.borrow_mut().next_f();
        let z = rng.borrow_mut().next_f();
        let axis = Vector3{x,y,z}.normalized();

        let quaternion = Quaternion::from_angle_axis(angle, axis);
        let (angle_copy, axis_copy) = quaternion.to_angle_axis();

        assert!(ris_math::f_eq(angle, angle_copy));
        assert!(ris_math::f_eq(axis.x, axis_copy.x));
        assert!(ris_math::f_eq(axis.y, axis_copy.y));
        assert!(ris_math::f_eq(axis.z, axis_copy.z));

        oha
    });
}
