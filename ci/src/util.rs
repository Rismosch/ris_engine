use std::path::Path;

pub fn clean_or_create_dir(dir: &Path) -> std::io::Result<()> {
    if !dir.exists() {
        eprintln!("creating dir... {:?}", dir);
        std::fs::create_dir_all(dir)?;
    } else {
        eprintln!("cleaning dir... {:?}", dir);
        for entry in dir.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;
            if metadata.is_file() {
                std::fs::remove_file(path)?;
            } else if metadata.is_dir() {
                std::fs::remove_dir_all(path)?;
            } else {
                return Err(std::io::Error::from(std::io::ErrorKind::Other));
            }
        }

        eprintln!("finished cleaning {:?}!", dir);
    }

    Ok(())
}

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src = entry.path();
        let dst = dst.as_ref().join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_all(src, dst)?;
        } else {
            std::fs::copy(src, dst)?;
        }
    }

    Ok(())
}

pub fn sanitize_path(value: &str) -> String {
    const INVALID_CHARS: [char; 9] = [':', '*', '?', '"', '<', '>', '|', '\\', '/'];

    let mut value = String::from(value);
    for invalid_char in INVALID_CHARS {
        value = value.replace(invalid_char, "_");
    }

    value
}
