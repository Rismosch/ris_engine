use std::collections::HashMap;
use std::ffi::c_void;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::thread::current;
use std::u64;

use ris_asset_data::asset_id;
use ris_asset_data::asset_id::AssetId;
use ris_asset_data::asset_id::AssetIdKind;
use ris_error::prelude::*;
use ris_io::FatPtr;
use ris_ptr::Janitor;

use crate::assets::ris_asset::RisAsset;
use crate::assets::ris_god_asset;
use crate::codecs::json;
use crate::codecs::json::JsonObject;
use crate::codecs::json::JsonValue;
use crate::RisGodAsset;

pub const DEFAULT_ASSET_DIRECTORY: &str = "assets/in_use";
pub const DEFAULT_COMPILED_FILE: &str = "ris_assets";
pub const DEFAULT_DECOMPILED_DIRECTORY: &str = "decompiled_assets";

const BIN_ASSET_EXTENSIONS: &[&str] = &[
    "qoi",
    "spv",
];

const VTABLES: &[(&str, VTable)] = &[
    (ris_god_asset::EXTENSION, VTable::new::<RisGodAsset>()),
];

#[derive(Debug, Clone)]
struct VTable {
    size: usize,
    alloc: unsafe fn() -> *mut c_void,
    from_json: unsafe fn(*mut c_void, &JsonObject) -> RisResult<()>,
    all_references_mut: unsafe fn(*mut c_void) -> Vec<&'static mut AssetId>,
    destructor: unsafe fn(*mut c_void),
}

impl VTable {
    const fn new<T: RisAsset + 'static>() -> Self {
        use crate::assets::ris_asset;

