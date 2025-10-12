use criterion::Criterion;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;

use ris_rng::rng::Rng;
use ris_rng::rng::Seed;

fn vec_overwrite(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_overwrite");

    let mut rng = Rng::new(Seed::new());

    let count = 10;
    let max_elements = 4096;
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        let len = rng.next_i32_between(0, max_elements) as usize;
        let bytes = rng.next_bytes(len);
        values.push(bytes);
    }

    group.bench_function("to_vec + assign", |b| {
        b.iter(|| {
            let mut vec = Vec::new();

            for value in &values {
                vec = value.to_vec();
                black_box(&vec);
            }

            black_box(vec);
        })
    });

    group.bench_function("clear + extend_from_slice", |b| {
        b.iter(|| {
            let mut vec = Vec::new();

            for value in &values {
                vec.clear();
                vec.extend_from_slice(value);
                black_box(&vec);
            }

            black_box(vec);
        })
    });

    group.bench_function("clear + copy_from_slice", |b| {
        b.iter(|| {
            let mut vec = Vec::new();

            for value in &values {
                vec.clear();
                let len = value.len();
                vec.reserve(len);
                unsafe { vec.set_len(len) };
                vec.copy_from_slice(value);
                black_box(&vec);
            }

            black_box(vec);
        })
    });

    group.bench_function("copy_from_slice", |b| {
        b.iter(|| {
            let mut vec = Vec::new();

            for value in &values {
                let len = value.len();
                vec.reserve(len);
                unsafe { vec.set_len(len) };
                vec.copy_from_slice(value);
                black_box(&vec);
            }

            black_box(vec);
        })
    });

    group.finish();
}

criterion_group!(benches, vec_overwrite);
criterion_main!(benches);
