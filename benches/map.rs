use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use std::collections::HashMap;

use ris_data::ris_map;
use ris_data::ris_map::RisMap;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;

fn map_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("map_insert");

    let low = ris_map::EXP / 4;
    let medium = ris_map::EXP / 2;
    let high = ris_map::EXP;

    for exp in [low, medium, high] {
        let mut ris_map = RisMap::default();
        let mut hash_map = HashMap::new();
        let key_values = generate_random_key_values(1 << exp);

        group.bench_function(format!("RisMap::assign::{}", key_values.len()), |b| {
            b.iter(|| {
                for (key, value) in &key_values {
                    let _ = ris_map.assign(key, value);
                }
            })
        });

        group.bench_function(format!("HashMap::insert::{}", key_values.len()), |b| {
            b.iter(|| {
                for (key, value) in &key_values {
                    hash_map.insert(key, value);
                }
            })
        });

        black_box(ris_map);
        black_box(hash_map);
    }

    group.finish();
}

fn map_retreive(c: &mut Criterion) {
    let mut group = c.benchmark_group("map_retreive");

    let low = ris_map::EXP / 4;
    let medium = ris_map::EXP / 2;
    let high = ris_map::EXP;

    for exp in [low, medium, high] {
        let mut ris_map = RisMap::default();
        let mut hash_map = HashMap::new();
        let key_values = generate_random_key_values(1 << exp);

        for (key, value) in &key_values {
            let _ = ris_map.assign(key, value);
            hash_map.insert(key, value);
        }

        group.bench_function(format!("RisMap::find::{}", key_values.len()), |b| {
            b.iter(|| {
                for (key, _) in &key_values {
                    let value = ris_map.find(key);
                    let _ = black_box(value);
                }
            })
        });

        group.bench_function(format!("HashMap::get::{}", key_values.len()), |b| {
            b.iter(|| {
                for (key, _) in &key_values {
                    let value = hash_map.get(key);
                    let _ = black_box(value);
                }
            })
        });

        black_box(ris_map);
        black_box(hash_map);
    }

    group.finish();
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
