use std::cell::RefCell;
use std::rc::Rc;

use ris_math::color;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;
use ris_util::assert_feq;
use ris_util::testing;
use ris_util::testing::miri_choose;

#[test]
fn should_convert_lab_to_rgb() {
    let rng = Rc::new(RefCell::new(Rng::new(Seed::new().unwrap())));
    testing::repeat(miri_choose(1_000_000, 100), move |_| {
        let mut rng = rng.borrow_mut();

        let r = rng.range_f(0., 1.);
        let g = rng.range_f(0., 1.);
        let b = rng.range_f(0., 1.);

        let rgb = color::Rgb { r, g, b };
        let lab: color::Lab = rgb.into();
        let rgb_: color::Rgb = lab.into();
        let lab_: color::Lab = rgb_.into();

        assert_feq!(rgb.r, rgb_.r, 0.001);
        assert_feq!(rgb.g, rgb_.g, 0.001);
        assert_feq!(rgb.b, rgb_.b, 0.001);
        assert_feq!(lab.l, lab_.l, 0.001);
        assert_feq!(lab.a, lab_.a, 0.001);
        assert_feq!(lab.b, lab_.b, 0.001);
    });
}
