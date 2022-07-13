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
macro_rules! log {
    ($($arg:tt)*) => {
        let formatted_message = format!($($arg)*);

        unsafe {
            for appender in ris_log::log::APPENDERS.iter() {
                appender.print(&formatted_message);
            }
        }
    };
}