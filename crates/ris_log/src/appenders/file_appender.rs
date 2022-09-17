use ris_data::info::app_info::AppInfo;

use super::i_appender::IAppender;

pub struct FileAppender {}

impl FileAppender {
    pub fn new(app_info: &AppInfo) -> Box<FileAppender> {
        let pref_path = &app_info.file.pref_path;

        let appender = FileAppender {};
        Box::new(appender)
    }
}

impl IAppender for FileAppender {
    fn print(&mut self, message: &str) {
        println!("FILEAPPENDER {}\n", message);
    }
}
