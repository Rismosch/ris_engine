pub mod affinity;
pub mod channel;
pub mod job_future;
pub mod thread_pool;
pub mod spin_lock;

pub use channel::Channel;
pub use channel::Sender;
pub use channel::Stealer;
pub use channel::Receiver;
pub use job_future::JobFuture;
pub use job_future::JobFutureSetter;
pub use thread_pool::ThreadPool;
pub use thread_pool::ThreadPoolCreateInfo;
pub use spin_lock::SpinLock;
pub use spin_lock::SpinLockGuard;

pub const DEFAULT_BUFFER_CAPACITY: usize = 1024;
