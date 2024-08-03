use crate::appender::IAppender;
use crate::log_level::LogLevel;
use crate::log_message::LogMessage;

pub struct LogGuard;

/// # Safety
///
/// The logger is a singleton. Initialize it only once.
pub unsafe fn init(
    _log_level: LogLevel,
    _appenders: Vec<Box<dyn IAppender + Send>>,
) -> LogGuard {
    LogGuard
}

pub fn forward_to_appenders(_log_message: LogMessage) {}

#[macro_export]
macro_rules! trace {
    ($($arg:expr),* $(,)?) => {{
        $(let _ = &$arg;)*
    }};
}

#[macro_export]
macro_rules! debug {
    ($($arg:expr),* $(,)?) => {{
        $(let _ = &$arg;)*
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:expr),* $(,)?) => {{
        $(let _ = &$arg;)*
    }};
}

#[macro_export]
macro_rules! warning {
    ($($arg:expr),* $(,)?) => {{
        $(let _ = &$arg;)*
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:expr),* $(,)?) => {{
        $(let _ = &$arg;)*
    }};
}

#[macro_export]
macro_rules! fatal {
    ($($arg:expr),* $(,)?) => {{
        $(let _ = &$arg;)*
    }};
}

#[macro_export]
macro_rules! log {
    ($priority:expr, $($arg:expr),* $(,)?) => {{
        let _ = $priority;
        $(let _ = $arg;)*
    }};
}
