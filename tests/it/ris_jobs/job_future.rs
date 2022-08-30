use std::{task::Poll, thread, sync::{atomic::{AtomicBool, Ordering}, Arc}, time::{Instant, Duration}};

use ris_jobs::job_future::SettableJobFuture;
use ris_util::repeat::repeat;

#[test]
fn should_set_and_poll_on_single_thread(){
    let (mut settable, future) = SettableJobFuture::new();

    assert!(future.poll().is_pending());

    settable.set(String::from("hello world"));
    
    match future.poll().clone() {
        Poll::Pending => panic!("expected future to be reads"),
        Poll::Ready(result) => assert_eq!(result, "hello world"),
    }
}

#[test]
fn should_set_and_poll_on_different_threads(){
    repeat(100, || {
        const TIMEOUT: u128 = 100;

        let result = Arc::new(AtomicBool::new(false));
        let (mut settable, future) = SettableJobFuture::new();

        let set_handle = thread::spawn(move ||{
            thread::sleep(Duration::from_millis(1));
            settable.set(42);
        });

        let poll_result = result.clone();
        let poll_handle = thread::spawn(move || {
            let start = Instant::now();
            loop {
                match future.poll() {
                    Poll::Pending => {
                        let now = Instant::now();
                        let duration = now - start;
                        if duration.as_millis() > TIMEOUT {
                            break;
                        }
                    },
                    Poll::Ready(value) => {
                        assert_eq!(*value, 42);
                        poll_result.store(true, Ordering::SeqCst);
                        break;
                    },
                }
            }
        });

        set_handle.join().unwrap();
        poll_handle.join().unwrap();

        assert!(result.load(Ordering::SeqCst));
    });
}