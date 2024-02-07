use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use std::collections::HashMap;

use ris_data::ris_map::RisMap;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;

fn map_insert(c: &mut Criterion) {
    let mut ris_map = RisMap::default();
    let mut hash_map = HashMap::new();

    let key_values = generate_random_key_values(100);

    let mut group = c.benchmark_group("map insert");

    group.bench_function("RisMap::assign", |b| {
        b.iter(|| for (key, value) in &key_values {
            ris_map.assign(key, value).unwrap();
        })
    });

    group.bench_function("HashMap::insert", |b| {
        b.iter(|| for (key, value) in &key_values {
            hash_map.insert(key, value);
        })
    });

    group.finish();

    black_box(ris_map);
    black_box(hash_map);
}

fn map_retreive(c: &mut Criterion) {
    let mut ris_map = RisMap::default();
    let mut hash_map = HashMap::new();

    let key_values = generate_random_key_values(100);

    for (key, value) in &key_values {
        ris_map.assign(key, value).unwrap();
        hash_map.insert(key, value);
    }

    let mut group = c.benchmark_group("map retreive");

    group.bench_function("RisMap::find", |b| {
        b.iter(|| for (key, _) in &key_values {
            let value = ris_map.find(key).unwrap().unwrap();
            black_box(value);
        })
    });

    group.bench_function("HashMap::get", |b| {
        b.iter(|| for (key, _) in &key_values {
            let value = hash_map.get(key).unwrap();
            black_box(value);
        })
    });

    group.finish();

    black_box(ris_map);
    black_box(hash_map);
}

fn generate_random_key_values(count: usize) -> Vec<(String, u32)> {
    let mut result = Vec::with_capacity(count);

    let mut rng = Rng::new(Seed::new().unwrap());

    for _ in 0..count {
        let value = rng.next_u();
        let key = format!("{}", value);

        result.push((key, value));
    }

    result
}

criterion_group!(benches, map_insert, map_retreive);
criterion_main!(benches);
