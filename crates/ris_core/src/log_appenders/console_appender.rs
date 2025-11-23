use ris_log::constructed_log_message::ConstructedLogFormatArgs;
use ris_log::log::IAppender;
use ris_log::log_message::LogMessage;

pub struct ConsoleAppender;

impl IAppender for ConsoleAppender {
    fn print(&mut self, message: &LogMessage) {
        let args = ConstructedLogFormatArgs {
            ansi_support: true,
            show_timestamp: false,
            show_priority: true,
            show_priority_padding: true,
            show_foot: false,
        };

        eprintln!("{}", message.fmt(args));
    }
}
