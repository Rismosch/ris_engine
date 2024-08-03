use std::path::Path;

use ris_error::RisResult;
use ris_log::appender::IAppender;
use ris_log::log_message::LogMessage;

const LOG_EXTENSION: &str = ".log";
const OLD_LOG_COUNT: usize = 10;

pub struct FileAppender {
    #[cfg(feature = "testing")]
    fallback_file: ris_file::fallback_file::FallbackFileAppend,
}

impl FileAppender {
    pub fn new(directory: &Path) -> RisResult<Self> {
        #[cfg(feature = "testing")]
        {

            let fallback_file = ris_file::fallback_file::FallbackFileAppend::new(directory, LOG_EXTENSION, OLD_LOG_COUNT)?;

            Ok(Self { fallback_file })
        }

        #[cfg(not(feature = "testing"))]
        {
            let _ = directory;
            let _ = LOG_EXTENSION;
            let _ = OLD_LOG_COUNT;

            Ok(Self{})
        }
    }
}

impl IAppender for FileAppender {
    fn print(&mut self, message: &LogMessage) {
        #[cfg(feature = "testing")]
        {
            use std::io::Write;

            let to_log = message.fmt(false);

            let file = self.fallback_file.current();
            let result = writeln!(file, "\n{}", to_log);

            if result.is_err() {
                let error_message = format!("failed to log the following message: {}", to_log);
                let _ = sdl2::messagebox::show_simple_message_box(
                    sdl2::messagebox::MessageBoxFlag::ERROR,
                    "log failed",
                    &error_message,
                    None,
                );
            }
        }

        #[cfg(not(feature = "testing"))]
        {
            _ = message;
        }
    }
}
