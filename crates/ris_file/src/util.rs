use std::path::Path;

pub static mut TRACE: bool = false;

macro_rules! trace {
    ($($arg:tt)*) => {
        if unsafe {$crate::util::TRACE} {
            eprintln!($($arg)*);
        }
    };
}

pub fn clean_or_create_dir(dir: &Path) -> std::io::Result<()> {
    if !dir.exists() {
        trace!("creating dir... {:?}", dir);
        std::fs::create_dir_all(dir)?;
    } else {
        trace!("cleaning dir... {:?}", dir);
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

        trace!("finished cleaning {:?}!", dir);
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
