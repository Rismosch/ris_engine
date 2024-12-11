use ris_log::log::IAppender;
use ris_log::log_message::LogMessage;

pub struct ConsoleAppender;

impl IAppender for ConsoleAppender {
    fn print(&mut self, message: &LogMessage) {
        eprintln!("\n{}", message.fmt(true));
    }
}
