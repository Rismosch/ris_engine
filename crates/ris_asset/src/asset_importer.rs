use std::fs::File;
use std::path::PathBuf;

use ris_util::ris_error::RisError;

use crate::importer::*;

pub enum ImporterKind {
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
            (info.source_file_path, info.target_file_path, info.importer)
        }
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

            let source_stem = ris_util::unroll_option!(
                source_path.file_stem(),
                "failed to find file stem from {:?}",
                source_path
            )?;
            let source_stem = ris_util::unroll_option!(
                source_stem.to_str(),
                "failed to convert stem {:?} to string",
                source_stem,
            )?;
            let source_stem = String::from(source_stem);

            let (importer, target_extension) = match source_extension.as_str() {
                glsl_importer::IN_EXT => (ImporterKind::GLSL, glsl_importer::OUT_EXT),
                // insert new inporter here...
                _ => {
                    return ris_util::result_err!(
                        "failed to deduce importer. unkown extension: {}",
                        source_extension
                    )
                }
            };

            let mut target_path = PathBuf::new();
            target_path.push(target_directory);
            target_path.push(format!("{source_stem}.{target_extension}"));

            (source_path, target_path, importer)
        }
    };

    let parent = target_path.parent();
    if let Some(parent) = parent {
        if !parent.exists() {
            ris_util::unroll!(
                std::fs::create_dir_all(parent),
                "failed to create target directory {:?}",
                parent
            )?;
        }
    }

    if target_path.exists() {
        ris_util::unroll!(
            std::fs::remove_file(&target_path),
            "failed to delete target file {:?}",
            target_path,
        )?;
    }

    let mut source_file = ris_util::unroll!(
        File::open(&source_path),
        "failed to open file {:?}",
        source_path,
    )?;

    let mut target_file = ris_util::unroll!(
        File::create(&target_path),
        "failed to create target file {:?}",
        target_path,
    )?;

    match importer {
        ImporterKind::GLSL => glsl_importer::import(&mut source_file, &mut target_file),
        // insert more importers here...
    }
}
