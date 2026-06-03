use std::collections::HashMap;
use std::ffi::c_void;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::thread::current;
use std::u64;

use ris_asset_data::asset_id::AssetId;
use ris_asset_data::asset_id::AssetIdKind;
use ris_error::prelude::*;
use ris_io::FatPtr;

use crate::assets::ris_asset::RisAsset;
use crate::assets::ris_god_asset;
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
    alloc: unsafe fn() -> *mut c_void,
    from_json: unsafe fn(*mut c_void, &JsonObject) -> RisResult<()>,
    all_references_mut: unsafe fn(*mut c_void, fn(&[&mut AssetId])),
    destructor: unsafe fn(*mut c_void),
}

impl VTable {
    const fn new<T: RisAsset>() -> Self {
        use crate::assets::ris_asset;

        Self {
            alloc: ris_asset::impl_alloc::<T>,
            from_json: ris_asset::impl_from_json::<T>,
            all_references_mut: ris_asset::impl_all_references_mut::<T>,
            destructor: ris_asset::impl_destructor::<T>,
        }
    }
}

#[derive(Debug, Clone)]
struct DeserializedAsset {
    data: *mut c_void,
    vtable: VTable,
}

impl Drop for DeserializedAsset {
    fn drop(&mut self) {
        if self.data.is_null() {
            return;
        }

        unsafe {(self.vtable.destructor)(self.data)};
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct CompileOptions {
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
    extension: String,
    id_path: String,
    id_index: u64,
    kind: AssetKind,
    filesize: u64,
}

/// compiles a directory to a ris_asset file
/// - `source`: the directory to be compiled
/// - `target`: the path to the final compiled file. if this file exists already, it will be overwritten
pub fn compile(source: &str, target: &str, options: CompileOptions) -> RisResult<()> {
    // initialize
    let source = clean_path(source);

    let mut assets = std::collections::HashMap::<String, AssetUnit>::new();

    // find all assets
    ris_log::debug!("finding assets...");
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
                    Some(deserialized) => AssetKind::Ris(deserialized),
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
                .ris_expect("path to start with source")?
                .to_string();

            let asset_unit = AssetUnit {
                path: entry_path,
                extension,
                id_path: id.clone(),
                id_index: u64::MAX,
                kind,
                filesize: u64::MAX,
            };

            assets.insert(id, asset_unit);
        }
    }

    ris_log::info!("found {} assets:", assets.len());

    // count references
    ris_log::debug!("find referenced assets...");
    let god_asset = assets.get(ris_god_asset::PATH).ris_expect("god asset to exist")?;

    let mut references = std::collections::VecDeque::<AssetUnit>::new();
    references.push_back(god_asset.clone());

    let mut referenced_assets = std::collections::HashMap::<String, AssetUnit>::new();
    while let Some(mut reference) = references.pop_front() {
        let key = reference.id_path.clone();
        if referenced_assets.contains_key(&key) {
            continue;
        }

        let mut file = std::fs::File::open(&reference.path)?;
        reference.filesize = file.seek(SeekFrom::End(0))?;
        file.seek(SeekFrom::Start(0))?;

        if let AssetKind::Ris(vtable) = reference.kind {
            // deserialize asset and find references
            let mut file_content = String::new();
            file.read_to_string(&mut file_content)?;

            let JsonValue::Object(json) =  JsonValue::deserialize(file_content)? else {
                return ris_error::new_result!("file content is not a JsonObject");
            };

            unsafe {
                let asset = (vtable.alloc)();
                ((vtable.all_references_mut)(asset, |ids| {
                    for &id in ids {
                        if *id == AssetId::null() {
                            continue;
                        }

                        let clean_id = clean_path(id.path());
                        if let Some(asset) = assets.get(&clean_id) {
                            references.push_back(asset.clone());
                        }
                    }
                }));
                (vtable.destructor)(asset);
            }
        }

        referenced_assets.insert(key, reference);
    }

    ris_log::debug!("assets: {:#?}", referenced_assets);

    ris_log::info!(
        "{}/{} assets are referenced",
        referenced_assets.len(),
        assets.len(),
    );

    // compute asset ids
    ris_log::debug!("compute asset ids...");
    todo!("indices are enough");

    // change asset ids
    ris_log::debug!("change asset ids...");
    todo!("handle the existance of both index and path asset ids");

    // compile file
    ris_log::debug!("compiled file...");
    todo!();

    // append original paths
    ris_log::debug!("write original paths...");
    todo!();

    ris_log::debug!("compiled all assets!");
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
        .replace('\\', "")
}

