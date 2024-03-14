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
                let abs = f32::abs(*value);
                black_box(abs);
            }
        });
    });

    group.bench_function("bit_magic", |b| {
        b.iter(|| {
            for value in &values {
                let abs = ris_math::fastabs(*value);
                black_box(abs);
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
                let result = -value;
                black_box(result);
            }
        });
    });

    group.bench_function("bit_magic", |b| {
        b.iter(|| {
            for value in &values {
                let result = ris_math::fastneg(*value);
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
                let result = ris_math::fastlog2(*value);
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
                let result = ris_math::fastexp2(*value);
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
                let result = ris_math::fastpow(*value1, *value2);
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
                let result = ris_math::fastsqrt(*value);
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
                let result = ris_math::fastinversesqrt(*value);
                black_box(result);
            }
        });
    });

    group.finish();
}

criterion_group!(benches, abs, negate, log2, exp2, pow, sqrt, inversesqrt);
criterion_main!(benches);
