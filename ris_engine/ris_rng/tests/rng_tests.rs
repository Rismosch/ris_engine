use ris_rng::rng;
use ris_test::util::*;

#[test]
fn should_be_repeatable() {
    single_threaded(|| {
        let mut results1 = Vec::new();
        let mut results2 = Vec::new();
        let mut results3 = Vec::new();

        unsafe {
            rng::init_with_seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        }
        for _ in 0..100 {
            results1.push(rng::next_u());
        }

        unsafe {
            rng::init_with_seed([15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
        }
        for _ in 0..100 {
            results2.push(rng::next_u());
        }

        unsafe {
            rng::init_with_seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        }
        for _ in 0..100 {
            results3.push(rng::next_u());
        }

        for i in 0..100 {
            assert_ne!(results1[i], results2[i]);
            assert_eq!(results1[i], results3[i]);
        }
    });
}

#[test]
fn should_generate_numbers_between_0_and_1() {
    single_threaded(|| {
        unsafe {
            rng::init().unwrap();
        }

        for _ in 0..1_000_000 {
            let random = rng::next_f();
            assert!(random >= 0.);
            assert!(random <= 1.);
        }
    });
}

#[test]
fn should_generate_numbers_between_int_range() {
    single_threaded(|| {
        unsafe {
            rng::init().unwrap();
        }

        let min = -13;
        let max = 42;

        for _ in 0..1_000_000 {
            let random = rng::range_i(min, max);
            assert!(random >= min);
            assert!(random <= max);
        }
    });
}

#[test]
fn should_generate_numbers_between_float_range() {
    single_threaded(|| {
        unsafe {
            rng::init().unwrap();
        }

        let min = -12.34;
        let max = 56.78;

        for _ in 0..1_000_000 {
            let random = rng::range_f(min, max);
            assert!(random >= min);
            assert!(random <= max);
        }
    });
}