        Self {
            size: std::mem::size_of::<T>(),
            alloc: ris_asset::impl_alloc::<T>,
            from_json: ris_asset::impl_from_json::<T>,
            all_references_mut: ris_asset::impl_all_references_mut::<T>,
            destructor: ris_asset::impl_destructor::<T>,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct CompileSettings {
    pub include_original_paths: bool,
}

#[derive(Debug, Clone)]
enum AssetKind {
    Ris(VTable),
    Bin,
}

#[derive(Debug, Clone)]
struct AssetUnit {
    path: PathBuf,
    id_path: String,
    id_index: u64,
    kind: AssetKind,
    filesize: u64,
}

/// compiles a directory to a ris_asset file
/// - `source`: the directory to be compiled
/// - `target`: the path to the final compiled file. if this file exists already, it will be overwritten
pub fn compile(source: &str, target: &str, settings: CompileSettings) -> RisResult<()> {
    // asset compiler makes use of both index and path ids
    if AssetId::kind() != None {
        return ris_error::new_result!("expected AssetId::kind() to be None, but was {:?}.", AssetId::kind());
    }

    // initialize
    let source = clean_path(source);
    if std::fs::exists(target)? {
        std::fs::remove_file(target)?;
    }

    let mut target_file = std::fs::File::create_new(target)?;

    // find all assets
    ris_log::info!("search all assets...");
    let mut assets = std::collections::HashMap::<String, AssetUnit>::new();
    let mut directories = std::collections::VecDeque::new();
    directories.push_back(PathBuf::from(&source));

    while let Some(current) = directories.pop_front() {
        let entries = std::fs::read_dir(&current)?;
        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let entry_path = entry.path();

            if metadata.is_dir() {
                directories.push_back(entry_path.clone());
                continue;
            } else if !metadata.is_file() {
                ris_log::warning!(
                    "asset \"{}\" was neither a dir, nor a file, and will be ignored",
                    entry_path.display(),
                );
                continue;
            }

            // file found! check if known...
            let extension = entry_path
                .extension()
                .ris_expect("path to have extension")?
                .to_str()
                .ris_expect("path to be valid utf-8")?
                .to_lowercase();

            let kind = if BIN_ASSET_EXTENSIONS.contains(&extension.as_str()) {
                AssetKind::Bin
            } else {
                let mut result = None;

                for (key, vtable) in VTABLES.iter() {
                    if *key != extension {
                        continue;
                    }

                    result = Some(vtable.clone());
                    break;
                }

                match result {
                    Some(deserialized) => {
                        AssetKind::Ris(deserialized)
                    },
                    None => {
                        ris_log::warning!("asset \"{}\" has unknown extension and will be ignored", entry_path.display());
                        continue;
                    }
                }
            };

            // compute asset id
            let mut prefix = source.clone();
            if !prefix.ends_with('/') {
                prefix.push('/');
            };

            let cleaned = clean_path(&entry_path);
            let id = cleaned
                .strip_prefix(&prefix)
                .ris_expect(&format!(
                    "path \"{}\" to start with prefix \"{}\"",
                    cleaned,
                    prefix,
                ))?
                .to_string();

            let asset_unit = AssetUnit {
                path: entry_path,
                id_path: id.clone(),
                id_index: u64::MAX,
                kind,
                filesize: u64::MAX,
            };

            ris_log::debug!("asset \"{}\" found", asset_unit.path.display());
            if assets.insert(id, asset_unit).is_some() {
                return ris_error::new_result!("expected asset id to be unique");
            };
        }
    }

    ris_log::info!("found {} assets", assets.len());

    // find references
    ris_log::info!("determine assets, which are actually referenced...");
    let god_asset = assets.get(ris_god_asset::PATH).ris_expect("god asset to exist")?;

    let mut references = std::collections::VecDeque::<AssetUnit>::new();
    references.push_back(god_asset.clone());

    let mut referenced_assets = std::collections::HashMap::<String, AssetUnit>::new();

    let mut counter = 0;
    while let Some(mut reference) = references.pop_front() {
        let key = reference.id_path.clone();
        if referenced_assets.contains_key(&key) {
            continue;
        }

        counter += 1;
        ris_log::debug!("finding reference... {}/{}", counter, assets.len());

        let mut file = std::fs::File::open(&reference.path)?;
        reference.filesize = file.seek(SeekFrom::End(0))?;
        file.seek(SeekFrom::Start(0))?;

        if let AssetKind::Ris(vtable) = &reference.kind {
            // deserialize asset and find references
            let mut file_content = String::new();
            file.read_to_string(&mut file_content)?;

            let JsonValue::Object(json) =  JsonValue::deserialize(file_content)? else {
                return ris_error::new_result!("file content is not a JsonObject");
            };

            unsafe {
                let j = Janitor {
                    data: (vtable.alloc)(),
                    destructor: vtable.destructor,
                };

                (vtable.from_json)(j.data, &json)?;
                let ids = (vtable.all_references_mut)(j.data);

                for id in ids {
                    if id.is_null_path() {
                        continue;
                    }

                    let clean_id = clean_path(id.path());
                    if let Some(asset) = assets.get(&clean_id) {
                        references.push_back(asset.clone());
                    } else {
                        ris_log::warning!(
                            "asset \"{}\" references a non-existant asset: \"{}\"",
                            reference.path.display(),
                            id.path().display(),
                        )
                    }
                }
            }
        }

        if referenced_assets.insert(key, reference).is_some() {
            return ris_error::new_result!("expected asset id to be unique");
        }
    }

    let pruned_len = assets.len() - referenced_assets.len();
    let pruned_percentage = 100.0 * pruned_len as f32 / assets.len() as f32;

    ris_log::info!(
        "{} assets ({:.1}%) are not referenced at all and wont be compiled",
        pruned_len,
        pruned_percentage,
    );

    // compute asset ids
    ris_log::info!("flattening referenced asset hashmap to list...");
    let mut asset_list = referenced_assets
        .into_iter()
        .map(|(_, value)| value)
        .collect::<Vec<_>>();
    let god_asset_index = asset_list
        .iter()
        .position(|asset| asset.id_path == ris_god_asset::PATH)
        .ris_expect("god asset to exist")?;

    let from = god_asset_index;
    let to = 0;
    asset_list.swap(from, to);

    ris_log::debug!("moved god asset from position {} to {}", from, to);

    let mut asset_index_lookup = std::collections::HashMap::new();

    let mut expected_size = 0;
    for (index, asset) in asset_list.iter_mut().enumerate() {
        asset.id_index = expected_size.try_into()?;
        let asset_size = match &asset.kind {
            AssetKind::Bin => std::mem::size_of::<u32>() as u64 + asset.filesize,
            AssetKind::Ris(vtable) => vtable.size as u64,
        };

        expected_size += asset_size;

        if asset_index_lookup.insert(asset.id_path.clone(), index).is_some() {
            return ris_error::new_result!("expected asset id to be unique");
        }
    }

    // compile file
    ris_log::info!("write file...");

    for (i, asset) in asset_list.iter().enumerate() {
        ris_log::debug!("asset {}/{} \"{}\"", i + 1, asset_list.len(), asset.id_path);

        let mut asset_file = std::fs::File::open(&asset.path)?;
        let mut file_content = vec![0u8; asset.filesize as usize];
        asset_file.read_exact(&mut file_content)?;

        match &asset.kind {
            AssetKind::Bin => {
                ris_log::debug!("write binary asset...");
                let size = file_content.len() as u32;
                target_file.write_all(&size.to_ne_bytes())?;
                target_file.write_all(&file_content)?;
            },
            AssetKind::Ris(vtable) => {
                unsafe {
                    let j = Janitor {
                        data: (vtable.alloc)(),
                        destructor: vtable.destructor,
                    };

                    let json_string = String::from_utf8(file_content)?;
                    let JsonValue::Object(json) = JsonValue::deserialize(json_string)? else {
                        return ris_error::new_result!("file content was not json");
                    };

                    (vtable.from_json)(j.data, &json)?;
                    ris_log::debug!("resolve references...");

                    let references = (vtable.all_references_mut)(j.data);
                    for reference in references {
                        if reference.is_null_path() {
                            *reference = AssetId::null_index();
                        } else {
                            let reference_id = clean_path(reference.path());
                            *reference = match asset_index_lookup.get(&reference_id) {
                                Some(&reference_index) => {
                                    let reference_asset = &asset_list[reference_index];
                                    AssetId::from_index_unchecked(reference_asset.id_index)
                                },
                                None => AssetId::null_index(),
                            };
                        }
                    }

                    let bytes = std::slice::from_raw_parts(
                        j.data as *mut u8,
                        vtable.size,
                    );

                    ris_log::debug!("write ris asset...");
                    target_file.write_all(&bytes)?;
                }
            },
        };
    }

    // append original paths
    if settings.include_original_paths {
        ris_log::info!("append original paths...");

        let addr = target_file.seek(SeekFrom::Current(0))?;

        for asset in asset_list.iter() {
            let bytes = asset.id_path.as_bytes();
            target_file.write_all(bytes)?;
            target_file.write_all(&[0u8])?;
        }

        target_file.write_all(&addr.to_ne_bytes())?;
        target_file.write_all(&[1u8])?;
    } else {

        target_file.write_all(&[0u8])?;
    }

    ris_log::info!("compiled all assets!");
    Ok(())
}

/// decompiles a .ris_asset file to a directory.
/// - `source`: the path to the compiled file
/// - `target`: the path to the final directory. if this directory exists already, it will be cleared
pub fn decompile(source: &str, target: &str) -> RisResult<()> {
    todo!();
}

fn clean_path(path: impl AsRef<Path>) -> String {
    path
        .as_ref()
        .display()
        .to_string()
        .replace('\\', "/")
}

