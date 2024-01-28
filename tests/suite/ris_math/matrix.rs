use ris_math::matrix::Mat2x2;
use ris_math::matrix::Mat3x3;
use ris_math::matrix::Mat4x4;
use ris_math::vector::Vec2;
use ris_math::vector::Vec3;
use ris_math::vector::Vec4;
use ris_util::assert_feq;

#[test]
fn should_multiply_2_by_2() {
    let a = Mat2x2(Vec2(3., 4.), Vec2(1., 2.));
    let b = Mat2x2(Vec2(2., 1.), Vec2(3., 5.));
    let c = Mat2x2(Vec2(3., 2.), Vec2(-1., 5.));
    let d = Mat2x2(Vec2(-2., -3.), Vec2(4., 1.));
    let p = Mat2x2(Vec2(-2., 1.), Vec2(5., -3.));
    let q = Mat2x2(Vec2(4., 3.), Vec2(-2., 2.));

    let ab = a * b;
    assert_feq!(ab.0.0, 7.);
    assert_feq!(ab.1.0, 14.);
    assert_feq!(ab.0.1, 10.);
    assert_feq!(ab.1.1, 22.);

    let cd = c * d;
    assert_feq!(cd.0.0, -3.);
    assert_feq!(cd.1.0, 11.);
    assert_feq!(cd.0.1, -19.);
    assert_feq!(cd.1.1, 13.);

    let pq = p * q;
    assert_feq!(pq.0.0, 7.);
    assert_feq!(pq.1.0, 14.);
    assert_feq!(pq.0.1, -5.);
    assert_feq!(pq.1.1, -8.);

    let qp = q * p;
    assert_feq!(qp.0.0, -10.);
    assert_feq!(qp.1.0, 26.);
    assert_feq!(qp.0.1, -4.);
    assert_feq!(qp.1.1, 9.);
}

#[test]
fn should_multiply_3_by_3() {
    let a = Mat3x3(Vec3(2., 3., -4.),Vec3(0., 5., 1.),Vec3(-1., 2., 4.));
    let b = Mat3x3(Vec3(5., -1., 2.),Vec3(1., 0., -3.),Vec3(-2., 4., 3.));
    let c = Mat3x3(Vec3(3., 0., -4.),Vec3(-2., -1., 2.),Vec3(5., 6., -1.));
    let d = Mat3x3(Vec3(2., 3., 1.),Vec3(-1., -5., 4.),Vec3(0., 2., -2.));

    let ab = a * b;
    assert_feq!(ab.0.0, 8.);
    assert_feq!(ab.1.0, 5.);
    assert_feq!(ab.2.0, -7.);
    assert_feq!(ab.0.1, 14.);
    assert_feq!(ab.1.1, -3.);
    assert_feq!(ab.2.1, 20.);
    assert_feq!(ab.0.2, -13.);
    assert_feq!(ab.1.2, -16.);
    assert_feq!(ab.2.2, 24.);

    let ba = b * a;
    assert_feq!(ba.0.0, 21.);
    assert_feq!(ba.1.0, 3.);
    assert_feq!(ba.2.0, -11.);
    assert_feq!(ba.0.1, -18.);
    assert_feq!(ba.1.1, 4.);
    assert_feq!(ba.2.1, 17.);
    assert_feq!(ba.0.2, -17.);
    assert_feq!(ba.1.2, -12.);
    assert_feq!(ba.2.2, 4.);

    let cd = c * d;
    assert_feq!(cd.0.0, 5.);
    assert_feq!(cd.1.0, 27.);
    assert_feq!(cd.2.0, -14.);
    assert_feq!(cd.0.1, 3.);
    assert_feq!(cd.1.1, 29.);
    assert_feq!(cd.2.1, -14.);
    assert_feq!(cd.0.2, -3.);
    assert_feq!(cd.1.2, -10.);
    assert_feq!(cd.2.2, 6.);

    let dc = d * c;
    assert_feq!(dc.0.0, 6.);
    assert_feq!(dc.1.0, -3.);
    assert_feq!(dc.2.0, 4.);
    assert_feq!(dc.0.1, 1.);
    assert_feq!(dc.1.1, 3.);
    assert_feq!(dc.2.1, -17.);
    assert_feq!(dc.0.2, 11.);
    assert_feq!(dc.1.2, -10.);
    assert_feq!(dc.2.2, 31.);
}

