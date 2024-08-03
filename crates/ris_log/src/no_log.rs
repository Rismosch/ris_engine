use crate::appender::IAppender;
use crate::log_level::LogLevel;
use crate::log_message::LogMessage;

pub struct LogGuard;

/// # Safety
///
/// The logger is a singleton. Initialize it only once.
pub unsafe fn init(log_level: LogLevel, appenders: Vec<Box<dyn IAppender + Send>>) -> LogGuard {
    LogGuard
}

pub fn forward_to_appenders(_log_message: LogMessage) {}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{}};
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{}};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{}};
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {{}};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{}};
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {{}};
}

#[macro_export]
macro_rules! log {
    ($priority:expr, $($arg:tt)*) => {{}};
}
