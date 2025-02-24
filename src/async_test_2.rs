use ris_async::thread_pool::ThreadPool;

pub fn run() {
    let buffer_size = 1024;
    let cpu_count = sdl2::cpuinfo::cpu_count() as usize;
    let threads = cpu_count;
    let set_affinity = true;

    let _guard = ThreadPool::new(
        buffer_size,
        cpu_count,
        threads,
        set_affinity,
    ).unwrap();

    for i in 0..50 {
        ThreadPool::submit(hello(i));
    }
}

async fn hello(value: usize) {
    let result = ThreadPool::submit(mul_10(value)).await;
    println!(
        "hello {} * 10 = {}, thread: {:?}",
        value,
        result,
        std::thread::current().name()
    );
}

async fn mul_10(value: usize) -> usize {
    value * 10
}

