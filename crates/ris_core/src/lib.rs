pub mod god_job;
pub mod god_object;
pub mod logic_frame;
pub mod output_frame;
pub mod ui_helper;

use ris_data::info::app_info::AppInfo;
use ris_data::settings::Settings;

pub fn determine_thread_count(app_info: &AppInfo, settings: &Settings) -> usize {
    if let Some(workers) = app_info.args.workers {
        workers
    } else if let Some(workers) = settings.job().get_workers() {
        workers
    } else {
        app_info.cpu.cpu_count
    }
}
