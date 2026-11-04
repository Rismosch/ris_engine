use std::cell::UnsafeCell;
use std::rc::Rc;

use ris_async::ris_deque::RisDeque;

#[test]
fn should_push_and_pop() {
    let mut b = RisDeque::<usize, 4>::default();

    b.push_back(1).unwrap();
    b.push_back(2).unwrap();
    assert!(b.pop_back().unwrap() == 2);
    assert!(b.pop_back().unwrap() == 1);
    assert!(b.pop_back().is_none());

    b.push_front(3).unwrap();
    b.push_front(4).unwrap();
    assert!(b.pop_front().unwrap() == 4);
    assert!(b.pop_front().unwrap() == 3);
    assert!(b.pop_front().is_none());

    b.push_back(5).unwrap();
    b.push_back(6).unwrap();
    assert!(b.pop_front().unwrap() == 5);
    assert!(b.pop_front().unwrap() == 6);
    assert!(b.pop_front().is_none());

    b.push_front(7).unwrap();
    b.push_front(8).unwrap();
    assert!(b.pop_back().unwrap() == 7);
    assert!(b.pop_back().unwrap() == 8);
    assert!(b.pop_back().is_none());

    b.push_front(9).unwrap();
    b.push_back(10).unwrap();
    b.push_front(11).unwrap();
    assert!(b.push_back(12).is_err());
    assert!(b.push_front(13).is_err());
}

#[test]
fn should_push_and_pop_boxed() {
    let mut b = RisDeque::<Box<usize>, 4>::default();

    b.push_back(Box::new(1)).unwrap();
    b.push_back(Box::new(2)).unwrap();
    assert!(*b.pop_back().unwrap() == 2);
    assert!(*b.pop_back().unwrap() == 1);
    assert!(b.pop_back().is_none());

    b.push_front(Box::new(3)).unwrap();
    b.push_front(Box::new(4)).unwrap();
    assert!(*b.pop_front().unwrap() == 4);
    assert!(*b.pop_front().unwrap() == 3);
    assert!(b.pop_front().is_none());

    b.push_back(Box::new(5)).unwrap();
    b.push_back(Box::new(6)).unwrap();
    assert!(*b.pop_front().unwrap() == 5);
    assert!(*b.pop_front().unwrap() == 6);
    assert!(b.pop_front().is_none());

    b.push_front(Box::new(7)).unwrap();
    b.push_front(Box::new(8)).unwrap();
    assert!(*b.pop_back().unwrap() == 7);
    assert!(*b.pop_back().unwrap() == 8);
    assert!(b.pop_back().is_none());

    b.push_front(Box::new(9)).unwrap();
    b.push_back(Box::new(10)).unwrap();
    b.push_front(Box::new(11)).unwrap();
    assert!(b.push_back(Box::new(12)).is_err());
    assert!(b.push_front(Box::new(13)).is_err());
}

#[test]
fn should_drop_owned_data() {
    #[derive(Debug, Clone)]
    struct Counter(Rc<UnsafeCell<usize>>);
    impl Drop for Counter {
        fn drop(&mut self) {
            unsafe {*self.0.get() += 1}
        }
    }

    let c = Counter(Rc::new(UnsafeCell::new(0)));
    let mut b = RisDeque::<Counter, 16>::default();
    assert!(unsafe{*c.0.get()} == 0);
    b.push_back(c.clone()).unwrap();
    b.push_back(c.clone()).unwrap();
    b.push_back(c.clone()).unwrap();
    b.push_back(c.clone()).unwrap();
    b.push_back(c.clone()).unwrap();
    assert!(unsafe{*c.0.get()} == 0);
    drop(b.pop_back().unwrap());
    assert!(unsafe{*c.0.get()} == 1);
    drop(b.pop_front().unwrap());
    assert!(unsafe{*c.0.get()} == 2);
    drop(b);
    assert!(unsafe{*c.0.get()} == 5);
}
