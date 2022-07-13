use crate::log_level::LogLevel;
use chrono::Utc;

pub trait IAppender {
    fn print(&self, message: &str);
}

pub static mut SHOULD_LOG_FILENAMES: bool = false;
pub static mut LOG_LEVEL: LogLevel = LogLevel::None;
pub static mut APPENDERS: Vec<Box<dyn IAppender>> = Vec::new();

pub fn init(priority: LogLevel, should_log_filenames: bool) {
    unsafe {
        LOG_LEVEL = priority;
        SHOULD_LOG_FILENAMES = should_log_filenames;
    }
}

pub fn register_appender<TAppender: 'static + IAppender>(appender: TAppender) {
    let wrapped_appender = Box::new(appender);
    unsafe {
        APPENDERS.push(wrapped_appender);
    }
}

pub fn get_current_time_string() -> String {
    format!("{}", Utc::now().format("%T"))
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        ris_log::log!(ris_log::log_level::LogLevel::Trace, $($arg)*);
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        ris_log::log!(ris_log::log_level::LogLevel::Debug, $($arg)*);
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        ris_log::log!(ris_log::log_level::LogLevel::Info, $($arg)*);
    };
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        ris_log::log!(ris_log::log_level::LogLevel::Warning, $($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        ris_log::log!(ris_log::log_level::LogLevel::Error, $($arg)*);
    };
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        ris_log::log!(ris_log::log_level::LogLevel::Fatal, $($arg)*);
    };
}

#[macro_export]
macro_rules! log {
    ($priority:expr, $($arg:tt)*) => {
        if (unsafe {$priority as u8 >= ris_log::log::LOG_LEVEL as u8}) {
            let package_name = env!("CARGO_PKG_NAME");
            let current_time = ris_log::log::get_current_time_string();
            let formatted_message = format!($($arg)*);
            
            let filename = if unsafe {ris_log::log::SHOULD_LOG_FILENAMES} {
                format!("\n at {}:{}\n",file!(),line!())
            } else {
                String::from("")
            };
            
            let message_to_print = format!(
                "<{}> {} [{}]: {}{}",
                package_name,
                $priority,
                current_time,
                formatted_message,
                filename
            );

            ris_log::forward_to_appenders!("{}",message_to_print);
        }
    };
}

#[macro_export]
macro_rules! forward_to_appenders {
    ($($arg:tt)*) => {
        let message = format!($($arg)*);

        unsafe {
            for appender in ris_log::log::APPENDERS.iter() {
                appender.print(&message);
            }
        }
    };
}
