use crate::log_message::LogMessage;

pub trait IAppender {
    fn print(&mut self, message: &LogMessage);
}
