use std::sync::Arc;

use vulkano::device::Device;
use vulkano::shader::ShaderModule;

use ris_asset::asset_loader;
use ris_asset::AssetId;
use ris_error::RisResult;
use ris_jobs::job_future::JobFuture;
use ris_jobs::job_system;

pub fn load_async(
    device: Arc<Device>,
    asset_id: AssetId,
) -> JobFuture<RisResult<Arc<ShaderModule>>> {
    job_system::submit(move || {
        ris_log::trace!("loading shader... {:?}", asset_id,);

        let future = asset_loader::load_async(asset_id.clone());

        let bytes = future.wait(None)??;

        let shader = unsafe { ShaderModule::from_bytes(device.clone(), &bytes) }?;

        ris_log::trace!("loaded shader! {:?}", asset_id,);

        Ok(shader)
    })
}
