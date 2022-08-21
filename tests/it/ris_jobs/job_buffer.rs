use std::{cell::RefCell, rc::Rc};

use ris_jobs::{job::Job, job_buffer::JobBuffer};

#[test]
fn should_push_and_pop() {
    let mut job_buffer = JobBuffer::new(4);

    let data = Rc::new(RefCell::new(0));
    let moved_data = data.clone();
    let job = Job::new(move || *moved_data.borrow_mut() = 42);

    let push = job_buffer.push(job);
    assert!(push.is_ok());

    let job = job_buffer.pop();
    assert!(job.is_ok());
    job.unwrap().invoke();
    assert_eq!(*data.borrow(), 42);
}

#[test]
fn should_push_and_steal() {
    let mut job_buffer = JobBuffer::new(4);

    let data = Rc::new(RefCell::new(0));
    let moved_data = data.clone();
    let job = Job::new(move || *moved_data.borrow_mut() = 42);

    let push = job_buffer.push(job);
    assert!(push.is_ok());

    let job = job_buffer.steal();
    assert!(job.is_ok());
    job.unwrap().invoke();
    assert_eq!(*data.borrow(), 42);
}

#[test]
fn should_push_till_full() {
    let mut job_buffer = JobBuffer::new(2);

    let data = Rc::new(RefCell::new(0));
    let moved_data1 = data.clone();
    let moved_data2 = data.clone();
    let moved_data3 = data.clone();

    let job1 = Job::new(move || *moved_data1.borrow_mut() = 1);
    let job2 = Job::new(move || *moved_data2.borrow_mut() = 2);
    let job3 = Job::new(move || *moved_data3.borrow_mut() = 3);

    let push1 = job_buffer.push(job1);
    let push2 = job_buffer.push(job2);
    let push3 = job_buffer.push(job3);

    assert!(push1.is_ok());
    assert!(push2.is_ok());
    assert!(push3.is_err());

    push3.err().unwrap().not_pushed_job.invoke();
    assert_eq!(*data.borrow(), 3);
}

#[test]
fn should_pop_till_empty() {
    let mut job_buffer = JobBuffer::new(4);

    let data = Rc::new(RefCell::new(0));
    let moved_data1 = data.clone();
    let moved_data2 = data.clone();

    let job1 = Job::new(move || *moved_data1.borrow_mut() = 1);
    let job2 = Job::new(move || *moved_data2.borrow_mut() = 2);

    let _ = job_buffer.push(job1);
    let _ = job_buffer.push(job2);

    let pop1 = job_buffer.pop();
    let pop2 = job_buffer.pop();
    let pop3 = job_buffer.pop();

    assert!(pop1.is_ok());
    assert!(pop2.is_ok());
    assert!(pop3.is_err());

    pop1.ok().unwrap().invoke();
    assert_eq!(*data.borrow(), 2);
    pop2.ok().unwrap().invoke();
    assert_eq!(*data.borrow(), 1);
}

#[test]
fn should_steal_till_empty() {
    let mut job_buffer = JobBuffer::new(4);

    let data = Rc::new(RefCell::new(0));
    let moved_data1 = data.clone();
    let moved_data2 = data.clone();

    let job1 = Job::new(move || *moved_data1.borrow_mut() = 1);
    let job2 = Job::new(move || *moved_data2.borrow_mut() = 2);

    let _ = job_buffer.push(job1);
    let _ = job_buffer.push(job2);

    let steal1 = job_buffer.steal();
    let steal2 = job_buffer.steal();
    let steal3 = job_buffer.steal();

    assert!(steal1.is_ok());
    assert!(steal2.is_ok());
    assert!(steal3.is_err());

    steal1.ok().unwrap().invoke();
    assert_eq!(*data.borrow(), 1);
    steal2.ok().unwrap().invoke();
    assert_eq!(*data.borrow(), 2);
}

