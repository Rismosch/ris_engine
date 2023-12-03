use std::cell::RefCell;
use std::rc::Rc;

use ris_jobs::job::Job;

#[test]
fn should_execute() {
    let flag = Rc::new(RefCell::new(false));
    let cloned = flag.clone();
    let mut job = Job::new(move || *cloned.borrow_mut() = true);

    assert!(!*flag.borrow_mut());
    job.invoke();
    assert!(*flag.borrow_mut());
}

#[test]
fn should_execute_once() {
    let counter = Rc::new(RefCell::new(0));
    let cloned = counter.clone();
    let mut job = Job::new(move || *cloned.borrow_mut() += 1);

    job.invoke();
    job.invoke();
    job.invoke();

    assert_eq!(*counter.borrow_mut(), 1);
}
