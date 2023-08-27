use std::path::PathBuf;

use ris_data::info::app_info::AppInfo;
use ris_jobs::job_future::JobFuture;
use ris_jobs::job_future::SettableJobFuture;

use crate::asset_id::AssetId;

pub const ASSET_FILE_NAME: &str = ".ris_asset";

pub struct AssetLoader{
}

impl AssetLoader{
    pub fn new(app_info: &AppInfo) -> Self {
        let mut asset_path = PathBuf::new();
        asset_path.push(&app_info.file.base_path);
        asset_path.push(ASSET_FILE_NAME);



        Self{}
    }

    //pub fn load(id: AssetId) -> JobFuture<Box<[u8]>> {
    //    let (job_future, settable_job_future) = SettableJobFuture::new();
    //    job_future
    //}
}
