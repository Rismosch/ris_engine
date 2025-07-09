use ris_data::counter::Counter;

#[test]
fn should_should_add_and_subtract() {
    let mut counter = Counter::default();

    let equals = |lhs: Counter, rhs: isize| {
        let left = lhs.raw();
        let right = usize::from_ne_bytes(rhs.to_ne_bytes());
        println!("binary left:  0b{:b}", left);
        println!("binary right: 0b{:b}", right);
        println!("bruh: {}", isize::from_ne_bytes(left.to_ne_bytes()));
        assert_eq!(left, right);
    };

    equals(counter, 0);
    counter += 1;
    equals(counter, 1);
    counter += 42;
    equals(counter, 43);
    counter -= 100;
    equals(counter, -57);
    counter += isize::MAX;
    //equals(counter, 9223372036854775750);
    panic!();
    counter += isize::MAX / 2;
    //equals(counter, -4611686018427387963);
    panic!();
    counter -= isize::MIN;
    //equals(counter, 4611686018427387845);
    panic!();
}

#[test]
fn should_sort_case_1() {
    panic!();
}

#[test]
fn should_sort_case_2() {
    panic!();
}
