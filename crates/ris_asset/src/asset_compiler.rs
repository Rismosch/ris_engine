use std::collections::HashMap;
use std::fs::File;
use std::io::Cursor;
use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;

use ris_data::asset_id::AssetId;
use ris_error::Extensions;
use ris_error::RisResult;
use ris_io::FatPtr;

use crate::RisHeader;

// # File Format
//
// encoding: little-endian
//
// - [u8; 16]: magic `ris_assets\0\0\0\0\0\0"`
// - FatPtr: p_original_asset_names
// - u32: asset_lookup_count
// - [u64; asset_lookup_count]: asset_lookup
// - [u8; ?]: assets
// - [u8; ?]: original names (utf8 encoded strings, seperated by `\0`)

pub const MAGIC: [u8; 16] = [
    0x72, 0x69, 0x73, 0x5F, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

pub const DEFAULT_ASSET_DIRECTORY: &str = "assets/in_use";
pub const DEFAULT_COMPILED_FILE: &str = "ris_assets";
pub const DEFAULT_DECOMPILED_DIRECTORY: &str = "decompiled_assets";

#[derive(Default, Debug, Clone, Copy)]
pub struct CompileOptions {
    pub include_original_paths: bool,
}

/// compiles a directory to a ris_asset file
/// - `source`: the directory to be compiled
/// - `target`: the path to the final compiled file. if this file exists already, it will be
/// overwritten
pub fn compile(source: &str, target: &str, options: CompileOptions) -> RisResult<()> {
    let mut assets = Vec::new();
    let mut asset_lookup_hashmap = HashMap::new();
    let mut directories = std::collections::VecDeque::new();
    let source_path = PathBuf::from(source);
    directories.push_back(source_path.clone());

    // find all asset files
    while let Some(current) = directories.pop_front() {
        let entries = std::fs::read_dir(&current)?;

        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let entry_path = entry.path();

            if metadata.is_file() {
                asset_lookup_hashmap.insert(entry_path.clone(), assets.len());
                assets.push(entry_path);
            } else if metadata.is_dir() {
                directories.push_back(entry_path);
            } else {
                return ris_error::new_result!(
                    "entry \"{}\" is neither a file, nor a directory",
                    entry_path.display(),
                );
            }
        }
    }

    ris_log::trace!("found {} assets:", assets.len());
    for (i, file) in assets.iter().enumerate() {
        ris_log::trace!("{}: \"{}\"", i, file.display());
    }

    // create the target file
    let target_path = Path::new(target);
    if target_path.exists() {
        std::fs::remove_file(target_path)?;
    }

    let mut target_file = File::create(target_path)?;
    let target_file = &mut target_file;

    // write magic
    ris_io::seek(target_file, SeekFrom::Start(0))?;
    ris_io::write(target_file, &MAGIC)?;

    // write ptr to original paths
    let addr_p_original_asset_names = ris_io::seek(target_file, SeekFrom::Current(0))?;
    ris_io::write_fat_ptr(target_file, FatPtr::null())?; // placeholder

    // write lookup
    ris_io::write_uint(target_file, assets.len())?;
    let addr_asset_lookup = ris_io::seek(target_file, SeekFrom::Current(0))?;
    let mut asset_lookup = vec![0; assets.len()];
    for asset_lookup_entry in asset_lookup.iter() {
        ris_io::write_u64(target_file, *asset_lookup_entry)?; // placeholder
    }

    // compile assets
    for (i, asset) in assets.iter().enumerate() {
        ris_log::info!(
            "compiling... {}/{} \"{}\"",
            i + 1,
            assets.len(),
            asset.display(),
        );

        let mut file = File::open(asset)?;

        let file_size = ris_io::seek(&mut file, SeekFrom::End(0))? as usize;
        let mut file_content = vec![0; file_size];
        ris_io::seek(&mut file, SeekFrom::Start(0))?;
        ris_io::read(&mut file, &mut file_content)?;

        let modified_file_content = match RisHeader::load(&file_content)? {
            // asset is not a ris_asset, return unmodified
            None => file_content,

            // asset is ris_asset, change directory id to compiled id
            Some(ris_header) => {
                let mut references = Vec::with_capacity(ris_header.references.len());
                for reference in &ris_header.references {
                    match reference {
                        AssetId::Index(id) => {
                            return ris_error::new_result!(
                                "attempted to compile an already compiled asset: {}",
                                id,
                            );
                        }
                        AssetId::Path(id) => {
                            let mut id_path = PathBuf::from(&source_path);
                            id_path.push(id);
                            let lookup_value = asset_lookup_hashmap.get(&id_path);

                            let compiled_id = *lookup_value.into_ris_error()?;
                            references.push(compiled_id);
                        }
                    }
                }

                let mut file_content = Cursor::new(file_content);
                let ris_asset_content = ris_io::read_at(&mut file_content, ris_header.p_content())?;

                let mut modified_file_content = Cursor::new(Vec::new());
                let stream = &mut modified_file_content;
                ris_io::write(stream, &ris_header.magic)?;
                ris_io::write_bool(stream, true)?;
                ris_io::write_uint(stream, references.len())?;
                for reference in references {
                    ris_io::write_uint(stream, reference)?;
                }
                ris_io::write(stream, &ris_asset_content)?;

                modified_file_content.into_inner()
            }
        };

        // write to compiled file
        let asset_addr = ris_io::seek(target_file, SeekFrom::Current(0))?;
        asset_lookup[i] = asset_addr;
        ris_io::write(target_file, &modified_file_content)?;
    }

    // all assets are compiled, compile original paths
    let p_original_asset_names = if options.include_original_paths {
        let original_paths = assets
            .iter()
            .map(|x| {
                Ok({
                    let mut original_path = x.to_str().into_ris_error()?.to_string();
                    original_path.replace_range(0..source.len(), "");
                    let mut original_path = original_path.replace('\\', "/");
                    if original_path.starts_with('/') {
                        original_path.remove(0);
                    }

                    original_path
                })
            })
            .collect::<RisResult<Vec<_>>>()?;

        let original_paths = original_paths
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<_>>();

        let begin = ris_io::seek(target_file, SeekFrom::Current(0))?;
        ris_io::write_uint(target_file, original_paths.len())?;
        for original_path in original_paths {
            ris_io::write_string(target_file, original_path)?;
        }
        let end = ris_io::seek(target_file, SeekFrom::Current(0))?;
        FatPtr::begin_end(begin, end)?
    } else {
        let addr = ris_io::seek(target_file, SeekFrom::Current(0))?;
        FatPtr { addr, len: 0 }
    };

    // fill placeholder
    ris_io::seek(target_file, SeekFrom::Start(addr_p_original_asset_names))?;
    ris_io::write_fat_ptr(target_file, p_original_asset_names)?;

    ris_io::seek(target_file, SeekFrom::Start(addr_asset_lookup))?;
    for asset_lookup_entry in asset_lookup.iter() {
        ris_io::write_u64(target_file, *asset_lookup_entry)?;
    }

    Ok(())
}

