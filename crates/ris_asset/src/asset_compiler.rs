use std::collections::HashMap;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

// "ris_assets\0\0\0\0\0\0"
const MAGIC: [u8; 16] = [0x72, 0x69, 0x73, 0x5F, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; 
const ADDR_SIZE: usize = std::mem::size_of::<u64>();

/// compiles a directory from a .ris_asset file
/// - `source`: the directory to be compiled
/// - `target`: the path to the final compiled file. if this file exists already, it will be
/// overwritten
pub fn compile(source: &str, target: &str) -> Result<(), String> {
    let mut assets = Vec::new();
    let mut assets_lookup = HashMap::new();
    let mut directories = Vec::new();
    let source_path = PathBuf::from(source);
    directories.push(source_path);

    let mut index = 0;
    while index < directories.len(){
        let current_directory = &directories[index];
        let entries = std::fs::read_dir(current_directory)
            .map_err(|e| format!("failed to read directory \"{:?}\": {}", current_directory, e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("failed to read entry: {}", e))?;
            let metadata = entry.metadata().map_err(|e| format!("failed to read metadata: {}", e))?;
            
            let entry_path = entry.path();
            if metadata.is_file() {
                assets_lookup.insert(entry_path.clone(), assets.len());
                assets.push(entry_path);
            } else if metadata.is_dir() {
                directories.push(entry_path);
            } else {
                return Err(format!("entry \"{:?}\" is neither a file, nor a directory", entry_path));
            }
        }

        index += 1;
    }

    ris_log::trace!("found {} assets:", assets.len());
    for (i, file) in assets.iter().enumerate() {
        ris_log::trace!("{}: {:?}", i, file);
    }

    // TODO: reassign ids by reading the files and using the lookup

    let target_path = Path::new(target);
    if target_path.exists() {
        std::fs::remove_file(target_path)
            .map_err(|e| format!("failed to remove \"{:?}\": {}", target_path, e))?

    }

    let mut target = File::create(target_path)
        .map_err(|e| format!("failed to create \"{:?}\": {}", target_path, e))?;
    
    target.set_len(0).map_err(|e| format!("failed to clear target: {}", e))?;
    seek(&mut target, SeekFrom::Start(0))?;

    write(&mut target, &MAGIC)?;
    
    let addr_original_paths = seek(&mut target, SeekFrom::Current(0))?;
    write(&mut target, &[0; ADDR_SIZE])?; // placeholder

    let asset_count = assets.len() as u64;
    let asset_count_bytes = u64::to_le_bytes(asset_count);
    write(&mut target, &asset_count_bytes)?;
                                      //
    let addr_lookup = seek(&mut target, SeekFrom::Current(0))?;
    let lookup_size = ADDR_SIZE * asset_count as usize;
    write(&mut target, &vec![0; lookup_size])?; // placeholder

    for (i, asset) in assets.iter().enumerate() {
        let mut file = File::open(asset)
            .map_err(|e| format!("failed to open asset \"{:?}\": {}", &asset, e))?;

        let file_size = seek(&mut file, SeekFrom::End(0))? as usize;
        let mut file_content = vec![0; file_size];
        seek(&mut file, SeekFrom::Start(0))?;
        let read_byte_count = read(&mut file, &mut file_content)?;
        if read_byte_count != file_size {
            return Err(format!("expected to read {} bytes, but only read {}", file_size, read_byte_count));
        }

        let addr_asset = seek(&mut target, SeekFrom::Current(0))?;
        write(&mut target, &file_content)?;
        let addr_current = seek(&mut target, SeekFrom::Current(0))?;

        let addr_lookup_entry = addr_lookup + (ADDR_SIZE * i) as u64;
        seek(&mut target, SeekFrom::Start(addr_lookup_entry))?;
        let addr_asset_bytes = u64::to_le_bytes(addr_asset);
        write(&mut target, &addr_asset_bytes)?;

        seek(&mut target, SeekFrom::Start(addr_current))?;
    }

    let addr_current = seek(&mut target, SeekFrom::Current(0))?;
    seek(&mut target, SeekFrom::Start(addr_original_paths))?;
    let addr_current_bytes = u64::to_le_bytes(addr_current);
    write(&mut target, &addr_current_bytes)?;
    seek(&mut target, SeekFrom::Start(addr_current))?;

    for asset in assets {
        let mut original_path = String::from(asset.to_str().ok_or(String::from("asset path is not valid UTF8"))?);
        original_path.replace_range(0..source.len() + 1, "");
        let original_path = original_path.replace('\\', "/");
        let relative_path_bytes = original_path.as_bytes();
        write(&mut target, relative_path_bytes)?;
        write(&mut target, &[0])?; // seperate paths with \0
    }

    Ok(())
}

/// decompiles a .ris_asset file to a directory.
/// - `source`: the path to the compiled file
/// - `target`: the path to the final directory. if this directory exists already, it will be
/// cleared
pub fn decompile(source: &str, target: &str) -> Result<(), String> {
    Ok(())
}

fn seek(file: &mut File, pos: SeekFrom) -> Result<u64, String> {
    file.seek(pos).map_err(|e| format!("failed to seek: {}", e))
}

fn read(file: &mut File, buf: &mut [u8]) -> Result<usize, String> {
    file.read(buf).map_err(|e| format!("failed to read: {}", e))
}

fn write(file: &mut File, buf: &[u8]) -> Result<usize, String> {
    file.write(buf).map_err(|e| format!("failed to write: {}", e))
}
