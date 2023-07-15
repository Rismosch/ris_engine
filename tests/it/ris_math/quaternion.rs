use ris_math::matrix4x4::Matrix4x4;
use ris_math::quaternion::Quaternion;
use ris_rng::rng::Rng;
use ris_util::testing;

#[test]
fn should_normalize_quaternion(){
    let rng = std::rc::Rc::new(std::cell::RefCell::new(Rng::new().unwrap()));
    testing::repeat(1_000_000, move || {
        let w = rng.borrow_mut().next_f();
        let x = rng.borrow_mut().next_f();
        let y = rng.borrow_mut().next_f();
        let z = rng.borrow_mut().next_f();

        let quaternion = Quaternion {
            w,
            x,
            y,
            z,
        };

        let normalized_quaternion = quaternion.normalized();
        let expected_magnitude = 1.;
        let actual_magnitude = normalized_quaternion.magnitude();

        assert!(ris_math::f_eq(expected_magnitude, actual_magnitude));
    });
}

#[test]
fn should_convert_quaternion_to_matrix_and_back(){
    let rng = std::rc::Rc::new(std::cell::RefCell::new(Rng::new().unwrap()));
    testing::repeat(1_000_000, move || {
        let w = rng.borrow_mut().next_f();
        let x = rng.borrow_mut().next_f();
        let y = rng.borrow_mut().next_f();
        let z = rng.borrow_mut().next_f();

        let quaternion = Quaternion {
            w,
            x,
            y,
            z,
        }.normalized();

        let matrix = Matrix4x4::from_quaternion(quaternion);
        let copy = Quaternion::from_matrix(matrix);

        assert!(ris_math::f_eq(quaternion.w, copy.w));
        assert!(ris_math::f_eq(quaternion.x, copy.x));
        assert!(ris_math::f_eq(quaternion.y, copy.y));
        assert!(ris_math::f_eq(quaternion.z, copy.z));
    });
}
