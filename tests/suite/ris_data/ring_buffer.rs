use std::cell::UnsafeCell;
use std::rc::Rc;

use ris_data::ring_buffer::RingBuffer;

#[test]
fn should_push_and_pop() {
    let mut b = RingBuffer::<usize, 4>::default();

    b.push_back(1).unwrap();
    b.push_back(2).unwrap();
    assert!(b.pop_front().unwrap() == 1);
    assert!(b.pop_front().unwrap() == 2);
    assert!(b.pop_front().is_none());
    assert!(b.pop_back().is_none());

    b.push_front(3).unwrap();
    b.push_front(4).unwrap();
    b.push_front(5).unwrap();
    assert!(b.push_front(6).is_err());
    assert!(b.push_back(7).is_err());
    assert!(b.pop_back().unwrap() == 3);
    assert!(b.pop_back().unwrap() == 4);
    assert!(b.pop_back().unwrap() == 5);
    assert!(b.pop_back().is_none());
    assert!(b.pop_front().is_none());
}

