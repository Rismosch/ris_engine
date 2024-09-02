use ris_math::quaternion::Quat;
use ris_math::vector::Vec4;
use ris_util::assert_quat_eq;

#[test]
fn should_assert_quat_eq() {
    let q1 = Quat::identity();
    let q2 = Quat(1.0, 2.0, 3.0, 4.0).normalize();
    let q2_ = Quat::from(Vec4::from(q2) * -1.0);
    let q3 = Quat(5.0, 0.0, -7.0, 0.0).normalize();
    let q3_ = Quat::from(Vec4::from(q3) * -1.0);

    assert_quat_eq!(q1, q1);
    assert_quat_eq!(q2, q2);
    assert_quat_eq!(q2_, q2_);
    assert_quat_eq!(q3, q3);
    assert_quat_eq!(q3_, q3_);

    assert_quat_eq!(q2, q2_);
    assert_quat_eq!(q3, q3_);
}

#[test]
#[should_panic]
fn should_assert_quat_neq() {
    let q2 = Quat(1.0, 2.0, 3.0, 4.0).normalize();
    let q3 = Quat(5.0, 0.0, -7.0, 0.0).normalize();

    assert_quat_eq!(q2, q3);
}
