use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use ris_async::thread_pool::ThreadPool;
use ris_async::thread_pool::ThreadPoolCreateInfo;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;

fn async_runner(c: &mut Criterion) {
    let mut group = c.benchmark_group("async");

    let mut rng = Rng::new(Seed::new().unwrap());

    let hash_iterations = 1_000;
    let hash_input_count = 1_000;
    let mut hash_inputs = Vec::with_capacity(hash_input_count);
    for _ in 0..hash_inputs.capacity() {
        let hash_input = rng.next_u64();
        hash_inputs.push(hash_input);
    }

    let cpu_count = sdl2::cpuinfo::cpu_count() as usize;

    // control
    group.bench_function("control", |b| {
        b.iter(|| {
            for &input in &hash_inputs {
                let result = dummy_work(input, hash_iterations);
                black_box(result);
            }
        });
    });

    // async thread pool
    let cpu_counts = [cpu_count, cpu_count / 2, 1];
    let bools = [0, 1];
    for cpu_count in cpu_counts {
        for set_affinity in bools {
            for park_workers in bools {
                let create_info = ThreadPoolCreateInfo {
                    buffer_capacity: 256,
                    cpu_count,
                    threads: cpu_count,
                    set_affinity: set_affinity == 1,
                    use_parking: park_workers == 1,
                };
                let thread_pool = ThreadPool::init(create_info).unwrap();

                group.bench_function(format!("cpu: {}, aff:{}, prk:{} ", cpu_count, set_affinity, park_workers), |b| {
                    b.iter(|| {
                        let mut futures = Vec::with_capacity(hash_inputs.len());
                        for &input in &hash_inputs {
                            let future = ThreadPool::submit(async move {
                                dummy_work(input, hash_iterations)
                            });
                            futures.push(future);
                        }

                        for future in futures {
                            let result = ThreadPool::block_on(future);
                            black_box(result);
                        }
                    });
                });

                drop(thread_pool)
            }
        }
    }
}

fn dummy_work(input: u64, iterations: usize) -> u64 {
    use std::hash::Hash;
    use std::hash::Hasher;
    use std::collections::hash_map::DefaultHasher;

    let mut hash = input;
    for _ in 0..iterations {
        let mut hasher = DefaultHasher::new();
        hash.hash(&mut hasher);
        hash = hasher.finish();
    }

    hash
}

criterion_group!(benches, async_runner);
criterion_main!(benches);
