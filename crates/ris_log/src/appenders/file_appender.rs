use std::path::PathBuf;

use ris_data::info::app_info::AppInfo;

use super::i_appender::IAppender;

const LOG_DIRERCTORY_NAME: &str = "logs";
const LOG_FILE_NAME: &str = "current.log";
const OLD_LOG_DIRERCTORY: &str = "old";
const OLD_LOG_COUNT: u8 = 10;

pub struct FileAppender {
    old_log_directory: PathBuf,
    current_log_path: PathBuf,
}

impl FileAppender {
    pub fn new(app_info: &AppInfo) -> Box<Self> {
        let file_appender = Self::construct(app_info);
        file_appender.create_old_directory();
        file_appender.delete_expired_logs();
        file_appender.move_current_log();
        file_appender.create_new_log_file();

        Box::new(file_appender)
    }
}

impl IAppender for FileAppender {
    fn print(&mut self, message: &str) {
        println!("FILEAPPENDER {}\n", message);
    }
}

impl FileAppender {
    fn construct(app_info: &AppInfo) -> Self {
        let mut log_directory = PathBuf::new();
        log_directory.push(&app_info.file.pref_path);
        log_directory.push(LOG_DIRERCTORY_NAME);

        let mut current_log_path = PathBuf::new();
        current_log_path.push(log_directory.clone());
        current_log_path.push(LOG_FILE_NAME);

        let mut old_log_directory = PathBuf::new();
        old_log_directory.push(log_directory);
        old_log_directory.push(OLD_LOG_DIRERCTORY);

        Self {
            current_log_path,
            old_log_directory,
        }
    }
    
    fn create_old_directory(&self) {
        if !&self.old_log_directory.exists() {
            if let Err(error) = std::fs::create_dir_all(&self.old_log_directory) {
                panic!("couldn't create \"{:?}\": {}", &self.old_log_directory, error);
            };
        }
    }
    
    fn delete_expired_logs(&self) {
        let entries = match std::fs::read_dir(&self.old_log_directory) {
            Ok(entries) => entries,
            Err(error) => panic!("couldn't read \"{:?}\": {}", self.old_log_directory, error),
        };

        for entry in entries {
            match entry {
                Ok(entry) => {
                    println!("{:?} {:?} {:?} {:?}\n\n", entry.file_type(), entry.file_name(), entry.metadata(), entry.path())
                },
                Err(error) => panic!("couldn't read old log entry: {}", error),
            }
        }
    }
    
    fn move_current_log(&self) {
    
    }
    
    fn create_new_log_file(&self) {
    
    }
}