#[test]
fn should_multiply_4_by_4() {
    let a = Mat4x4(Vec4(1., 1., 4., 5.),Vec4(3., 3., 3., 2.),Vec4(5., 1., 9., 0.),Vec4(9., 7., 7., 9.));
    let b = Mat4x4(Vec4(5., 0., 6., 0.),Vec4(1., 9., 1., 0.),Vec4(1., 1., 8., 1.),Vec4(0., 1., 0., 2.));
    let c = Mat4x4(Vec4(-10., 5., 10., -8.),Vec4(1., -5., -8., -4.),Vec4(-10., -5., -6., 3.),Vec4(3., 6., 8., -2.));
    let d = Mat4x4(Vec4(7., 2., -9., 10.),Vec4(0., -1., 0., -8.),Vec4(8., -10., 8., -9.),Vec4(-10., 10., 2., -4.));

    let ab = a * b;
    assert_feq!(ab.0.0, 35.);
    assert_feq!(ab.1.0, 33.);
    assert_feq!(ab.2.0, 53.);
    assert_feq!(ab.3.0, 21.);
    assert_feq!(ab.0.1, 11.);
    assert_feq!(ab.1.1, 29.);
    assert_feq!(ab.2.1, 19.);
    assert_feq!(ab.3.1, 17.);
    assert_feq!(ab.0.2, 74.);
    assert_feq!(ab.1.2, 40.);
    assert_feq!(ab.2.2, 86.);
    assert_feq!(ab.3.2, 17.);
    assert_feq!(ab.0.3, 25.);
    assert_feq!(ab.1.3, 23.);
    assert_feq!(ab.2.3, 16.);
    assert_feq!(ab.3.3, 20.);

    let ba = b * a;
    assert_feq!(ba.0.0, 10.);
    assert_feq!(ba.1.0, 21.);
    assert_feq!(ba.2.0, 35.);
    assert_feq!(ba.3.0, 59.);
    assert_feq!(ba.0.1, 18.);
    assert_feq!(ba.1.1, 32.);
    assert_feq!(ba.2.1, 18.);
    assert_feq!(ba.3.1, 79.);
    assert_feq!(ba.0.2, 39.);
    assert_feq!(ba.1.2, 45.);
    assert_feq!(ba.2.2, 103.);
    assert_feq!(ba.3.2, 117.);
    assert_feq!(ba.0.3, 14.);
    assert_feq!(ba.1.3, 7.);
    assert_feq!(ba.2.3, 9.);
    assert_feq!(ba.3.3, 25.);

    let cd = c * d;
    assert_feq!(cd.0.0, 52.);
    assert_feq!(cd.1.0, -25.);
    assert_feq!(cd.2.0, -197.);
    assert_feq!(cd.3.0, 78.);
    assert_feq!(cd.0.1, 130.);
    assert_feq!(cd.1.1, -43.);
    assert_feq!(cd.2.1, -4.);
    assert_feq!(cd.3.1, -134.);
    assert_feq!(cd.0.2, 188.);
    assert_feq!(cd.1.2, -56.);
    assert_feq!(cd.2.2, 40.);
    assert_feq!(cd.3.2, -224.);
    assert_feq!(cd.0.3, -111.);
    assert_feq!(cd.1.3, 20.);
    assert_feq!(cd.2.3, 18.);
    assert_feq!(cd.3.3, 54.);

    let dc = d * c;
    assert_feq!(dc.0.0, 90.);
    assert_feq!(dc.1.0, -17.);
    assert_feq!(dc.2.0, -148.);
    assert_feq!(dc.3.0, 105.);
    assert_feq!(dc.0.1, -205.);
    assert_feq!(dc.1.1, 47.);
    assert_feq!(dc.2.1, 75.);
    assert_feq!(dc.3.1, -100.);
    assert_feq!(dc.0.2, 154.);
    assert_feq!(dc.1.2, -81.);
    assert_feq!(dc.2.2, 48.);
    assert_feq!(dc.3.2, 33.);
    assert_feq!(dc.0.3, -198.);
    assert_feq!(dc.1.3, 138.);
    assert_feq!(dc.2.3, -18.);
    assert_feq!(dc.3.3, -82.);
}

