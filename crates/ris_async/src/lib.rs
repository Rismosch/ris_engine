pub mod affinity;
pub mod job_channel;
pub mod job_future;
pub mod oneshot_channel;
pub mod spin_lock;
pub mod thread_pool;

pub use job_channel::job_channel;
pub use job_channel::JobReceiver;
pub use job_channel::JobSender;
pub use job_channel::JobStealer;
pub use job_future::JobFuture;
pub use job_future::JobFutureSetter;
pub use oneshot_channel::oneshot_channel;
pub use oneshot_channel::OneshotSender;
pub use oneshot_channel::OneshotReceiver;
pub use spin_lock::SpinLock;
pub use spin_lock::SpinLockGuard;
pub use thread_pool::ThreadPool;
pub use thread_pool::ThreadPoolCreateInfo;
pub use thread_pool::ThreadPoolGuard;

pub const DEFAULT_BUFFER_CAPACITY: usize = 1024;
