use ris_test::test::test;

#[test]
fn should_succeed() {
    let mut result = false;

    test()
    .run(|| result = true);

    assert!(result);
}

#[test]
#[should_panic(expected = "test failed")]
fn should_fail() {
    test()
    .run(|| panic!("test failed"))
}