use std::{thread::JoinHandle, sync::atomic::{AtomicBool, Ordering}};

use crate::{log_level::LogLevel, iappender::IAppender};
use chrono::Utc;

pub static mut LOG_LEVEL: LogLevel = LogLevel::None;
pub static mut SHOULD_LOG_FILENAMES: bool = false;
pub static mut APPENDERS: Vec<Box<dyn IAppender>> = Vec::new();

static mut THREAD_BLOCKED: AtomicBool = AtomicBool::new(false);
static mut STOP_LOG_THREAD: AtomicBool = AtomicBool::new(false);
static mut LOG_THREAD: Option<JoinHandle<()>> = None;


pub fn init(log_level: LogLevel, should_log_filenames: bool) {
    unsafe {
        LOG_LEVEL = log_level;
        SHOULD_LOG_FILENAMES = should_log_filenames;
        
        THREAD_BLOCKED.swap(false, Ordering::SeqCst);
        STOP_LOG_THREAD.swap(false, Ordering::SeqCst);
        LOG_THREAD = Some(std::thread::spawn(log_thread));
    }
}

pub fn log_thread()
{
    loop {
        unsafe {
            let should_stop_thread = STOP_LOG_THREAD.load(Ordering::SeqCst);
            if should_stop_thread {
                break;
            }

            // log logic here
        }
    }
}

pub fn drop(){
    unsafe {
        STOP_LOG_THREAD.swap(true, Ordering::SeqCst);
        if let Some(log_thread) = LOG_THREAD.take() {
            log_thread.join();
        }
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
