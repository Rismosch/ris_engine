use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use ris_log::appenders::i_appender::IAppender;

pub struct BlockingAppender {
    messages: Arc<Mutex<Vec<String>>>,
    timeout: Duration,
}

impl BlockingAppender {
    pub fn new(timeout: Duration) -> (Box<Self>, Arc<Mutex<Vec<String>>>) {
        let messages = Arc::new(Mutex::new(Vec::new()));

        let appender = Box::new(Self {
            messages: messages.clone(),
            timeout,
        });

        (appender, messages)
    }
}

impl IAppender for BlockingAppender {
    fn print(&mut self, message: &str) {
        let mut messages = self.messages.lock().unwrap();

        thread::sleep(self.timeout);

        messages.push(String::from(message));
    }
}
