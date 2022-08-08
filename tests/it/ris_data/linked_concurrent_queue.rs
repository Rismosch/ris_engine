use std::{thread::JoinHandle, sync::{Arc, Mutex}};

use ris_data::linked_concurrent_queue::LinkedConcurrentQueue;

#[test]
fn should_push_and_pop() {
    let mut queue = LinkedConcurrentQueue::new();

    assert_eq!(queue.pop(), None);

    queue.push(10);
    assert_eq!(queue.pop(), Some(10));
    assert_eq!(queue.pop(), None);

    queue.push(10);
    queue.push(20);
    queue.push(30);

    assert_eq!(queue.pop(), Some(10));

    queue.push(40);

    assert_eq!(queue.pop(), Some(20));
    assert_eq!(queue.pop(), Some(30));
    assert_eq!(queue.pop(), Some(40));
    assert_eq!(queue.pop(), None);
    assert_eq!(queue.pop(), None);
}

#[test]
fn should_push_and_pop_from_different_threads(){
    let thread_count= 10;
    let timeout = std::time::Duration::from_secs(5);

    for _ in 0..1 {
        let mut handles = Vec::new();
        let mut queue = LinkedConcurrentQueue::new();
        let mut results = Vec::new();

        for i in 0..thread_count {
            let mut thread_queue = queue.clone();
            let thread = std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(i));

                thread_queue.push(i);
            });

            handles.push(thread);
        }

        for i in 0..thread_count {
            let now = std::time::Instant::now();
            loop {
                let result = queue.pop();
                if let Some(result) = result {
                    results.push(result);
                    break;
                }

                let duration = std::time::Instant::now() - now;
                if duration > timeout {
                    panic!("didn't expect queue to be empty for {:?}", duration)
                }

                std::thread::yield_now();
            }
        }

        for handle in handles {
            let _ = handle.join();
        }

        println!("{:?}", results);

        assert_eq!(results.len(), thread_count as usize);
        for i in 0..thread_count {
            assert!(results.contains(&i))
        }
    }
}