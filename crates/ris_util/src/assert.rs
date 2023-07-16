pub fn feq(left: f32, right: f32, tolerance: f32) {
    let diff = ris_math::diff(left, right);
    assert!(diff < tolerance, "expected {} and {} to be within {}, but differed by {}", left, right, tolerance, diff);
}
