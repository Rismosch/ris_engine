use std::io::Write;
use std::path::Path;

use sdl2::messagebox::MessageBoxFlag;

use ris_util::error::RisResult;
use ris_util::fallback_file::FallbackFileAppend;

const LOG_EXTENSION: &str = ".log";
const OLD_LOG_COUNT: usize = 10;

pub struct FileAppender {
    fallback_file: FallbackFileAppend,
}

impl FileAppender {
    pub fn new(directory: &Path) -> RisResult<Self> {
        let fallback_file = FallbackFileAppend::new(
            directory,
            LOG_EXTENSION,
            OLD_LOG_COUNT
        )?;

        Ok(Self{fallback_file})
    }

    pub fn print(&mut self, message: &str) {
        let file = self.fallback_file.current();
        let result = writeln!(file, "{}\n", message);

        if result.is_err() {
            let error_message = format!("failed to log the following message: {}", message);
            let _ = sdl2::messagebox::show_simple_message_box(
                MessageBoxFlag::ERROR,
                "log failed",
                &error_message,
                None,
            );
        }
    }
}

