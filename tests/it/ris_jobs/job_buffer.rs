use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

use ris_jobs::{job::Job, job_buffer::JobBuffer};
use ris_util::retry::retry;

//-----------------------//
//                       //
// single threaded tests //
//                       //
//-----------------------//

#[test]
fn should_push_and_pop() {
    let mut job_buffer = JobBuffer::new(4);

    let data = Rc::new(RefCell::new(0));
    let moved_data = data.clone();
    let job = Job::new(move || *moved_data.borrow_mut() = 42);

    let push = job_buffer.push(job);
    assert!(push.is_ok());

    let job = job_buffer.wait_and_pop();
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

    push3.err().unwrap().not_pushed.invoke();
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

    let pop1 = job_buffer.wait_and_pop();
    let pop2 = job_buffer.wait_and_pop();
    let pop3 = job_buffer.wait_and_pop();

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

        push6.err().unwrap().not_pushed.invoke();
        assert_eq!(*data.borrow(), 6);

        let steal1 = job_buffer.steal();
        let pop2 = job_buffer.wait_and_pop();
        let steal3 = job_buffer.steal();
        let pop4 = job_buffer.wait_and_pop();
        let steal5 = job_buffer.steal();
        let pop6 = job_buffer.wait_and_pop();
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

    let pop1 = duplicated_buffer.wait_and_pop();
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
    let steal2 = original_buffer.wait_and_pop();

    assert!(pop1.is_ok());
    assert!(steal2.is_ok());

    pop1.unwrap().invoke();
    assert_eq!(*data.borrow(), 1);
    steal2.unwrap().invoke();
    assert_eq!(*data.borrow(), 2);
}

//----------------------//
//                      //
// multi threaded tests //
//                      //
//----------------------//

