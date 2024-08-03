pub use ris_log::appender::IAppender;
pub use ris_log::log_message::LogMessage;

pub struct ConsoleAppender;

impl IAppender for ConsoleAppender {
    fn print(&mut self, message: &LogMessage) {
        #[cfg(feature = "testing")]
        {
            eprintln!("\n{}", message.fmt(true));
        }

        #[cfg(not(feature = "testing"))]
        {
            _ = message;
        }
    }
}