#[test]
fn should_calculate_the_determinant_of_2_by_2() {
    let a = Mat2x2(Vec2(2., 1.), Vec2(5., -3.));
    let b = Mat2x2(Vec2(3., 2.), Vec2(-5., 1.));
    let c = Mat2x2(Vec2(-2., -6.), Vec2(4., 2.));
    let d = Mat2x2(Vec2(-3., 4.), Vec2(-1., -5.));
    let e = Mat2x2(Vec2(5., -4.), Vec2(-3., 2.));

    let da = a.determinant();
    let db = b.determinant();
    let dc = c.determinant();
    let dd = d.determinant();
    let de = e.determinant();

    assert_feq!(da, -11.);
    assert_feq!(db, 13.);
    assert_feq!(dc, 20.);
    assert_feq!(dd, 19.);
    assert_feq!(de, -2.);
}

#[test]
fn should_calculate_the_determinant_of_3_by_3() {
    let a = Mat3x3(Vec3(3., 2., -3.), Vec3(0., -5., 1.), Vec3(-1., 4., 3.));
    let b = Mat3x3(Vec3(2., 4., -5.), Vec3(-3., 2., 3.), Vec3(1., -1., -2.));
    let c = Mat3x3(Vec3(5., -1., 2.), Vec3(1., 0., -3.), Vec3(-2., 4., 3.));
    let d = Mat3x3(Vec3(2., 3., -4.), Vec3(0., 5., 1.), Vec3(-1., 2., 4.));
    let e = Mat3x3(Vec3(2., 3., 1.), Vec3(-1., -5., 4.), Vec3(0., 2., -2.));

    let da = a.determinant();
    let db = b.determinant();
    let dc = c.determinant();
    let dd = d.determinant();
    let de = e.determinant();

    assert_feq!(da, -44.);
    assert_feq!(db, -19.);
    assert_feq!(dc, 65.);
    assert_feq!(dd, 13.);
    assert_feq!(de, -4.);
}

#[test]
fn should_calculate_the_determinant_of_4_by_4() {
    let a = Mat4x4(Vec4(1., 1., 4., 5.),Vec4(3., 3., 3., 2.),Vec4(5., 1., 9., 0.),Vec4(9., 7., 7., 9.));
    let b = Mat4x4(Vec4(5., 0., 6., 0.),Vec4(1., 9., 1., 0.),Vec4(1., 1., 8., 1.),Vec4(0., 1., 0., 2.));
    let c = Mat4x4(Vec4(-10., 5., 10., -8.),Vec4(1., -5., -8., -4.),Vec4(-10., -5., -6., 3.),Vec4(3., 6., 8., -2.));
    let d = Mat4x4(Vec4(7., 2., -9., 10.),Vec4(0., -1., 0., -8.),Vec4(8., -10., 8., -9.),Vec4(-10., 10., 2., -4.));

    let da = a.determinant();
    let db = b.determinant();
    let dc = c.determinant();
    let dd = d.determinant();

    assert_feq!(da, -376.);
    assert_feq!(db, 613.);
    assert_feq!(dc, 898.);
    assert_feq!(dd, 5932.);
}

#[test]
fn should_calculate_the_inverse_of_2_by_2() {
    let assert_is_identity = |m: Mat2x2| {
        assert_feq!(m.0.0, 1.);
        assert_feq!(m.1.0, 0.);
        assert_feq!(m.0.1, 0.);
        assert_feq!(m.1.1, 1.);
    };

    let a = Mat2x2(Vec2(3., 4.), Vec2(1., 2.));
    let b = Mat2x2(Vec2(2., 1.), Vec2(3., 5.));
    let c = Mat2x2(Vec2(3., 2.), Vec2(-1., 5.));
    let d = Mat2x2(Vec2(2., 4.), Vec2(3., 5.));

    let inva = a.inverse().unwrap();
    assert_feq!(inva.0.0, 1.);
    assert_feq!(inva.1.0, -0.5);
    assert_feq!(inva.0.1, -2.);
    assert_feq!(inva.1.1, 1.5);
    assert_is_identity(inva * a);
    assert_is_identity(a * inva);

    let invb = b.inverse().unwrap();
    assert_feq!(invb.0.0, 0.71428573);
    assert_feq!(invb.1.0, -0.42857143);
    assert_feq!(invb.0.1, -0.14285715);
    assert_feq!(invb.1.1, 0.2857143);
    assert_is_identity(invb * b);
    assert_is_identity(b * invb);

    let invc = c.inverse().unwrap();
    assert_feq!(invc.0.0, 0.29411766);
    assert_feq!(invc.1.0, 0.05882353);
    assert_feq!(invc.0.1, -0.11764706);
    assert_feq!(invc.1.1, 0.1764706);
    assert_is_identity(invc * c);
    assert_is_identity(c * invc);

    let invd = d.inverse();
    assert!(invd.is_none());
}

