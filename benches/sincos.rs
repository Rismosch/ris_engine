use std::f32::consts::PI;

use criterion::Criterion;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;

use ris_rng::rng::Rng;
use ris_rng::rng::Seed;

fn sin_cos(c: &mut Criterion) {
    let mut group = c.benchmark_group("sincos");

    let mut rng = Rng::new(Seed::new());

    let count = 1_000_000;
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        let value = rng.next_f32_between(0., 2. * PI);
        values.push(value);
    }

    group.bench_function("std", |b| {
        b.iter(|| {
            for value in &values {
                let sin = f32::sin(*value);
                let cos = f32::cos(*value);

                black_box(sin);
                black_box(cos);
            }
        });
    });

    group.bench_function("bhaskara", |b| {
        b.iter(|| {
            for value in &values {
                let sincos = sincos_bhaskara(*value);

                black_box(sincos);
            }
        });
    });

    group.bench_function("bhaskara_branchless", |b| {
        b.iter(|| {
            for value in &values {
                let sincos = sincos_bhaskara_branchless(*value);

                black_box(sincos);
            }
        });
    });

    group.bench_function("bhaskara_without_sqrt", |b| {
        b.iter(|| {
            for value in &values {
                let sincos = sincos_bhaskara_without_sqrt(*value);

                black_box(sincos);
            }
        });
    });

    group.finish();
}

pub fn sincos_bhaskara(angle: f32) -> (f32, f32) {
    let sin = if angle < PI {
        bhaskara(angle - 0.5 * PI)
    } else {
        -bhaskara(angle - 1.5 * PI)
    };

    let mut cos = f32::sqrt(1. - sin * sin);

    if angle > 0.5 * PI && angle < 1.5 * PI {
        cos = -cos;
    }

    (sin, cos)
}

pub fn sincos_bhaskara_branchless(angle: f32) -> (f32, f32) {
    let sin_part1 = bhaskara(angle - 0.5 * PI);
    let sin_part2 = -bhaskara(angle - 1.5 * PI);
    let sin_choose = (angle > PI) as usize as f32;

    let flipsign = (angle > 0.5 * PI && angle < 1.5 * PI) as usize as f32;
    let sign = ris_math::mix(1., -1., flipsign);

    let sin = ris_math::mix(sin_part1, sin_part2, sin_choose);
    let cos = sign * f32::sqrt(1. - sin * sin);

    (sin, cos)
}

pub fn sincos_bhaskara_without_sqrt(angle: f32) -> (f32, f32) {
    let sin_part1 = bhaskara(angle - 0.5 * PI);
    let sin_part2 = -bhaskara(angle - 1.5 * PI);
    let sin_choose = (angle > PI) as usize as f32;

    let cos_angle_choose = (angle > 1.5 * PI) as usize as f32;
    let cos_angle = ris_math::mix(angle + 0.5 * PI, angle - 1.5 * PI, cos_angle_choose);

    let cos_part1 = bhaskara(cos_angle - 0.5 * PI);
    let cos_part2 = -bhaskara(cos_angle - 1.5 * PI);
    let cos_choose = (cos_angle > PI) as usize as f32;

    let sin = ris_math::mix(sin_part1, sin_part2, sin_choose);
    let cos = ris_math::mix(cos_part1, cos_part2, cos_choose);

    (sin, cos)
}

pub fn bhaskara(x: f32) -> f32 {
    let pi2 = PI * PI;
    let xx = x * x;
    let xx4 = xx + xx + xx + xx;
    (pi2 - xx4) / (pi2 + xx)
}

criterion_group!(benches, sin_cos);
criterion_main!(benches);
