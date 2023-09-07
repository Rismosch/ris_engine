use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::path::Path;

use ris_util::ris_error::RisError;

pub struct ImportInfo{
    stream: File,
    filepath: Path,
}

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

    let file = ris_util::unroll!(File::open(source), "failed to open file to import: {:?}", &source)?;

    let (result_bytes, extension) = match extension.as_str() {
        "glsl" => {
            crate::importer::glsl_importer::import(file);

            (1, 2)
        },
        _ => return ris_util::result_err!("unsupported extension \"{}\"", extension),
    };

    Ok(())
}

