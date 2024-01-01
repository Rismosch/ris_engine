use std::sync::Arc;

use vulkano::device::Device;
use vulkano::shader::ShaderModule;

use ris_asset::loader::scenes_loader::Material;
use ris_error::RisResult;

pub type Shaders = (Arc<ShaderModule>, Arc<ShaderModule>);

pub fn load_shaders(device: &Arc<Device>, material: &Material) -> RisResult<Shaders> {
    let vertex_id = material.vertex_shader.clone();
    let fragmend_id = material.fragment_shader.clone();

    ris_log::trace!(
        "loading shaders: vert: {:?} frag {:?}",
        vertex_id,
        fragmend_id
    );

    let vertex_future = ris_asset::asset_loader::load(vertex_id);
    let fragment_future = ris_asset::asset_loader::load(fragmend_id);

    let vertex_bytes = ris_error::unroll!(vertex_future.wait(), "failed to load vertex asset")?;
    let fragment_bytes =
        ris_error::unroll!(fragment_future.wait(), "failed to load fragment asset")?;

    let vertex_shader = ris_error::unroll!(
        unsafe { ShaderModule::from_bytes(device.clone(), &vertex_bytes) },
        "failed to load vertex shader module"
    )?;

    let fragment_shader = ris_error::unroll!(
        unsafe { ShaderModule::from_bytes(device.clone(), &fragment_bytes) },
        "failed to lad fragment shader module"
    )?;

    Ok((vertex_shader, fragment_shader))
}