#[test]
fn should_calculate_the_inverse_of_3_by_3() {
    let assert_is_identity = |m: Mat3x3| {
        assert_feq!(m.0.0, 1.);
        assert_feq!(m.1.0, 0.);
        assert_feq!(m.2.0, 0.);
        assert_feq!(m.0.1, 0.);
        assert_feq!(m.1.1, 1.);
        assert_feq!(m.2.1, 0.);
        assert_feq!(m.0.2, 0.);
        assert_feq!(m.1.2, 0.);
        assert_feq!(m.2.2, 1.);
    };

    let a = Mat3x3(Vec3(3., 2., -3.), Vec3(0., -5., 1.), Vec3(-1., 4., 3.));
    let b = Mat3x3(Vec3(2., 4., -5.), Vec3(-3., 2., 3.), Vec3(1., -1., -2.));
    let c = Mat3x3(Vec3(5., -1., 2.), Vec3(1., 0., -3.), Vec3(-2., 4., 3.));
    let d = Mat3x3(Vec3(1., 2., 3.), Vec3(4., 5., 6.), Vec3(7., 8., 9.));

    let inva = a.inverse().unwrap();
    assert_feq!(inva.0.0, 0.4318182);
    assert_feq!(inva.1.0, 0.022727273);
    assert_feq!(inva.2.0, 0.11363637);
    assert_feq!(inva.0.1, 0.4090909);
    assert_feq!(inva.1.1, -0.13636364);
    assert_feq!(inva.2.1, 0.3181818);
    assert_feq!(inva.0.2, 0.29545453);
    assert_feq!(inva.1.2, 0.06818182);
    assert_feq!(inva.2.2, 0.3409091);
    assert_is_identity(inva * a);
    assert_is_identity(a * inva);

    let invb = b.inverse().unwrap();
    assert_feq!(invb.0.0, 0.05263158);
    assert_feq!(invb.1.0, 0.15789473);
    assert_feq!(invb.2.0, -0.05263158);
    assert_feq!(invb.0.1, -0.68421054);
    assert_feq!(invb.1.1, -0.05263158);
    assert_feq!(invb.2.1, -0.31578946);
    assert_feq!(invb.0.2, -1.1578947);
    assert_feq!(invb.1.2, -0.47368422);
    assert_feq!(invb.2.2, -0.84210527);
    assert_is_identity(invb * b);
    assert_is_identity(b * invb);

    let invc = c.inverse().unwrap();
    assert_feq!(invc.0.0, 0.18461539);
    assert_feq!(invc.1.0, 0.046153847);
    assert_feq!(invc.2.0, 0.06153846);
    assert_feq!(invc.0.1, 0.16923077);
    assert_feq!(invc.1.1, 0.2923077);
    assert_feq!(invc.2.1, -0.2769231);
    assert_feq!(invc.0.2, 0.046153847);
    assert_feq!(invc.1.2, 0.26153848);
    assert_feq!(invc.2.2, 0.015384615);
    assert_is_identity(invc * c);
    assert_is_identity(c * invc);

    let invd = d.inverse();
    assert!(invd.is_none());

}

