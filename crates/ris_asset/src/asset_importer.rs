use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use ris_util::ris_error::RisError;

use crate::importer::*;

pub enum ImporterKind{
    Unkown(String),
    GLSL,
}

pub struct SpecificImporterInfo {
    pub source_file_path: PathBuf,
    pub target_file_path: PathBuf,
    pub importer: ImporterKind,
}

pub struct DeduceImporterInfo {
    pub source_file_path: PathBuf,
    pub target_directory: PathBuf,
}

pub enum ImporterInfo {
    Specific(SpecificImporterInfo),
    DeduceFromFileName(DeduceImporterInfo),
}

pub fn import(info: ImporterInfo) -> Result<(), RisError> {
    let (source_path, target_path, importer) = match info {
        ImporterInfo::Specific(info) => {
            (
                info.source_file_path,
                info.target_file_path,
                info.importer,
            )
        },
        ImporterInfo::DeduceFromFileName(info) => {
            let source_path = info.source_file_path;
            let target_directory = info.target_directory;

            let source_extension = ris_util::unroll_option!(
                source_path.extension(),
                "failed to find extension from {:?}",
                source_path
            )?;
            let source_extension = ris_util::unroll_option!(
                source_extension.to_str(),
                "failed to convert extension {:?} to string",
                source_extension
            )?;
            let source_extension = source_extension.to_lowercase();
            let (importer, target_extension) = match source_extension.as_str() {
                glsl_importer::IN_EXT => (ImporterKind::GLSL, glsl_importer::OUT_EXT),
                _ => (ImporterKind::Unkown(source_extension), ""),
            };

            return ris_util::result_err!("not implemented yet");
        },
    };

    Ok(())

    //let importer_kind = match importer {
    //    kind => kind,
    //    ImporterKind::DeduceFromFileName => {
    //        let extension = ris_util::unroll_option!(
    //            source.extension(),
    //            "failed to find extension from source {:?}",
    //            source
    //        )?;
    //        let extension = ris_util::unroll_option!(
    //            extension.to_str(),
    //            "failed to convert extension {:?} to string",
    //            extension
    //        )?;
    //        let extension = extension.to_lowercase();

    //        match extension.as_str() {
    //            "glsl" => ImporterKind::GLSL,
    //            _ => return ris_util::result_err!("unsupported extension \"{}\"", extension),
    //        }
    //    },
    //};

    //let file = ris_util::unroll!(File::open(source), "failed to open file to import: {:?}", &source)?;

    //match importer_kind {
    //    ImporterKind::GLSL => {
    //        (1,2)
    //    },
    //    ImporterKind::DeduceFromFileName => unreachable!(),
    //}

    //Ok(())
}

