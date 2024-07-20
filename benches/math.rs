use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use ris_rng::rng::Rng;
use ris_rng::rng::Seed;

fn abs(c: &mut Criterion) {
    let mut group = c.benchmark_group("math_abs");

    let mut rng = Rng::new(Seed::new().unwrap());

    let count = 1_000_000;
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        let value = rng.range_f(-1_000_000., 1_000_000.);
        values.push(value);
    }

    group.bench_function("std", |b| {
        b.iter(|| {
            for value in &values {
                let result = f32::abs(*value);
                black_box(result);
            }
        });
    });

    group.bench_function("bit_magic", |b| {
        b.iter(|| {
            for value in &values {
                let result = ris_math::fast::as_float(ris_math::fast::as_int(*value) & 0x7FFF_FFFF);
                black_box(result);
            }
        });
    });

    group.finish();
}

fn negate(c: &mut Criterion) {
    let mut group = c.benchmark_group("math_negate");

    let mut rng = Rng::new(Seed::new().unwrap());

    let count = 1_000_000;
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        let value = rng.range_f(-1_000_000., 1_000_000.);
        values.push(value);
    }

    group.bench_function("std", |b| {
        b.iter(|| {
            for value in &values {
                //let result = -value;
                let result = std::ops::Neg::neg(value);
                black_box(result);
            }
        });
    });

    group.bench_function("bit_magic", |b| {
        b.iter(|| {
            for value in &values {
                let result =
                    ris_math::fast::as_float(ris_math::fast::as_int(*value) ^ 0x8000_0000u32 as i32);
                black_box(result);
            }
        });
    });

    group.finish();
}

fn log2(c: &mut Criterion) {
    let mut group = c.benchmark_group("math_log2");

    let mut rng = Rng::new(Seed::new().unwrap());

    let count = 1_000_000;
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        let value = rng.range_f(-1_000_000., 1_000_000.);
        values.push(value);
    }

    group.bench_function("std", |b| {
        b.iter(|| {
            for value in &values {
                let result = f32::log2(*value);
                black_box(result);
            }
        });
    });

    group.bench_function("bit_magic", |b| {
        b.iter(|| {
            for value in &values {
                let result = ris_math::fast::log2(*value);
                black_box(result);
            }
        });
    });

    group.finish();
}

fn exp2(c: &mut Criterion) {
    let mut group = c.benchmark_group("math_exp2");

    let mut rng = Rng::new(Seed::new().unwrap());

    let count = 1_000_000;
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        let value = rng.range_f(-1_000_000., 1_000_000.);
        values.push(value);
    }

    group.bench_function("std", |b| {
        b.iter(|| {
            for value in &values {
                let result = f32::exp2(*value);
                black_box(result);
            }
        });
    });

    group.bench_function("bit_magic", |b| {
        b.iter(|| {
            for value in &values {
                let result = ris_math::fast::exp2(*value);
                black_box(result);
            }
        });
    });

    group.finish();
}

fn pow(c: &mut Criterion) {
    let mut group = c.benchmark_group("math_pow");

    let mut rng = Rng::new(Seed::new().unwrap());

    let count = 1_000_000;
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        let value1 = rng.range_f(-1_000_000., 1_000_000.);
        let value2 = rng.range_f(-1_000_000., 1_000_000.);
        values.push((value1, value2));
    }

    group.bench_function("std", |b| {
        b.iter(|| {
            for (value1, value2) in &values {
                let result = f32::powf(*value1, *value2);
                black_box(result);
            }
        });
    });

    group.bench_function("bit_magic", |b| {
        b.iter(|| {
            for (value1, value2) in &values {
                let result = ris_math::fast::pow(*value1, *value2);
                black_box(result);
            }
        });
    });

    group.finish();
}

