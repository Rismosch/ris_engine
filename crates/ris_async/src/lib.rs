pub mod affinity;
pub mod channel;
pub mod job_channel;
pub mod job_future;
pub mod spin_lock;
pub mod thread_pool;

pub use channel::SingleUseReceiver;
pub use channel::SingleUseSender;
pub use channel::single_use_channel;
pub use channel::UnsafeSender;
pub use channel::UnsafeReceiver;
pub use channel::unsafe_channel;
pub use job_channel::JobReceiver;
pub use job_channel::JobSender;
pub use job_channel::JobStealer;
pub use job_channel::job_channel;
pub use job_future::JobFuture;
pub use job_future::JobFutureSetter;
pub use spin_lock::SpinLock;
pub use spin_lock::SpinLockGuard;
pub use thread_pool::ThreadPool;
pub use thread_pool::ThreadPoolCreateInfo;
pub use thread_pool::ThreadPoolGuard;

pub const DEFAULT_BUFFER_CAPACITY: usize = 1024;
