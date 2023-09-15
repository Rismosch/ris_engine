use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;

use ris_util::ris_error::RisError;

// "ris_assets\0\0\0\0\0\0"
pub const MAGIC: [u8; 16] = [
    0x72, 0x69, 0x73, 0x5F, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
pub const ADDR_SIZE: usize = std::mem::size_of::<u64>();

pub const DEFAULT_ASSET_DIRECTORY: &str = "assets";
pub const DEFAULT_COMPILED_FILE: &str = "compiled.ris_assets";
pub const DEFAULT_DECOMPILED_DIRECTORY: &str = "decompiled";

/// compiles a directory from a .ris_asset file
/// - `source`: the directory to be compiled
/// - `target`: the path to the final compiled file. if this file exists already, it will be
/// overwritten
pub fn compile(source: &str, target: &str) -> Result<(), RisError> {
    let mut assets = Vec::new();
    let mut assets_lookup = HashMap::new();
    let mut directories = std::collections::VecDeque::new();
    let source_path = PathBuf::from(source);
    directories.push_back(source_path);

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

    // TODO: reassign ids by reading the files and using the lookup

    let target_path = Path::new(target);
    if target_path.exists() {
        ris_util::unroll!(
            std::fs::remove_file(target_path),
            "failed to remove \"{:?}\"",
            target_path
        )?;
    }

    let mut target = ris_util::unroll!(
        File::create(target_path),
        "failed to create \"{:?}\"",
        target_path
    )?;

    crate::util::seek(&mut target, SeekFrom::Start(0))?;

    crate::util::write(&mut target, &MAGIC)?;

    let addr_original_paths = crate::util::seek(&mut target, SeekFrom::Current(0))?;
    crate::util::write(&mut target, &[0; ADDR_SIZE])?; // placeholder

    let asset_count = assets.len() as u64;
    let asset_count_bytes = u64::to_le_bytes(asset_count);
    crate::util::write(&mut target, &asset_count_bytes)?;

    let addr_lookup = crate::util::seek(&mut target, SeekFrom::Current(0))?;
    let lookup_size = ADDR_SIZE * asset_count as usize;
    crate::util::write(&mut target, &vec![0; lookup_size])?; // placeholder

    for (i, asset) in assets.iter().enumerate() {
        let mut file =
            ris_util::unroll!(File::open(asset), "failed to open asset \"{:?}\"", &asset)?;

        let file_size = crate::util::seek(&mut file, SeekFrom::End(0))? as usize;
        let mut file_content = vec![0; file_size];
        crate::util::seek(&mut file, SeekFrom::Start(0))?;
        crate::util::read(&mut file, &mut file_content)?;

        let addr_asset = crate::util::seek(&mut target, SeekFrom::Current(0))?;
        crate::util::write(&mut target, &file_content)?;
        let addr_current = crate::util::seek(&mut target, SeekFrom::Current(0))?;

        let addr_lookup_entry = addr_lookup + (ADDR_SIZE * i) as u64;
        crate::util::seek(&mut target, SeekFrom::Start(addr_lookup_entry))?;
        let addr_asset_bytes = u64::to_le_bytes(addr_asset);
        crate::util::write(&mut target, &addr_asset_bytes)?;

        crate::util::seek(&mut target, SeekFrom::Start(addr_current))?;
    }

    let addr_current = crate::util::seek(&mut target, SeekFrom::Current(0))?;
    crate::util::seek(&mut target, SeekFrom::Start(addr_original_paths))?;
    let addr_current_bytes = u64::to_le_bytes(addr_current);
    crate::util::write(&mut target, &addr_current_bytes)?;
    crate::util::seek(&mut target, SeekFrom::Start(addr_current))?;

    for (i, asset) in assets.iter().enumerate() {
        let mut original_path = String::from(ris_util::unroll_option!(
            asset.to_str(),
            "asset path is not valid UTF8"
        )?);
        original_path.replace_range(0..source.len(), "");
        let original_path = original_path.replace('\\', "/");
        let relative_path_bytes = original_path.as_bytes();
        crate::util::write(&mut target, relative_path_bytes)?;

        if i != assets.len() - 1 {
            crate::util::write(&mut target, &[0])?; // seperate paths with \0
        }
    }

    Ok(())
}

/// decompiles a .ris_asset file to a directory.
/// - `source`: the path to the compiled file
/// - `target`: the path to the final directory. if this directory exists already, it will be
/// cleared
pub fn decompile(source: &str, target: &str) -> Result<(), RisError> {
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

    crate::util::seek(&mut source, SeekFrom::Start(MAGIC.len() as u64))?;

    let mut addr_original_paths_bytes = [0u8; ADDR_SIZE];
    crate::util::read(&mut source, &mut addr_original_paths_bytes)?;
    let addr_original_paths = u64::from_le_bytes(addr_original_paths_bytes);

    let mut lookup_len_bytes = [0u8; ADDR_SIZE];
    crate::util::read(&mut source, &mut lookup_len_bytes)?;
    let lookup_len = u64::from_le_bytes(lookup_len_bytes);

    let mut lookup = Vec::with_capacity(lookup_len as usize);

    for _ in 0..lookup_len {
        let mut addr_asset_bytes = [0u8; ADDR_SIZE];
        crate::util::read(&mut source, &mut addr_asset_bytes)?;
        let addr_asset = u64::from_le_bytes(addr_asset_bytes);
        lookup.push(addr_asset);
    }

    let file_end = crate::util::seek(&mut source, SeekFrom::End(0))?;
    crate::util::seek(&mut source, SeekFrom::Start(addr_original_paths))?;
    let orig_paths_len = file_end - addr_original_paths;

    let mut original_paths = Vec::with_capacity(orig_paths_len as usize);
    let read_bytes = ris_util::unroll!(
        source.read_to_end(&mut original_paths),
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

    for i in 0..lookup.len() {
        let addr_asset = lookup[i];
        let original_path = &original_paths[i];

        let addr_next_asset = if i == lookup.len() - 1 {
            addr_original_paths
        } else {
            lookup[i + 1]
        };
        let asset_len = addr_next_asset - addr_asset;

        crate::util::seek(&mut source, SeekFrom::Start(addr_asset))?;
        let mut file_bytes = vec![0u8; asset_len as usize];
        crate::util::read(&mut source, &mut file_bytes)?;

        // TODO: reassign ids

        let mut asset_path = PathBuf::new();
        asset_path.push(target);
        asset_path.push(original_path);
        let parent = ris_util::unroll_option!(
            asset_path.parent(),
            "asset does not have a parent directory"
        )?;
        ris_util::unroll!(
            std::fs::create_dir_all(parent), this throws for some reason
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
