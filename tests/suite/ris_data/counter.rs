use ris_rng::rng::Seed;
use ris_rng::rng::Rng;
use ris_data::counter::Counter;

#[test]
fn should_should_add() {
    let mut counter = Counter::default();

    assert_eq!(counter.raw(), 0);
    counter.add_one();
    assert_eq!(counter.raw(), 1);
    counter.add_one();
    counter.add_one();
    counter.add_one();
    counter.add_one();
    assert_eq!(counter.raw(), 5);
}

#[test]
fn should_overflow() {
    let mut counter = Counter::from_raw(usize::MAX - 2);

    counter.add_one();
    counter.add_one();
    counter.add_one();
    assert_eq!(counter.raw(), 0);
    counter.add_one();
    counter.add_one();
    assert_eq!(counter.raw(), 2);
}

#[test]
fn should_sort_case_1() {
    let mut counters = vec![
        Counter::from_raw(2),
        Counter::from_raw(20),
        Counter::from_raw(4),
        Counter::from_raw(2),
        Counter::from_raw(19),
        Counter::from_raw(13),
        Counter::from_raw(6),
        Counter::from_raw(11),
        Counter::from_raw(0),
        Counter::from_raw(18),
    ];

    counters.sort();

    assert_eq!(
        counters,
        vec![
            Counter::from_raw(0),
            Counter::from_raw(2),
            Counter::from_raw(2),
            Counter::from_raw(4),
            Counter::from_raw(6),
            Counter::from_raw(11),
            Counter::from_raw(13),
            Counter::from_raw(18),
            Counter::from_raw(19),
            Counter::from_raw(20),
        ],
    );
}

#[test]
fn should_sort_case_2() {
    let mut counters = vec![
        Counter::from_raw(1),
        Counter::from_raw(5),
        Counter::from_raw(0),
        Counter::from_raw(18446744073709551610),
        Counter::from_raw(5),
        Counter::from_raw(18446744073709551611),
        Counter::from_raw(6),
        Counter::from_raw(18446744073709551612),
        Counter::from_raw(4),
        Counter::from_raw(18446744073709551610),
    ];

    counters.sort();

    assert_eq!(
        counters,
        vec![
            Counter::from_raw(18446744073709551610),
            Counter::from_raw(18446744073709551610),
            Counter::from_raw(18446744073709551611),
            Counter::from_raw(18446744073709551612),
            Counter::from_raw(0),
            Counter::from_raw(1),
            Counter::from_raw(4),
            Counter::from_raw(5),
            Counter::from_raw(5),
            Counter::from_raw(6),
        ],
    );
}
