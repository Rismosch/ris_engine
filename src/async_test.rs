use std::future::Future;
use std::pin::pin;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;
use std::task::Wake;
use std::thread::Thread;

pub fn run() {
    let mut futures = Vec::with_capacity(10);
    for i in 0..futures.capacity() {
        let future = math(i);
        futures.push(future);
    }

    for future in futures {
        let result = block_on(future);
        println!("result: {}", result);
    }
}

async fn math(mut value: usize) -> usize {
    println!("hello from math {}", value);
    value = mul_ten(value).await;
    value = add_one(value).await;
    value
}


async fn add_one(value: usize) -> usize {
    println!("hello from add_one {}", value);
    value + 1
}

async fn mul_ten(value: usize) -> usize {
    println!("hello from mul_ten {}", value);
    value * 10
}

struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: std::sync::Arc<Self>) {
        self.0.unpark();
    }
}

fn block_on<F: Future>(mut future: F) -> F::Output {
    let mut future = pin!(future);

    let t = std::thread::current();
    let waker = Arc::new(ThreadWaker(t)).into();
    let mut context = Context::from_waker(&waker);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(result) => return result,
            Poll::Pending => std::thread::park(),
        }
    }
}
