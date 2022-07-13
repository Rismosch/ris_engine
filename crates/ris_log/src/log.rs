pub trait IAppender{
    fn print(&self, message: &str);
}

pub static mut APPENDERS: Vec<Box<dyn IAppender>> = Vec::new();

pub fn register_appender<TAppender: 'static + IAppender>(appender: TAppender)
{
    let wrapped_appender = Box::new(appender);
    unsafe {
        APPENDERS.push(wrapped_appender);
    }
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        ris_log::log!(ris_log::log_priority::LogPriority::Trace, $($arg)*);
    };
}

#[macro_export]
macro_rules! log {
    ($priority:expr, $($arg:tt)*) => {
        let formatted_message = format!($($arg)*);
        let message_to_print = format!("[ {} | {} | 17:55:03 ]: {}", std::env::var("CARGO_PKG_NAME").unwrap(), $priority, formatted_message);

        unsafe {
            for appender in ris_log::log::APPENDERS.iter() {
                appender.print(&message_to_print);
            }
        }
    };
}