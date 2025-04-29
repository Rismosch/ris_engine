use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::path::PathBuf;

use ris_error::Extensions;
use ris_error::RisResult;

use crate::importer::*;

pub const DEFAULT_SOURCE_DIRECTORY: &str = "assets/source_files";
pub const DEFAULT_IMPORT_DIRECTORY: &str = "assets/imported";
pub const DEFAULT_IN_USE_DIRECTORY: &str = "assets/in_use";
pub const COPY_INSTRUCTIONS_PATH: &str = "assets/copy_instructions.ris_meta";
pub const COPY_INSTRUCTION_COMMENT: char = '#';
pub const COPY_INSTRUCTION_SEPARATOR: &str = ":=>";
pub const META_COPY_TO: &str = "copy_to";

pub enum ImporterKind {
    GLB,
    GLSL,
    PNG,
}

pub struct SpecificImporterInfo {
    pub source_file_path: PathBuf,
    pub target_directory: PathBuf,
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

pub fn clean(import_directory: &str) -> RisResult<()> {
    let mut directories = VecDeque::new();
    directories.push_back(PathBuf::from(import_directory));

    while let Some(current) = directories.pop_front() {
        let entries = std::fs::read_dir(&current)?;

        let mut cleaned = 0;
        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let entry_path = entry.path();

            if metadata.is_file() {
                ris_log::debug!("cleaning \"{}\"...", entry_path.display());
                std::fs::remove_file(entry_path)?;
                cleaned += 1;
            } else if metadata.is_dir() {
                directories.push_back(entry_path);
            } else {
                return ris_error::new_result!(
                    "entry \"{}\" is neither a file nor a directory",
                    entry_path.display(),
                );
            }
        }

        ris_log::debug!("deleted {} files", cleaned);
    }

    Ok(())
}

pub fn import_all(
    source_directory: &str,
    import_directory: &str,
    in_use_directory: &str,
    temp_directory: Option<&str>,
) -> RisResult<()> {
    let temp_directory = temp_directory.map(PathBuf::from);
    let mut directories = VecDeque::new();

    // import source files
    let source_path = PathBuf::from(source_directory);
    directories.push_back(source_path);

    while let Some(current) = directories.pop_front() {
        let entries = std::fs::read_dir(&current)?;

        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let entry_path = entry.path();

            if metadata.is_file() {
                let mut source_directory = source_directory.replace('\\', "/");
                let mut target_directory = import_directory.replace('\\', "/");

                if !source_directory.ends_with('/') {
                    source_directory.push('/');
                }

                if !target_directory.ends_with('/') {
                    target_directory.push('/');
                }

                let source_path = entry_path.to_str().into_ris_error()?;
                let mut target_path_part = source_path.replace('\\', "/");
                target_path_part.replace_range(0..source_directory.len(), "");

                let mut target_path = PathBuf::new();
                target_path.push(target_directory.clone());
                target_path.push(&target_path_part);
                let target_path = PathBuf::from(target_path.parent().unwrap());

                ris_log::debug!(
                    "import \"{}\" to \"{}\"",
                    entry_path.display(),
                    target_path.display(),
                );

                let info = DeduceImporterInfo {
                    source_file_path: entry_path,
                    target_directory: target_path,
                };
                let importer_info = ImporterInfo::DeduceFromFileName(info);
                let temp_directory = temp_directory.as_deref();
                import(importer_info, temp_directory)?;
            } else if metadata.is_dir() {
                directories.push_back(entry_path);
            } else {
                return ris_error::new_result!(
                    "entry \"{}\" is neither a file nor a directory",
                    entry_path.display(),
                );
            }
        }
    }

    // copy imported files
    ris_log::debug!("copying imported files...");
    let file = std::fs::File::open(COPY_INSTRUCTIONS_PATH)?;
    let reader = std::io::BufReader::new(file);
    for (line_number, line) in reader.lines().enumerate() {
        let line_number = line_number + 1;
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with(COPY_INSTRUCTION_COMMENT) {
            continue;
        }

        let splits = line
            .split(COPY_INSTRUCTION_SEPARATOR)
            .collect::<Vec<_>>();

        if splits.len() != 2 {
            let count = splits.len() - 1;
            return ris_error::new_result!(
                "expected to find 1 \"{}\" but found {}. line: {}",
                COPY_INSTRUCTION_SEPARATOR,
                count,
                line_number,
            );
        }

        let split0 = splits[0].trim();
        let split1 = splits[1].trim();

        if split0.is_empty() || split1.is_empty() {
            return ris_error::new_result!(
                "source and target may not be empty. line: {}",
                line_number,
            )
        }

        let source = PathBuf::from(import_directory).join(split0);
        let target = PathBuf::from(in_use_directory).join(split1);

        if !source.exists() {
            return ris_error::new_result!(
                "source \"{}\" does not exist. line: {}",
                source.display(),
                line_number,
            );
        }

        let mut to_copy = Vec::new();


        if source.is_file() {
            to_copy.push((source, target));
        } else if source.is_dir() {
            directories.push_back(PathBuf::from(&source));

            while let Some(current) = directories.pop_front() {
                let entries = std::fs::read_dir(&current)?;

                for entry in entries {
                    let entry = entry?;
                    let metadata = entry.metadata()?;
                    let entry_path = entry.path();

                    if metadata.is_file() {
                        let entry_path_without_root = entry_path.strip_prefix(&source)?;
                        let actual_target = target.join(entry_path_without_root);
                        to_copy.push((entry_path, actual_target));
                    } else if metadata.is_dir() {
                        directories.push_back(entry_path);
                    } else {
                        return ris_error::new_result!(
                            "entry \"{}\" is neither a file nor a directory",
                            entry_path.display(),
                        );
                    }
                }
            }
        } else {
            return ris_error::new_result!(
                "source \"{}\" is neither a file nor a directory. line: {}",
                source.display(),
                line_number,
            );
        }

        for (source, target) in to_copy {
            ris_log::debug!(
                "copying \"{}\" to \"{}\"...",
                source.display(),
                target.display(),
            );

            let target_parent = target.parent().into_ris_error()?;
            std::fs::create_dir_all(target_parent)?;
            std::fs::copy(source, target)?;
        }
    }

    Ok(())
}

