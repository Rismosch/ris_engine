pub fn run() {
    let cpu_count = sdl2::cpuinfo::cpu_count() as usize;
    let thread_pool = ris_async::thread_pool::ThreadPool::new(
        1024,
        cpu_count,
        cpu_count,
        true,
    );


}


