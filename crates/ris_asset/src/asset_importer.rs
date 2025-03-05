use std::collections::VecDeque;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use ris_data::ris_yaml::RisYaml;
use ris_error::Extensions;
use ris_error::RisResult;

use crate::importer::*;

pub const DEFAULT_SOURCE_DIRECTORY: &str = "assets/source_files";
pub const DEFAULT_IMPORT_DIRECTORY: &str = "assets/imported";
pub const DEFAULT_IN_USE_DIRECTORY: &str = "assets/in_use";
pub const META_EXTENSION: &str = "ris_meta";
pub const META_COPY_TO: &str = "copy_to";

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
                let extension = entry_path.extension()
                    .into_ris_error()?
                    .to_str()
                    .into_ris_error()?
                    .to_lowercase();
                
                if extension.trim() != META_EXTENSION {
                    ris_log::debug!("cleaning \"{}\"...", entry_path.display());
                    std::fs::remove_file(entry_path)?;
                    cleaned += 1;
                }
            }
            else if metadata.is_dir() {
                directories.push_back(entry_path);
            } else {
                return ris_error::new_result!(
                    "entry \"{}\" is neither a file nor a directory",
                    entry_path.display(),
                );
            }
        }

        ris_log::debug!("deleted {} files", cleaned);

    };

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
    let import_path = PathBuf::from(import_directory);
    let in_use_path = PathBuf::from(in_use_directory);
    directories.push_back(import_path.clone());

    while let Some(current) = directories.pop_front() {
        let entries = std::fs::read_dir(&current)?;

        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let entry_path = entry.path();

            if metadata.is_file() {
                let Some(extension) = entry_path.extension() else {
                    continue;
                };

                let Some(extension_str) = extension.to_str() else {
                    continue;
                };

                if extension_str != META_EXTENSION {
                    continue;
                }

                let entry_parent = entry_path.parent().into_ris_error()?;
                let entry_stem = entry_path.file_stem().into_ris_error()?;
                let copy_source = PathBuf::from(entry_parent).join(entry_stem);

                let meta_content = std::fs::read_to_string(&entry_path)?;
                let yaml = RisYaml::deserialize(meta_content)?;
                let copy_to_value = yaml.get_value(META_COPY_TO).into_ris_error()?;
                let copy_target = in_use_path.join(copy_to_value);

                ris_log::trace!(
                    "copying \"{}\" to \"{}\"...",
                    copy_source.display(),
                    copy_target.display(),
                );

                //let copy_target_parent = copy_target.parent().into_ris_error()?;
                //std::fs::create_dir_all(copy_target_parent)?;

                let mut to_copy = VecDeque::new();
                to_copy.push_back((copy_source.clone(), copy_target.clone()));

                while let Some((copy_source, copy_target)) = to_copy.pop_front() {
                    if copy_source.ends_with(META_EXTENSION) {
                        continue;
                    }

                    if copy_source.is_file() {
                        let copy_target_parent = copy_target.parent().into_ris_error()?;
                        std::fs::create_dir_all(copy_target_parent)?;
                        std::fs::copy(&copy_source, &copy_target)?;
                    } else if copy_source.is_dir() {
                        for entry in std::fs::read_dir(&copy_source)? {
                            let entry = entry?;
                            let entry_path = entry.path();
                            let entry_name = entry_path.file_name().into_ris_error()?;

                            let new_copy_target = copy_target.join(entry_name);
                            to_copy.push_back((entry_path, new_copy_target));
                        }
                    } else {
                        return ris_error::new_result!(
                            "\"{}\" is neither a file nor a dir",
                            copy_source.display()
                        );
                    }
                }

                ris_log::debug!(
                    "copied \"{}\" to \"{}\"!",
                    copy_source.display(),
                    copy_target.display(),
                );
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

    Ok(())
}

pub fn create_file(file_path: &Path) -> RisResult<File> {
    let parent = file_path.parent();
    if let Some(parent) = parent {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    if file_path.exists() {
        std::fs::remove_file(file_path)?;
    }

    let file = File::create(file_path)?;
    Ok(file)
}

fn import(info: ImporterInfo, temp_directory: Option<&Path>) -> RisResult<()> {
    let (source_path, target_paths, importer) = match info {
        ImporterInfo::Specific(info) => {
            (info.source_file_path, info.target_file_paths, info.importer)
        }
        ImporterInfo::DeduceFromFileName(info) => {
            let source_path = info.source_file_path;
            let target_directory = info.target_directory;

            let source_extension = source_path.extension().into_ris_error()?;
            let source_extension = source_extension.to_str().into_ris_error()?;
            let source_extension = source_extension.to_lowercase();

            let (importer, target_extensions) = match source_extension.as_str() {
                glsl_to_spirv_importer::IN_EXT => {
                    (ImporterKind::GLSL, glsl_to_spirv_importer::OUT_EXT)
                }
                png_to_qoi_importer::IN_EXT => (ImporterKind::PNG, png_to_qoi_importer::OUT_EXT),
                // insert new inporter here...
                _ => {
                    ris_log::debug!(
                        "failed to deduce importer, unknown extension \"{}\"",
                        source_path.display(),
                    );
                    return Ok(());
                }
            };

            let source_stem = source_path.file_stem().into_ris_error()?;
            let source_stem = source_stem.to_str().into_ris_error()?;
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

    match importer {
        ImporterKind::GLSL => {
            glsl_to_spirv_importer::import(source_path, target_paths, temp_directory)
        }
        ImporterKind::PNG => png_to_qoi_importer::import(source_path, target_paths),
        // insert more importers here...
    }
}
