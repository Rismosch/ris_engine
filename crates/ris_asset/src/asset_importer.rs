use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use ris_error::RisResult;

use crate::importer::*;

pub const DEFAULT_SOURCE_DIRECTORY: &str = "assets/__raw";
pub const DEFAULT_TARGET_DIRECTORY: &str = "assets/__imported_raw";

pub enum ImporterKind {
    GLSL,
    PNG,
}

pub struct SpecificImporterInfo {
    pub source_file_path: PathBuf,
    pub target_file_paths: Vec<PathBuf>,
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

pub fn import(info: ImporterInfo) -> RisResult<()> {
    let (source_path, target_paths, importer) = match info {
        ImporterInfo::Specific(info) => {
            (info.source_file_path, info.target_file_paths, info.importer)
        }
        ImporterInfo::DeduceFromFileName(info) => {
            let source_path = info.source_file_path;
            let target_directory = info.target_directory;

            let source_extension = ris_error::unroll_option!(
                source_path.extension(),
                "failed to find extension from {:?}",
                source_path
            )?;
            let source_extension = ris_error::unroll_option!(
                source_extension.to_str(),
                "failed to convert extension {:?} to string",
                source_extension
            )?;
            let source_extension = source_extension.to_lowercase();

            let (importer, target_extensions) = match source_extension.as_str() {
                glsl_to_spirv_importer::IN_EXT => {
                    (ImporterKind::GLSL, glsl_to_spirv_importer::OUT_EXT)
                }
                png_to_qoi_importer::IN_EXT => (ImporterKind::PNG, png_to_qoi_importer::OUT_EXT),
                // insert new inporter here...
                _ => {
                    return ris_error::new_result!(
                        "failed to deduce importer. unkown extension: {}",
                        source_extension
                    )
                }
            };

            let source_stem = ris_error::unroll_option!(
                source_path.file_stem(),
                "failed to find file stem from {:?}",
                source_path
            )?;
            let source_stem = ris_error::unroll_option!(
                source_stem.to_str(),
                "failed to convert stem {:?} to string",
                source_stem,
            )?;
            let source_stem = String::from(source_stem);

            let mut target_paths = Vec::new();

            for target_extension in target_extensions {
                let mut target_path = PathBuf::new();
                target_path.push(target_directory.clone());
                target_path.push(format!("{source_stem}.{target_extension}"));

                target_paths.push(target_path);
            }

            (source_path, target_paths, importer)
        }
    };

    //let source_path = ris_error::unroll_option!(
    //    source_path.to_str(),
    //    "not a valid utf8",
    //)?;

    //for target_path in target_paths {
    //    let parent = target_path.parent();
    //    if let Some(parent) = parent {
    //        if !parent.exists() {
    //            ris_error::unroll!(
    //                std::fs::create_dir_all(parent),
    //                "failed to create target directory {:?}",
    //                parent
    //            )?;
    //        }
    //    }

    //    if target_path.exists() {
    //        ris_error::unroll!(
    //            std::fs::remove_file(&target_path),
    //            "failed to delete target file {:?}",
    //            target_path,
    //        )?;
    //    }

    //    let target_file = ris_error::unroll!(
    //        File::create(&target_path),
    //        "failed to create target file {:?}",
    //        target_path,
    //    )?;

    //    target_files.push(target_file);
    //}

    match importer {
        ImporterKind::GLSL => glsl_to_spirv_importer::import(source_path, target_paths),
        ImporterKind::PNG => png_to_qoi_importer::import(source_path, target_paths),
        // insert more importers here...
    }
}

pub fn import_all(source_directory: &str, target_directory: &str) -> RisResult<()> {
    let mut directories = std::collections::VecDeque::new();
    let source_path = PathBuf::from(source_directory);
    directories.push_back(source_path);

    let target_directory_path = PathBuf::from(target_directory);
    if target_directory_path.exists() {
        ris_error::unroll!(
            std::fs::remove_dir_all(target_directory_path),
            "failed to delete target directory"
        )?;
    }

    while let Some(current) = directories.pop_front() {
        let entries = ris_error::unroll!(
            std::fs::read_dir(&current),
            "failed to read directory \"{:?}\"",
            current,
        )?;

        for entry in entries {
            let entry = ris_error::unroll!(entry, "failed to read entry")?;
            let metadata = ris_error::unroll!(entry.metadata(), "failed to read metadata")?;
            let entry_path = entry.path();

            if metadata.is_file() {
                let mut source_directory = source_directory.replace('\\', "/");
                let mut target_directory = target_directory.replace('\\', "/");

                if !source_directory.ends_with('/') {
                    source_directory.push('/');
                }

                if !target_directory.ends_with('/') {
                    target_directory.push('/');
                }

                let source_path = ris_error::unroll_option!(
                    entry_path.to_str(),
                    "failed to convert path to string"
                )?;
                let mut target_path_part = source_path.replace('\\', "/");
                target_path_part.replace_range(0..source_directory.len(), "");

                let mut target_path = PathBuf::new();
                target_path.push(target_directory.clone());
                target_path.push(&target_path_part);
                let target_path = PathBuf::from(target_path.parent().unwrap());

                ris_log::debug!("import {:?} to {:?}", entry_path, target_path);

                let info = DeduceImporterInfo {
                    source_file_path: entry_path,
                    target_directory: target_path,
                };
                let importer_info = ImporterInfo::DeduceFromFileName(info);
                import(importer_info)?;
            } else if metadata.is_dir() {
                directories.push_back(entry_path);
            } else {
                return ris_error::new_result!(
                    "entry {:?} is neither a file nor a directory",
                    entry_path,
                );
            }
        }
    }
    Ok(())
}

pub fn create_file(file_path: &Path) -> RisResult<File> {
    let parent = file_path.parent();
    if let Some(parent) = parent {
        if !parent.exists() {
            ris_error::unroll!(
                std::fs::create_dir_all(parent),
                "failed to create target directory {:?}",
                parent
            )?;
        }
    }

    if file_path.exists() {
        ris_error::unroll!(
            std::fs::remove_file(file_path),
            "failed to delete target file {:?}",
            file_path,
        )?;
    }

    ris_error::unroll!(
        File::create(file_path),
        "failed to create target file {:?}",
        file_path,
    )
}
