pub mod color;
pub mod matrix;
pub mod quaternion;

#[test]
fn fast_sincos_should_compute_sin_and_cos() {
    let max_error = 0.00202;

    let count = 1 << 16;
    for i in 0..count {
        let f = std::f32::consts::PI * (i as f32) / (count as f32);

        let std_sin = f.sin();
        let std_cos = f.cos();

        let (sin, cos) = ris_math::fast_sincos(f);

        ris_util::assert_feq!(std_sin, sin, max_error);
        ris_util::assert_feq!(std_cos, cos, max_error);
    }
}