#[test]
fn should_steal_from_empty_buffer_from_multiple_threads() {
    let mut buffer = JobBuffer::new(1000);
    let mut handles = Vec::new();
    let results = Arc::new(Mutex::new(Vec::new()));

    for _ in 0..1000 {
        let mut copied_buffer = buffer.duplicate();
        let copied_results = results.clone();
        let handle = thread::spawn(move || {
            let result = copied_buffer.steal();
            copied_results.lock().unwrap().push(result.is_err());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let results = results.lock().unwrap();
    for i in 0..1000 {
        assert!(results[i], "{:?}", results);
    }

    let mut unsuccessful_steals = 0;
    while buffer.wait_and_pop().is_ok() {
        unsuccessful_steals += 1;
    }

    assert_eq!(unsuccessful_steals, 0);
}

#[test]
fn should_steal_from_full_buffer_from_multiple_threads() {
    retry(5, ||{
        let mut buffer = JobBuffer::new(1000);
        let mut handles = Vec::new();
        let results = Arc::new(Mutex::new(Vec::new()));

        for _ in 0..1000 {
            let job = Job::new(|| {});
            buffer.push(job).unwrap();
        }

        for _ in 0..1000 {
            let mut copied_buffer = buffer.duplicate();
            let copied_results = results.clone();
            let handle = thread::spawn(move || {
                let result = copied_buffer.steal();
                copied_results.lock().unwrap().push(result.is_ok());
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let results = results.lock().unwrap();
        let mut successful_steals = 0;
        for i in 0..1000 {
            assert_eq!(results.len(), 1000);
            if results[i] {
                successful_steals += 1;
            }
        }

        let mut unsuccessful_steals = 0;
        while buffer.wait_and_pop().is_ok() {
            unsuccessful_steals += 1;
        }

        assert!(successful_steals > 990, "successful_steals {}", successful_steals);
        assert_eq!(successful_steals + unsuccessful_steals, 1000);
    });
}

#[test]
fn should_steal_from_partially_filled_buffer_from_multiple_threads() {
    let mut buffer = JobBuffer::new(1000);
    let mut handles = Vec::new();
    let results = Arc::new(Mutex::new(Vec::new()));

    for _ in 0..50 {
        let job = Job::new(|| {});
        buffer.push(job).unwrap();
    }

    for _ in 0..1000 {
        let mut copied_buffer = buffer.duplicate();
        let copied_results = results.clone();
        let handle = thread::spawn(move || {
            let result = copied_buffer.steal();
            copied_results.lock().unwrap().push(result.is_ok());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let results = results.lock().unwrap();
    let mut successful_steals = 0;
    for i in 0..1000 {
        assert_eq!(results.len(), 1000);
        if results[i] {
            successful_steals += 1;
        }
    }

    let mut unsuccessful_steals = 0;
    while buffer.wait_and_pop().is_ok() {
        unsuccessful_steals += 1;
    }

    assert_eq!(successful_steals, 50);
    assert_eq!(unsuccessful_steals, 0);
}

#[test]
fn should_push_from_one_thread_while_one_is_stealing_on_empty_buffer() {
    retry(5, ||{
        let mut buffer = JobBuffer::new(1000);
        let push_results = Arc::new(Mutex::new(Vec::new()));
        let steal_results = Arc::new(Mutex::new(Vec::new()));
        
        let mut push_buffer = buffer.duplicate();
        let push_results_copy = push_results.clone();
        let push_handle = thread::spawn(move || {
            for _ in 0..1000 {
                let result = push_buffer.push(Job::new(|| {}));
                push_results_copy.lock().unwrap().push(result.is_ok());
            }
        });
    
        let mut steal_buffer = buffer.duplicate();
        let steal_results_copy = steal_results.clone();
        let steal_handle = thread::spawn(move || {
            for _ in 0..1000 {
                let result = steal_buffer.steal();
                steal_results_copy.lock().unwrap().push(result.is_ok());
            }
        });
    
        push_handle.join().unwrap();
        steal_handle.join().unwrap();
    
        let push_results = push_results.lock().unwrap();
        let steal_results = steal_results.lock().unwrap();
    
        assert_eq!(push_results.len(), 1000);
        assert_eq!(steal_results.len(), 1000);
    
        let mut successful_pushes = 0;
        let mut successful_steals = 0;
        for i in 0..1000 {
            if push_results[i] {
                successful_pushes += 1;
            }
    
            if steal_results[i] {
                successful_steals += 1;
            }
        }
    
        assert!(successful_pushes > 990, "successful_pushes: {}", successful_pushes);
        assert!(successful_steals > 990, "successful_steals: {}", successful_steals);
    });
}

#[test]
fn should_push_from_one_thread_while_one_is_stealing_on_full_buffer() {
    retry(5, ||{
        let mut buffer = JobBuffer::new(1000);
        let push_results = Arc::new(Mutex::new(Vec::new()));
        let steal_results = Arc::new(Mutex::new(Vec::new()));
        
        for _ in 0..1000 {
            buffer.push(Job::new(||{})).unwrap();
        }

        let mut push_buffer = buffer.duplicate();
        let push_results_copy = push_results.clone();
        let push_handle = thread::spawn(move || {
            for _ in 0..1000 {
                let result = push_buffer.push(Job::new(|| {}));
                push_results_copy.lock().unwrap().push(result.is_ok());
            }
        });
    
        let mut steal_buffer = buffer.duplicate();
        let steal_results_copy = steal_results.clone();
        let steal_handle = thread::spawn(move || {
            for _ in 0..1000 {
                let result = steal_buffer.steal();
                steal_results_copy.lock().unwrap().push(result.is_ok());
            }
        });
    
        push_handle.join().unwrap();
        steal_handle.join().unwrap();
    
        let push_results = push_results.lock().unwrap();
        let steal_results = steal_results.lock().unwrap();
    
        assert_eq!(push_results.len(), 1000);
        assert_eq!(steal_results.len(), 1000);
    
        let mut _successful_pushes = 0;
        let mut successful_steals = 0;
        for i in 0..1000 {
            if push_results[i] {
                _successful_pushes += 1;
            }
    
            if steal_results[i] {
                successful_steals += 1;
            }
        }
    
        // pushes are super unreliable. successful_pushes can be everything between
        // 0 or 1000. in this edgecase, i am just happy that nothing panics
        assert!(successful_steals > 990, "successful_steals: {}", successful_steals);
    });
}


#[test]
fn should_push_from_one_thread_while_multiple_are_stealing_on_empty_buffer() {
    retry(5, ||{
        let mut buffer = JobBuffer::new(1000);
        let push_results = Arc::new(Mutex::new(Vec::new()));
        let steal_results = Arc::new(Mutex::new(Vec::new()));
        
        let mut push_buffer = buffer.duplicate();
        let push_results_copy = push_results.clone();
        let push_handle = thread::spawn(move || {
            for _ in 0..1000 {
                let result = push_buffer.push(Job::new(|| {}));
                push_results_copy.lock().unwrap().push(result.is_ok());
            }
        });
    
        let mut steal_handles = Vec::new();
        for _ in 0..100 {
            let mut steal_buffer = buffer.duplicate();
            let steal_results_copy = steal_results.clone();
            let steal_handle = thread::spawn(move || {
                for _ in 0..10 {
                    let result = steal_buffer.steal();
                    steal_results_copy.lock().unwrap().push(result.is_ok());
                }
            });
            steal_handles.push(steal_handle);
        }
    
        push_handle.join().unwrap();
        for handle in steal_handles {
            handle.join().unwrap();
        }
    
        let push_results = push_results.lock().unwrap();
        let steal_results = steal_results.lock().unwrap();
    
        assert_eq!(push_results.len(), 1000);
        assert_eq!(steal_results.len(), 1000);
    
        let mut successful_pushes = 0;
        let mut successful_steals = 0;
        for i in 0..1000 {
            if push_results[i] {
                successful_pushes += 1;
            }
    
            if steal_results[i] {
                successful_steals += 1;
            }
        }
    
        assert!(successful_pushes > 990, "successful_pushes: {}", successful_pushes);
        assert!(successful_steals > 990, "successful_steals: {}", successful_steals);
    });
}

#[test]
fn should_push_from_one_thread_while_multiple_are_stealing_on_full_buffer() {
    retry(5, ||{
        let mut buffer = JobBuffer::new(1000);
        let push_results = Arc::new(Mutex::new(Vec::new()));
        let steal_results = Arc::new(Mutex::new(Vec::new()));
        
        for _ in 0..1000 {
            buffer.push(Job::new(||{})).unwrap();
        }

        let mut push_buffer = buffer.duplicate();
        let push_results_copy = push_results.clone();
        let push_handle = thread::spawn(move || {
            for _ in 0..1000 {
                let result = push_buffer.push(Job::new(|| {}));
                push_results_copy.lock().unwrap().push(result.is_ok());
            }
        });
    
        let mut steal_handles = Vec::new();
        for _ in 0..100 {
            let mut steal_buffer = buffer.duplicate();
            let steal_results_copy = steal_results.clone();
            let steal_handle = thread::spawn(move || {
                for _ in 0..10 {
                    let result = steal_buffer.steal();
                    steal_results_copy.lock().unwrap().push(result.is_ok());
                }
            });
            steal_handles.push(steal_handle);
        }
    
        push_handle.join().unwrap();
        for handle in steal_handles {
            handle.join().unwrap();
        }
    
        let push_results = push_results.lock().unwrap();
        let steal_results = steal_results.lock().unwrap();
    
        assert_eq!(push_results.len(), 1000);
        assert_eq!(steal_results.len(), 1000);
    
        let mut _successful_pushes = 0;
        let mut successful_steals = 0;
        for i in 0..1000 {
            if push_results[i] {
                _successful_pushes += 1;
            }
    
            if steal_results[i] {
                successful_steals += 1;
            }
        }
    
        // pushes are super unreliable. successful_pushes can be everything between
        // 0 or 1000. in this edgecase, i am just happy that nothing panics
        assert!(successful_steals > 990, "successful_steals: {}", successful_steals);
    });
}

// should_pop_from_one_thread_while_one_is_stealing_on_empty_buffer
// should_pop_from_one_thread_while_one_is_stealing_on_full_buffer
// should_pop_from_one_thread_while_multiple_are_stealing_on_empty_buffer
// should_pop_from_one_thread_while_multiple_are_stealing_on_full_buffer

// should_push_and_pop_from_one_thread_while_one_is_stealing_on_empty_buffer
// should_push_and_pop_from_one_thread_while_mutliple_are_stealing_on_empty_buffer
