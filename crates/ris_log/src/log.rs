use std::{thread::JoinHandle, sync::{atomic::{AtomicBool, Ordering}}};

use crate::{log_level::LogLevel, i_appender::IAppender};
use chrono::Utc;

pub struct Logger
{
    pub log_level: LogLevel,
    appenders: Vec<Box<dyn IAppender>>,
    stop_log_thread: AtomicBool,
    thread_handle: Option<JoinHandle<()>>
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.stop_log_thread.swap(true, Ordering::SeqCst);

        if let Some(thread_handle) = self.thread_handle.take(){
            let _ = thread_handle.join();
        }
    }
}

pub static mut LOG: Option<Logger> = None;

pub fn init(log_level: LogLevel, appenders: Vec<Box<dyn IAppender>>) {

    let thread_handle = Some(std::thread::spawn(log_thread));

    let log = Logger{
        log_level,
        appenders,
        stop_log_thread: AtomicBool::new(false),
        thread_handle,
    };

    unsafe {
        LOG = Some(log);
    }
}

pub fn drop(){
    unsafe {
        LOG = None;
    }
}

fn log_thread()
{
    unsafe {
        if let Some(log) = &LOG {

            loop {
                let should_stop_thread = log.stop_log_thread.load(Ordering::SeqCst);
                if should_stop_thread {
                    break;
                }
            }
        }
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
        {
            unsafe {
                if let Some(log) = &ris_log::log::LOG {
                    if ($priority as u8 >= log.log_level as u8) {
                        let package_name = env!("CARGO_PKG_NAME");
                        let current_time = ris_log::log::get_current_time_string();
                        let formatted_message = format!($($arg)*);
                        
                        let message_to_print = format!(
                            "[{}] {}: {}\n    in {} at {}:{}\n",
                            current_time,
                            $priority,
                            formatted_message,
                            package_name,
                            file!(),
                            line!()
                        );
            
                        ris_log::forward_to_appenders!("{}",message_to_print);
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! forward_to_appenders {
    ($($arg:tt)*) => {
        let message = format!($($arg)*);

        
    };
}
