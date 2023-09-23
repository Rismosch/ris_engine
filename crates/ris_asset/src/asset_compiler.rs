use std::collections::HashMap;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;

use ris_util::ris_error::RisError;

use crate::loader::ris_loader;
use crate::loader::ris_loader::RisLoaderError;

// "ris_assets\0\0\0\0\0\0"
pub const MAGIC: [u8; 16] = [
    0x72, 0x69, 0x73, 0x5F, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

pub const DEFAULT_ASSET_DIRECTORY: &str = "assets";
pub const DEFAULT_COMPILED_FILE: &str = "compiled.ris_assets";
pub const DEFAULT_DECOMPILED_DIRECTORY: &str = "decompiled_assets";

/// compiles a directory from a .ris_asset file
/// - `source`: the directory to be compiled
/// - `target`: the path to the final compiled file. if this file exists already, it will be
/// overwritten
pub fn compile(source: &str, target: &str) -> Result<(), RisError> {
    let mut assets = Vec::new();
    let mut assets_lookup = HashMap::new();
    let mut directories = std::collections::VecDeque::new();
    let source_path = PathBuf::from(source);
    directories.push_back(source_path.clone());

    // find all asset files
    while let Some(current) = directories.pop_front() {
        let entries = ris_util::unroll!(
            std::fs::read_dir(&current),
            "failed to read directory \"{:?}\"",
            current
        )?;

        for entry in entries {
            let entry = ris_util::unroll!(entry, "failed to read entry")?;
            let metadata = ris_util::unroll!(entry.metadata(), "failed to read metadata")?;

            let entry_path = entry.path();
            if metadata.is_file() {
                assets_lookup.insert(entry_path.clone(), assets.len());
                assets.push(entry_path);
            } else if metadata.is_dir() {
                directories.push_back(entry_path);
            } else {
                return ris_util::result_err!(
                    "entry \"{:?}\" is neither a file, nor a directory",
                    entry_path
                );
            }
        }
    }

    ris_log::trace!("found {} assets:", assets.len());
    for (i, file) in assets.iter().enumerate() {
        ris_log::trace!("{}: {:?}", i, file);
    }

    // create the target file
    let target_path = Path::new(target);
    if target_path.exists() {
        ris_util::unroll!(
            std::fs::remove_file(target_path),
            "failed to remove \"{:?}\"",
            target_path
        )?;
    }

    let mut target_file = ris_util::unroll!(
        File::create(target_path),
        "failed to create \"{:?}\"",
        target_path
    )?;

    // write magic
    crate::util::seek(&mut target_file, SeekFrom::Start(0))?;
    crate::util::write(&mut target_file, &MAGIC)?;

    // write references (none, but still doing for consistency between "ris_" files
    crate::util::write(&mut target_file, &[1])?;
    crate::util::write(&mut target_file, &u32::to_le_bytes(0))?;

    // write addr of original paths
    let addr_original_paths = crate::util::seek(&mut target_file, SeekFrom::Current(0))?;
    crate::util::write(&mut target_file, &[0; crate::ADDR_SIZE])?; // placeholder

    // write lookup
    let asset_count = assets.len() as u64;
    let asset_count_bytes = u64::to_le_bytes(asset_count);
    crate::util::write(&mut target_file, &asset_count_bytes)?;

    let addr_lookup = crate::util::seek(&mut target_file, SeekFrom::Current(0))?;
    let lookup_size = crate::ADDR_SIZE * asset_count as usize;
    crate::util::write(&mut target_file, &vec![0; lookup_size])?; // placeholder

    // compile assets
    for (i, asset) in assets.iter().enumerate() {
        let mut file =
            ris_util::unroll!(File::open(asset), "failed to open asset \"{:?}\"", &asset)?;

        let file_size = crate::util::seek(&mut file, SeekFrom::End(0))? as usize;
        let mut file_content = vec![0; file_size];
        crate::util::seek(&mut file, SeekFrom::Start(0))?;
        crate::util::read(&mut file, &mut file_content)?;

        // change directory ids to compiled ids
        let mut file_stream = std::io::Cursor::new(&file_content);
        let modified_file_content = match ris_loader::load(&mut file_stream) {
            Ok(ris_asset) => {
                ris_log::trace!("{:?}", ris_asset);

                let mut asset_bytes = Cursor::new(Vec::new());
                crate::util::write(&mut asset_bytes, &ris_asset.magic)?;
                crate::util::write(&mut asset_bytes, &[1])?;

                let reference_count = ris_asset.references.len() as u32;
                let reference_count_bytes = u32::to_le_bytes(reference_count);
                crate::util::write(&mut asset_bytes, &reference_count_bytes)?;

                for reference in ris_asset.references {
                    match reference {
                        crate::AssetId::Compiled(_id) => {
                            return ris_util::result_err!(
                                "attempted to compile an already compiled asset"
                            );
                        }
                        crate::AssetId::Directory(id) => {
                            let mut id_path = PathBuf::from(&source_path);
                            id_path.push(id);
                            let lookup_value = assets_lookup.get(&id_path);

                            let compiled_id = ris_util::unroll_option!(
                                lookup_value,
                                "asset references another asset that doesn't exist: {:?}",
                                id_path
                            )?;

                            let id_to_write = *compiled_id as u32;
                            let id_bytes = u32::to_le_bytes(id_to_write);
                            crate::util::write(&mut asset_bytes, &id_bytes)?;
                        }
                    }
                }

                crate::util::write(&mut asset_bytes, &ris_asset.content)?;

                asset_bytes.into_inner()
            }
            Err(error) => match error {
                RisLoaderError::NotRisAsset => file_content,
                RisLoaderError::IOError(error) => return Err(error),
            },
        };

        // write to compiled file
        let addr_asset = crate::util::seek(&mut target_file, SeekFrom::Current(0))?;
        crate::util::write(&mut target_file, &modified_file_content)?;
        let addr_current = crate::util::seek(&mut target_file, SeekFrom::Current(0))?;

        let addr_lookup_entry = addr_lookup + (crate::ADDR_SIZE * i) as u64;
        crate::util::seek(&mut target_file, SeekFrom::Start(addr_lookup_entry))?;
        let addr_asset_bytes = u64::to_le_bytes(addr_asset);
        crate::util::write(&mut target_file, &addr_asset_bytes)?;

        crate::util::seek(&mut target_file, SeekFrom::Start(addr_current))?;
    }

    // now that all assets are compiled, we can append the original paths
    // our current addr is the addr of the original paths
    let addr_current = crate::util::seek(&mut target_file, SeekFrom::Current(0))?;
    crate::util::seek(&mut target_file, SeekFrom::Start(addr_original_paths))?;
    let addr_current_bytes = u64::to_le_bytes(addr_current);
    crate::util::write(&mut target_file, &addr_current_bytes)?;
    crate::util::seek(&mut target_file, SeekFrom::Start(addr_current))?;

    // compile original paths
    for (i, asset) in assets.iter().enumerate() {
        let mut original_path = String::from(ris_util::unroll_option!(
            asset.to_str(),
            "asset path is not valid UTF8"
        )?);
        original_path.replace_range(0..source.len(), "");
        let mut original_path = original_path.replace('\\', "/");
        if original_path.starts_with('/') {
            original_path.remove(0);
        }

        ris_log::debug!("saving original path {:?}", original_path);
        let relative_path_bytes = original_path.as_bytes();
        crate::util::write(&mut target_file, relative_path_bytes)?;

        if i != assets.len() - 1 {
            crate::util::write(&mut target_file, &[0])?; // seperate paths with \0
        }
    }

    Ok(())
}

/// decompiles a .ris_asset file to a directory.
/// - `source`: the path to the compiled file
/// - `target`: the path to the final directory. if this directory exists already, it will be
/// cleared
pub fn decompile(source: &str, target: &str) -> Result<(), RisError> {
    // preparations
    let target = Path::new(target);
    if target.exists() {
        ris_util::unroll!(
            std::fs::remove_dir_all(target),
            "failed to delete target {:?}",
            target
        )?;
    }
    ris_util::unroll!(
        std::fs::create_dir_all(target),
        "failed to create target {:?}",
        target
    )?;

    let mut source = ris_util::unroll!(
        File::open(source),
        "failed to open source file {:?}",
        source
    )?;

    // load ris_asset
    let ris_asset = match ris_loader::load_magic_and_references(&mut source) {
        Ok(ris_asset) => ris_asset,
        Err(RisLoaderError::NotRisAsset) => return ris_util::result_err!("attempted to decompile a non ris_asset"),
        Err(RisLoaderError::IOError(e)) => return Err(e),
    };

    if !crate::util::bytes_equal(&MAGIC, &ris_asset.magic) {
        return ris_util::result_err!("expected magic value {:?} but was {:?}", MAGIC, ris_asset.magic);
    }

    let reference_len = ris_asset.references.len();
    if reference_len > 0 {
        ris_log::warning!("ris_assets referenced {} other assets. these will be ignored, as the assets stored in this file do not reference them at any point", reference_len);
    }

    //let mut content = std::io::Cursor::new(ris_asset.content);

    // get original paths addr
    let mut addr_original_paths_bytes = [0u8; crate::ADDR_SIZE];
    crate::util::read(&mut content, &mut addr_original_paths_bytes)?;
    let addr_original_paths = u64::from_le_bytes(addr_original_paths_bytes);

    // read lookup
    let mut lookup_len_bytes = [0u8; crate::ADDR_SIZE];
    crate::util::read(&mut content, &mut lookup_len_bytes)?;
    let lookup_len = u64::from_le_bytes(lookup_len_bytes);

    let mut lookup = Vec::with_capacity(lookup_len as usize);
    for _ in 0..lookup_len {
        let mut addr_asset_bytes = [0u8; crate::ADDR_SIZE];
        crate::util::read(&mut content, &mut addr_asset_bytes)?;
        let addr_asset = u64::from_le_bytes(addr_asset_bytes);
        lookup.push(addr_asset);
    }

    // read original paths
    let file_end = crate::util::seek(&mut content, SeekFrom::End(0))?;
    crate::util::seek(&mut content, SeekFrom::Start(addr_original_paths))?;
    ris_log::fatal!("schmoi {} {}", file_end, addr_original_paths);
    let orig_paths_len = file_end - addr_original_paths;

    let mut original_paths = Vec::with_capacity(orig_paths_len as usize);
    let read_bytes = ris_util::unroll!(
        content.read_to_end(&mut original_paths),
        "failed to read to the end"
    )?;
    if read_bytes != orig_paths_len as usize {
        return ris_util::result_err!(
            "expected to read {} bytes but actually read{}",
            orig_paths_len,
            read_bytes
        );
    }

    let original_paths_string = ris_util::unroll!(
        String::from_utf8(original_paths),
        "could not convert original paths to a string"
    )?;
    let mut original_paths: Vec<String> = original_paths_string
        .split('\0')
        .map(String::from)
        .collect();
    let placeholder_len = lookup_len as i64 - original_paths.len() as i64;
    if placeholder_len > 0 {
        for i in 0..placeholder_len {
            original_paths.push(format!("unnamed_asset_{}", i));
        }
    }

    // read assets
    for i in 0..lookup.len() {
        let addr_asset = lookup[i];
        let original_path = &original_paths[i];

        let addr_next_asset = if i == lookup.len() - 1 {
            addr_original_paths
        } else {
            lookup[i + 1]
        };
        let asset_len = addr_next_asset - addr_asset;

        crate::util::seek(&mut content, SeekFrom::Start(addr_asset))?;
        let mut file_bytes = vec![0u8; asset_len as usize];
        crate::util::read(&mut content, &mut file_bytes)?;

        // TODO: reassign ids

        let mut asset_path = PathBuf::new();
        asset_path.push(target);
        asset_path.push(original_path);
        let parent = ris_util::unroll_option!(
            asset_path.parent(),
            "asset does not have a parent directory"
        )?;
        ris_util::unroll!(
            std::fs::create_dir_all(parent),
            "failed to create asset parent \"{:?}\"",
            parent
        )?;

        let mut asset_file = ris_util::unroll!(
            File::create(asset_path.clone()),
            "failed to create asset \"{:?}\"",
            asset_path.clone()
        )?;
        crate::util::write(&mut asset_file, &file_bytes)?;
    }

    Ok(())
}
