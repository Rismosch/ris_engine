use std::path::Path;

use ris_error::prelude::*;

pub const IN_EXT_GLB: &str = "glb";

pub fn import(source: impl AsRef<Path>, target_dir: impl AsRef<Path>) -> RisResult<()> {
    let source = source.as_ref();
    let target_dir = target_dir.as_ref();


    ris_error::new_result!(
        "importing gltf {} {}",
        source.display(),
        target_dir.display(),
    )
}
