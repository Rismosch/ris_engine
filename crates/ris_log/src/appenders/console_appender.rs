use super::i_appender::IAppender;

pub struct ConsoleAppender {}

impl ConsoleAppender {
    pub fn new() -> Box<ConsoleAppender> {
        let appender = ConsoleAppender {};
        Box::new(appender)
    }
}

impl IAppender for ConsoleAppender {
    fn print(&self, message: &str) {
        println!("{}", message);
    }
}
