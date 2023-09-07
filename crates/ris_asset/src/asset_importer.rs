use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use ris_util::ris_error::RisError;

pub enum ImporterKind{
    GLSL,
    DeduceFromFileName,
}

pub fn import(source: &str, target: &str, importer: ImporterKind) -> Result<(), RisError> {
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

    let file = ris_util::unroll!(File::open(source), "failed to open file to import: {:?}", &source)?;

    let (bytes, extension) = match extension.as_str() {
        "glsl" => {
            let bytes = crate::importer::glsl_importer::import(info)?;
            let extension = crate::importer::glsl_importer::OUT_EXT;
            (bytes, extension)
        },
        _ => return ris_util::result_err!("unsupported extension \"{}\"", extension),
    };

    Ok(())
}