#[test]
fn should_push_pop_and_steal_multiple_times() {
    let mut job_buffer = JobBuffer::new(5);

    for _ in 0..5 {
        let data = Rc::new(RefCell::new(0));
        let moved_data1 = data.clone();
        let moved_data2 = data.clone();
        let moved_data3 = data.clone();
        let moved_data4 = data.clone();
        let moved_data5 = data.clone();
        let moved_data6 = data.clone();

        let job1 = Job::new(move || *moved_data1.borrow_mut() = 1);
        let job2 = Job::new(move || *moved_data2.borrow_mut() = 2);
        let job3 = Job::new(move || *moved_data3.borrow_mut() = 3);
        let job4 = Job::new(move || *moved_data4.borrow_mut() = 4);
        let job5 = Job::new(move || *moved_data5.borrow_mut() = 5);
        let job6 = Job::new(move || *moved_data6.borrow_mut() = 6);

        let push1 = job_buffer.push(job1);
        let push2 = job_buffer.push(job2);
        let push3 = job_buffer.push(job3);
        let push4 = job_buffer.push(job4);
        let push5 = job_buffer.push(job5);
        let push6 = job_buffer.push(job6);

        assert!(push1.is_ok());
        assert!(push2.is_ok());
        assert!(push3.is_ok());
        assert!(push4.is_ok());
        assert!(push5.is_ok());
        assert!(push6.is_err());

        push6.err().unwrap().not_pushed_job.invoke();
        assert_eq!(*data.borrow(), 6);

        let steal1 = job_buffer.steal();
        let pop2 = job_buffer.pop();
        let steal3 = job_buffer.steal();
        let pop4 = job_buffer.pop();
        let steal5 = job_buffer.steal();
        let pop6 = job_buffer.pop();
        let steal7 = job_buffer.steal();

        assert!(steal1.is_ok());
        assert!(pop2.is_ok());
        assert!(steal3.is_ok());
        assert!(pop4.is_ok());
        assert!(steal5.is_ok());
        assert!(pop6.is_err());
        assert!(steal7.is_err());

        steal1.ok().unwrap().invoke();
        assert_eq!(*data.borrow(), 1);
        pop2.ok().unwrap().invoke();
        assert_eq!(*data.borrow(), 5);
        steal3.ok().unwrap().invoke();
        assert_eq!(*data.borrow(), 2);
        pop4.ok().unwrap().invoke();
        assert_eq!(*data.borrow(), 4);
        steal5.ok().unwrap().invoke();
        assert_eq!(*data.borrow(), 3);
    }
}

#[test]
fn should_push_to_original_and_pop_from_duplicate() {
    let mut original_buffer = JobBuffer::new(4);
    let mut duplicated_buffer = original_buffer.duplicate();

    let data = Rc::new(RefCell::new(0));
    let moved_data1 = data.clone();
    let moved_data2 = data.clone();

    let job1 = Job::new(move || *moved_data1.borrow_mut() = 1);
    let job2 = Job::new(move || *moved_data2.borrow_mut() = 2);

    let push1 = original_buffer.push(job1);
    let push2 = original_buffer.push(job2);

    assert!(push1.is_ok());
    assert!(push2.is_ok());

    let pop1 = duplicated_buffer.pop();
    let steal2 = duplicated_buffer.steal();

    assert!(pop1.is_ok());
    assert!(steal2.is_ok());

    pop1.unwrap().invoke();
    assert_eq!(*data.borrow(), 2);
    steal2.unwrap().invoke();
    assert_eq!(*data.borrow(), 1);
}

#[test]
fn should_push_to_duplicate_and_pop_from_original() {
    let mut original_buffer = JobBuffer::new(4);
    let mut duplicated_buffer = original_buffer.duplicate();

    let data = Rc::new(RefCell::new(0));
    let moved_data1 = data.clone();
    let moved_data2 = data.clone();

    let job1 = Job::new(move || *moved_data1.borrow_mut() = 1);
    let job2 = Job::new(move || *moved_data2.borrow_mut() = 2);

    let push1 = duplicated_buffer.push(job1);
    let push2 = duplicated_buffer.push(job2);

    assert!(push1.is_ok());
    assert!(push2.is_ok());

    let pop1 = original_buffer.steal();
    let steal2 = original_buffer.pop();

    assert!(pop1.is_ok());
    assert!(steal2.is_ok());

    pop1.unwrap().invoke();
    assert_eq!(*data.borrow(), 1);
    steal2.unwrap().invoke();
    assert_eq!(*data.borrow(), 2);
}

// should_push_and_pop_from_different_threads
// should_push_and_steal_from_different_threads

// should_push_on_empty_buffer_from_multiple_threads
// should_push_on_full_buffer_from_multiple_threads

// should_pop_from_empty_buffer_from_multiple_threads
// should_pop_from_full_buffer_from_multiple_threads
// should_pop_from_partially_filled_buffer_from_multiple_threads

// should_steal_from_empty_buffer_from_multiple_threads
// should_steal_from_full_buffer_from_multiple_threads
// should_steal_from_partially_filled_buffer_from_multiple_threads

// should_push_from_multiple_threads_while_one_is_popping_on_empty_buffer
// should_push_from_multiple_threads_while_one_is_popping_on_full_buffer
// should_push_from_multiple_threads_while_multiple_threads_are_popping_on_empty_buffer
// should_push_from_multiple_threads_while_multiple_threads_are_popping_on_full_buffer

// should_push_from_multiple_threads_while_one_is_stealing_on_empty_buffer
// should_push_from_multiple_threads_while_one_is_stealing_on_full_buffer
// should_push_from_multiple_threads_while_multiple_threads_are_stealing_on_empty_buffer
// should_push_from_multiple_threads_while_multiple_threads_are_stealing_on_full_buffer
