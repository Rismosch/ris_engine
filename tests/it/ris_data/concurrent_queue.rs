use ris_data::concurrent_queue::ConcurrentQueue;

#[test]
fn should_push_and_pop() {
    let mut queue = ConcurrentQueue::new();

    queue.push(1);
    queue.push(2);
    queue.push(3);

    let one = queue.try_pop();
    let two = queue.try_pop();

    queue.push(4);
    queue.push(5);

    let three = queue.try_pop();
    let four = queue.try_pop();
    let five = queue.try_pop();
    let six = queue.try_pop();

    assert_eq!(one, Some(1));
    assert_eq!(two, Some(2));
    assert_eq!(three, Some(3));
    assert_eq!(four, Some(4));
    assert_eq!(five, Some(5));
    assert_eq!(six, None);
}
