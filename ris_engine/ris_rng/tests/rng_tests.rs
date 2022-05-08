use ris_test::util::*;

#[test]
fn repeates() {
    repeat(10, || {
        println!("hello world");
    });
}

#[test]
fn wraps() {
    wrap(teardown, || {
        println!("test");
        panic!("");
    });
}

fn teardown() {
    println!("teardown");
}
