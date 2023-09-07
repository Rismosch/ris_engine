use std::path::Path;

use ris_util::ris_error::RisError;

pub fn import(source: &str, target: &str) -> Result<(), RisError> {
    let source = Path::new(source);

    let extension = ris_util::unroll_option!(
        source.extension(),
        "failed to find extension from source {:?}",
        source
    )?;

    let extension = ris_util::unroll_option!(
        extension.to_str(),
        "failed to convert extension {:?} to string",
        extension
    )?;

    let extension = extension.to_lowercase();

    ris_log::debug!("import extension {:?}", extension);

    Ok(())
}