#[test]
fn should_calculate_the_inverse_of_4_by_4() {
    let assert_is_identity = |m: Mat4x4| {
        assert_feq!(m.0.0, 1.);
        assert_feq!(m.1.0, 0.);
        assert_feq!(m.2.0, 0.);
        assert_feq!(m.3.0, 0.);
        assert_feq!(m.0.1, 0.);
        assert_feq!(m.1.1, 1.);
        assert_feq!(m.2.1, 0.);
        assert_feq!(m.3.1, 0.);
        assert_feq!(m.0.2, 0.);
        assert_feq!(m.1.2, 0.);
        assert_feq!(m.2.2, 1.);
        assert_feq!(m.3.2, 0.);
        assert_feq!(m.0.3, 0.);
        assert_feq!(m.1.3, 0.);
        assert_feq!(m.2.3, 0.);
        assert_feq!(m.3.3, 1.);
    };

    let a = Mat4x4(Vec4(1., 1., 4., 5.),Vec4(3., 3., 3., 2.),Vec4(5., 1., 9., 0.),Vec4(9., 7., 7., 9.));
    let b = Mat4x4(Vec4(5., 0., 6., 0.),Vec4(1., 9., 1., 0.),Vec4(1., 1., 8., 1.),Vec4(0., 1., 0., 2.));
    let c = Mat4x4(Vec4(-10., 5., 10., -8.),Vec4(1., -5., -8., -4.),Vec4(-10., -5., -6., 3.),Vec4(3., 6., 8., -2.));
    let d = Mat4x4(Vec4(1., 2., 3., 4.),Vec4(5., 6., 7., 8.),Vec4(9., 10., 11., 12.),Vec4(13., 14., 15., 16.));

    let inva = a.inverse().unwrap();
    assert_feq!(inva.0.0, -0.27659574);
    assert_feq!(inva.1.0, 0.04255319);
    assert_feq!(inva.2.0, 0.14893617);
    assert_feq!(inva.3.0, 0.12765957);
    assert_feq!(inva.0.1, -0.625);
    assert_feq!(inva.1.1, 0.875);
    assert_feq!(inva.2.1, 0.25);
    assert_feq!(inva.3.1, -0.25);
    assert_feq!(inva.0.2, 0.10372341);
    assert_feq!(inva.1.2, -0.14095744);
    assert_feq!(inva.2.2, 0.069148935);
    assert_feq!(inva.3.2, -0.047872342);
    assert_feq!(inva.0.3, 0.2925532);
    assert_feq!(inva.1.3, -0.21808511);
    assert_feq!(inva.2.3, -0.13829787);
    assert_feq!(inva.3.3, 0.095744684);
    assert_is_identity(inva * a);
    assert_is_identity(a * inva);

    let invb = b.inverse().unwrap();
    assert_feq!(invb.0.0, 0.23327896);
    assert_feq!(invb.1.0, -0.0228385);
    assert_feq!(invb.2.0, -0.027732464);
    assert_feq!(invb.3.0, 0.01141925);
    assert_feq!(invb.0.1, 0.009787928);
    assert_feq!(invb.1.1, 0.110929854);
    assert_feq!(invb.2.1, -0.008156607);
    assert_feq!(invb.3.1, -0.055464927);
    assert_feq!(invb.0.2, -0.1761827);
    assert_feq!(invb.1.2, 0.0032626428);
    assert_feq!(invb.2.2, 0.14681892);
    assert_feq!(invb.3.2, -0.0016313214);
    assert_feq!(invb.0.3, 0.08809135);
    assert_feq!(invb.1.3, -0.0016313214);
    assert_feq!(invb.2.3, -0.07340946);
    assert_feq!(invb.3.3, 0.5008157);
    assert_is_identity(invb * b);
    assert_is_identity(b * invb);

    let invc = c.inverse().unwrap();
    assert_feq!(invc.0.0, 0.013363029);
    assert_feq!(invc.1.0, -0.3608018);
    assert_feq!(invc.2.0, 0.25278395);
    assert_feq!(invc.3.0, -0.051224943);
    assert_feq!(invc.0.1, -0.05790646);
    assert_feq!(invc.1.1, 0.5634744);
    assert_feq!(invc.2.1, -0.42873052);
    assert_feq!(invc.3.1, -0.111358576);
    assert_feq!(invc.0.2, -0.18262807);
    assert_feq!(invc.1.2, 0.9309577);
    assert_feq!(invc.2.2, -0.62138087);
    assert_feq!(invc.3.2, 0.033407573);
    assert_feq!(invc.0.3, -0.21158129);
    assert_feq!(invc.1.3, 1.7126949);
    assert_feq!(invc.2.3, -1.085746);
    assert_feq!(invc.3.3, -0.022271715);
    assert_is_identity(invc * c);
    assert_is_identity(c * invc);

    let invd = d.inverse();
    assert!(invd.is_none());
}
