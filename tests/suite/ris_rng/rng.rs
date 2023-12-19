use ris_rng::rng::Rng;
use ris_rng::rng::Seed;

#[cfg(not(miri))]
const LOOP_ITERATIONS: usize = 1_000_000;

#[cfg(miri)]
const LOOP_ITERATIONS: usize = 100;

#[test]
fn should_be_repeatable() {
    let mut results1 = Vec::new();
    let mut results2 = Vec::new();
    let mut results3 = Vec::new();

    let mut seed = Seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let mut rng = Rng::new(seed);
    for _ in 0..LOOP_ITERATIONS {
        results1.push(rng.next_u());
    }

    seed = Seed([15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    rng = Rng::new(seed);
    for _ in 0..LOOP_ITERATIONS {
        results2.push(rng.next_u());
    }

    seed = Seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    rng = Rng::new(seed);
    for _ in 0..LOOP_ITERATIONS {
        results3.push(rng.next_u());
    }

    for i in 0..LOOP_ITERATIONS {
        assert_ne!(results1[i], results2[i]);
        assert_eq!(results1[i], results3[i]);
    }
}

#[test]
fn should_generate_numbers_between_0_and_1() {
    let mut rng = Rng::new(Seed::new().unwrap());

    for _ in 0..LOOP_ITERATIONS {
        let random = rng.next_f();
        assert!(random >= 0.);
        assert!(random <= 1.);
    }
}

#[test]
fn should_generate_numbers_between_int_range() {
    let mut rng = Rng::new(Seed::new().unwrap());

    let min = -13;
    let max = 42;

    for _ in 0..LOOP_ITERATIONS {
        let random = rng.range_i(min, max);
        assert!(random >= min);
        assert!(random <= max);
    }
}

#[test]
fn should_generate_numbers_between_float_range() {
    let mut rng = Rng::new(Seed::new().unwrap());

    let min = -12.34;
    let max = 56.78;

    for _ in 0..LOOP_ITERATIONS {
        let random = rng.range_f(min, max);
        assert!(random >= min);
        assert!(random <= max);
    }
}
