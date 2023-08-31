use std::path::Path;
use std::path::PathBuf;

use ris_data::info::app_info::AppInfo;
use ris_jobs::job_future::JobFuture;
use ris_jobs::job_future::SettableJobFuture;

use crate::asset_id::AssetId;

pub struct AssetLoader {}

impl AssetLoader {
    pub fn new(app_info: &AppInfo) -> Self {
        let args_assets_path = String::from(app_info.args.assets);

        // check if path is relative
        let mut asset_path = PathBuf::new();
        asset_path.push(&app_info.file.base_path);
        asset_path.push();

        // check if path is absolute
        

        Self {}
    }

    //pub fn load(id: AssetId) -> JobFuture<Box<[u8]>> {
    //    let (job_future, settable_job_future) = SettableJobFuture::new();
    //    job_future
    //}
}
