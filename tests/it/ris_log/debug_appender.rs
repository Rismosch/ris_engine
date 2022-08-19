use std::sync::{Arc, Mutex};

use ris_log::appenders::i_appender::IAppender;

pub struct DebugAppender {
    messages: Arc<Mutex<Vec<String>>>,
}

impl DebugAppender {
    pub fn new() -> (Box<Self>, Arc<Mutex<Vec<String>>>) {
        let messages = Arc::new(Mutex::new(Vec::new()));

        let appender = Box::new(Self {
            messages: messages.clone(),
        });

        (appender, messages)
    }
}

impl IAppender for DebugAppender {
    fn print(&mut self, message: &str) {
        let mut messages = self.messages.lock().unwrap();
        messages.push(String::from(message));
    }
}
