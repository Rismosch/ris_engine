use crate::i_appender::IAppender;

pub struct ConsoleAppender {}

impl IAppender for ConsoleAppender {
    fn print(&self, message: &str) {
        println!("{}", message);
    }
}