/// decompiles a .ris_asset file to a directory.
/// - `source`: the path to the compiled file
/// - `target`: the path to the final directory. if this directory exists already, it will be
/// cleared
pub fn decompile(source: &str, target: &str) -> RisResult<()> {
    // preparations
    let target = Path::new(target);
    if target.exists() {
        std::fs::remove_dir_all(target)?;
    }

    std::fs::create_dir_all(target)?;

    let mut source = File::open(source)?;
    let source = &mut source;

    // read magic
    let mut magic = [0; 16];
    ris_io::read(source, &mut magic)?;
    if !ris_util::testing::bytes_eq(&magic, &MAGIC) {
        return ris_error::new_result!("expected magic to be {:?} but was {:?}", magic, MAGIC);
    }

    // get original paths addr
    let p_original_asset_names = ris_io::read_fat_ptr(source)?;

    // read lookup
    let asset_lookup_count = ris_io::read_uint(source)?;
    let mut asset_lookup = vec![0; asset_lookup_count];
    for asset_lookup_entry in asset_lookup.iter_mut() {
        *asset_lookup_entry = ris_io::read_u64(source)?;
    }

    // read original paths
    let mut original_paths = if p_original_asset_names.len == 0 {
        Vec::new()
    } else {
        ris_io::seek(source, SeekFrom::Start(p_original_asset_names.addr))?;
        let original_path_count = ris_io::read_uint(source)?;
        let mut original_paths = Vec::with_capacity(original_path_count);
        for _ in 0..original_path_count {
            let original_path = ris_io::read_string(source)?;
            original_paths.push(original_path);
        }
        original_paths
    };

    let mut i = original_paths.len();
    while original_paths.len() < asset_lookup.len() {
        original_paths.push(format!("asset_{}", i));
        i += 1;
    }

    // read assets
    for i in 0..asset_lookup.len() {
        let asset_begin = asset_lookup[i];
        let original_path = &original_paths[i];

        ris_log::info!(
            "decompiling... {}/{} \"{}\"",
            i + 1,
            asset_lookup.len(),
            original_path,
        );

        let asset_end = if i == asset_lookup.len() - 1 {
            p_original_asset_names.addr
        } else {
            asset_lookup[i + 1]
        };

        let p_asset = FatPtr::begin_end(asset_begin, asset_end)?;
        let file_content = ris_io::read_at(source, p_asset)?;

        // reassign ids
        let modified_file_content = match RisHeader::load(&file_content)? {
            // asset is not a ris_asset, return unmodified
            None => file_content,

            // asset is ris_asset, change compiled id to directory id
            Some(old_header) => {
                let mut references = Vec::with_capacity(old_header.references.len());
                for reference in &old_header.references {
                    match reference {
                        AssetId::Path(id) => {
                            return ris_error::new_result!(
                                "attempted to decompile an already decompiled asset: {}",
                                id,
                            );
                        }
                        AssetId::Index(id) => {
                            let reference = &original_paths[*id];
                            let new_asset_id = AssetId::Path(reference.clone());
                            references.push(new_asset_id);
                        }
                    }
                }

                let new_header = RisHeader::new(old_header.magic, references);
                let new_header_content = new_header.serialize()?;

                let mut file_content = Cursor::new(file_content);
                let ris_asset_content = ris_io::read_at(&mut file_content, old_header.p_content())?;

                let mut modified_file_content = Cursor::new(Vec::new());
                ris_io::write(&mut modified_file_content, &new_header_content)?;
                ris_io::write(&mut modified_file_content, &ris_asset_content)?;

                modified_file_content.into_inner()

                //let mut file_content = Cursor::new(file_content);
                //let ris_asset_content = ris_io::read_at(&mut file_content, ris_header.p_content())?;

                //let mut modified_file_content = Cursor::new(Vec::new());
                //let stream = &mut modified_file_content;
                //ris_io::write(stream, &ris_header.magic)?;
                //ris_io::write_bool(stream, false)?;
                //let addr_p_content = ris_io::seek(stream, SeekFrom::Current(0))?;
                //ris_io::write_fat_ptr(stream, FatPtr::null())?; // placeholder
                //ris_io::write_uint(stream, references.len())?;
                //for reference in references {
                //    ris_io::write_string(stream, reference)?;
                //}
                //let p_content = ris_io::write(stream, &ris_asset_content)?;
                //ris_io::seek(stream, SeekFrom::Start(addr_p_content))?;
                //ris_io::write_fat_ptr(stream, p_content)?;

                //modified_file_content.into_inner()
            }
        };

        // create and write file
        let mut asset_path = PathBuf::new();
        asset_path.push(target);
        asset_path.push(original_path);
        let parent = asset_path.parent().into_ris_error()?;
        std::fs::create_dir_all(parent)?;

        let mut decompiled_file = File::create(&asset_path)?;
        ris_io::write(&mut decompiled_file, &modified_file_content)?;
    }

    Ok(())
}
