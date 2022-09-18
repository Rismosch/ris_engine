use ris_log::i_appender::IAppender;

pub struct ConsoleAppender;

impl ConsoleAppender {
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl IAppender for ConsoleAppender {
    fn print(&mut self, message: &str) {
        println!("{}\n", message);
    }
}
