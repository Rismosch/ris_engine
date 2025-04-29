pub mod god_job;
pub mod god_object;
pub mod inspector_util;
pub mod output_frame;

pub mod log_appenders;

pub mod ui_helper;

use ris_data::info::app_info::AppInfo;
use ris_data::settings::Settings;

pub fn determine_thread_count(app_info: &AppInfo, settings: &Settings) -> usize {
    if let Some(workers) = app_info.args.workers {
        workers
    } else {
        settings.job().workers()
    }
}
