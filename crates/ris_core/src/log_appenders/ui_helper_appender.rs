use std::sync::Mutex;

use ris_error::Extensions;
use ris_error::RisResult;
use ris_log::log::IAppender;
use ris_log::log_message::LogMessage;

pub static MESSAGES: Mutex<Option<Vec<LogMessage>>> = Mutex::new(None);

pub struct UiHelperAppender {
    _boo: (),
}

impl UiHelperAppender {
    pub fn new() -> RisResult<Self> {
        let mut messages = MESSAGES.lock()?;
        *messages = Some(Vec::new());

        Ok(Self { _boo: () })
    }
}

impl Drop for UiHelperAppender {
    fn drop(&mut self) {
        match MESSAGES.lock() {
            Err(e) => eprintln!("error while dropping ui helper appender: {}", e),
            Ok(mut messages) => {
                messages.take();
            }
        }
    }
}

impl IAppender for UiHelperAppender {
    fn print(&mut self, message: &LogMessage) {
        let mut mutex_guard = ris_error::unwrap!(MESSAGES.lock(), "failed to lock messages");

        let messages = ris_error::unwrap!(
            mutex_guard.as_mut().into_ris_error(),
            "messages were not initialized",
        );

        messages.push(message.clone());
    }
}