fn sqrt(c: &mut Criterion) {
    let mut group = c.benchmark_group("math_sqrt");

    let mut rng = Rng::new(Seed::new().unwrap());

    let count = 1_000_000;
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        let value = rng.range_f(-1_000_000., 1_000_000.);
        values.push(value);
    }

    group.bench_function("std", |b| {
        b.iter(|| {
            for value in &values {
                let result = f32::sqrt(*value);
                black_box(result);
            }
        });
    });

    group.bench_function("bit_magic", |b| {
        b.iter(|| {
            for value in &values {
                let result = ris_math::fast::sqrt(*value);
                black_box(result);
            }
        });
    });

    group.finish();
}

fn inversesqrt(c: &mut Criterion) {
    let mut group = c.benchmark_group("math_inversesqrt");

    let mut rng = Rng::new(Seed::new().unwrap());

    let count = 1_000_000;
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        let value = rng.range_f(-1_000_000., 1_000_000.);
        values.push(value);
    }

    group.bench_function("std", |b| {
        b.iter(|| {
            for value in &values {
                let result = 1. / f32::sqrt(*value);
                black_box(result);
            }
        });
    });

    group.bench_function("bit_magic", |b| {
        b.iter(|| {
            for value in &values {
                let result = ris_math::fast::inversesqrt(*value);
                black_box(result);
            }
        });
    });

    group.finish();
}

fn sign(c: &mut Criterion) {
    let mut group = c.benchmark_group("math_sign");

    let mut rng = Rng::new(Seed::new().unwrap());

    let count = 1_000_000;
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        let value = rng.range_f(-1_000_000., 1_000_000.);
        values.push(value);
    }

    group.bench_function("branched", |b| {
        b.iter(|| {
            for value in &values {
                let result = if *value == 0. {
                    0.
                } else if *value > 0. {
                    1.
                } else {
                    -1.
                };
                black_box(result);
            }
        });
    });

    group.bench_function("branchless", |b| {
        b.iter(|| {
            for value in &values {
                let result = ris_math::fast::choose(
                    *value == 0.,
                    0.,
                    ris_math::fast::choose(
                        *value > 0.,
                        1., 
                        -1., 
                    ),
                );
                black_box(result);
            }
        });
    });

    group.finish();
}

fn min(c: &mut Criterion) {
    let mut group = c.benchmark_group("math_min");

    let mut rng = Rng::new(Seed::new().unwrap());

    let count = 1_000_000;
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        let value = rng.range_f(-1_000_000., 1_000_000.);
        values.push(value);
    }

    group.bench_function("branched", |b| {
        b.iter(|| {
            for chunk in values.chunks_exact(2) {
                let x = chunk[0];
                let y = chunk[1];
                let result = if y < x { y } else { x };
                black_box(result);
            }
        });
    });

    group.bench_function("branchless", |b| {
        b.iter(|| {
            for chunk in values.chunks_exact(2) {
                let x = chunk[0];
                let y = chunk[1];
                let result = ris_math::fast::choose(y < x, y, x);
                black_box(result);
            }
        });
    });

    group.finish();
}

fn max(c: &mut Criterion) {
    let mut group = c.benchmark_group("math_max");

    let mut rng = Rng::new(Seed::new().unwrap());

    let count = 1_000_000;
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        let value = rng.range_f(-1_000_000., 1_000_000.);
        values.push(value);
    }

    group.bench_function("branched", |b| {
        b.iter(|| {
            for chunk in values.chunks_exact(2) {
                let x = chunk[0];
                let y = chunk[1];
                let result = if x < y { y } else { x };
                black_box(result);
            }
        });
    });

    group.bench_function("branchless", |b| {
        b.iter(|| {
            for chunk in values.chunks_exact(2) {
                let x = chunk[0];
                let y = chunk[1];
                let result = ris_math::fast::choose(x < y, y, x);
                black_box(result);
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    abs,
    negate,
    log2,
    exp2,
    pow,
    sqrt,
    inversesqrt,
    sign,
    min,
    max,
);
criterion_main!(benches);
