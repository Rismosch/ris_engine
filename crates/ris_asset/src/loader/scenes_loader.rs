use ris_error::RisResult;

use crate::loader::ris_loader;
use crate::AssetId;

pub struct Scenes {
    pub material: Material,
}

#[derive(Clone)]
pub struct Material {
    pub vertex_shader: AssetId,
    pub fragment_shader: AssetId,
}

pub fn load(bytes: &[u8]) -> RisResult<Scenes> {
    let data = ris_error::unroll_option!(
        ris_loader::load(bytes)?,
        "failed to load ris asset from scenes"
    )?;

    let vertex_shader = data.references[0].clone();
    let fragment_shader = data.references[1].clone();
    let material = Material {
        vertex_shader,
        fragment_shader,
    };

    Ok(Scenes { material })
}
