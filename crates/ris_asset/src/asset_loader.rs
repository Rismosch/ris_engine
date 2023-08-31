use std::path::Path;
use std::path::PathBuf;

use ris_data::info::app_info::AppInfo;
use ris_jobs::job_future::JobFuture;
use ris_jobs::job_future::SettableJobFuture;
use ris_util::ris_error::RisError;

use crate::asset_id::AssetId;

pub struct AssetLoader {}

impl AssetLoader {
    pub fn new(app_info: &AppInfo) -> Result<Self, RisError> {

        let asset_path;

        // check if path is relative
        let mut relative_path = PathBuf::new();
        relative_path.push(&app_info.file.base_path);
        relative_path.push(String::from(&app_info.args.assets));
        let relative_path = Path::new(&relative_path);
        if relative_path.exists() {
            asset_path = relative_path;
        } else {

        }

        // check if path is absolute


        Ok(Self {})
    }

    //pub fn load(id: AssetId) -> JobFuture<Box<[u8]>> {
    //    let (job_future, settable_job_future) = SettableJobFuture::new();
    //    job_future
    //}
}
