pub mod error;
pub mod panic;

pub use error::OptionExtensions;
pub use error::ResultExtensions;
pub use error::RisError;
pub use error::RisResult;

pub mod prelude {
    pub use crate::OptionExtensions;
    pub use crate::ResultExtensions;
    pub use crate::RisError;
    pub use crate::RisResult;
}

pub static mut BACKTRACE_HOOK: Option<fn(&std::backtrace::Backtrace)> = None;
pub static mut FATAL_LOG_HOOK: Option<fn(&str)> = None;
pub static mut MESSAGE_BOX_HOOK: Option<fn(&str)> = None;