pub fn create_file(
    source: impl AsRef<Path>,
    target_dir: impl AsRef<Path>,
    extension: impl AsRef<str>,
) -> RisResult<File> {
    let source = source.as_ref();
    let target_dir = target_dir.as_ref();
    let extension = extension.as_ref();

    let file_stem = source
        .file_stem()
        .into_ris_error()?
        .to_str()
        .into_ris_error()?;

    let target = target_dir.join(format!("{}.{}", file_stem, extension,));

    let parent = target.parent();
    if let Some(parent) = parent {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    if target.exists() {
        std::fs::remove_file(&target)?;
    }

    let file = File::create(target)?;
    Ok(file)
}

fn import(info: ImporterInfo, temp_directory: Option<&Path>) -> RisResult<()> {
    let (source, target, importer) = match info {
        ImporterInfo::Specific(info) => {
            (info.source_file_path, info.target_directory, info.importer)
        }
        ImporterInfo::DeduceFromFileName(info) => {
            let source_path = info.source_file_path;
            let target_directory = info.target_directory;

            let source_extension = source_path.extension().into_ris_error()?;
            let source_extension = source_extension.to_str().into_ris_error()?;
            let source_extension = source_extension.to_lowercase();

            let importer = match source_extension.as_str() {
                glb_importer::IN_EXT_GLB => ImporterKind::GLB,
                glsl_to_spirv_importer::IN_EXT_GLSL => ImporterKind::GLSL,
                png_to_qoi_importer::IN_EXT_PNG => ImporterKind::PNG,
                // insert new importer here...
                _ => {
                    ris_log::debug!(
                        "failed to deduce importer, unknown extension \"{}\"",
                        source_path.display(),
                    );
                    return Ok(());
                }
            };

            (source_path, target_directory, importer)
        }
    };

    match importer {
        ImporterKind::GLB => glb_importer::import(source, target),
        ImporterKind::GLSL => glsl_to_spirv_importer::import(source, target, temp_directory),
        ImporterKind::PNG => png_to_qoi_importer::import(source, target),
        // insert new importers here...
    }
}
