use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct CmdStream<Tstdout: Write, Tstderr: Write>{
    pub stdout: Tstdout,
    pub stderr: Tstderr,
}

impl CmdStream<File, File> {
    pub fn new(dir: &Path, filename: &str) -> std::io::Result<Self> {
        let sanitized = crate::util::sanitize_path(filename);
        let stdout_filepath = dir.join(format!("{}.stdout.log", sanitized));
        let stderr_filepath = dir.join(format!("{}.stderr.log", sanitized));
        
        std::fs::create_dir_all(dir)?;
        let stdout = std::fs::File::create(&stdout_filepath)?;
        let stderr = std::fs::File::create(&stderr_filepath)?;

        eprintln!("created stream");
        eprintln!("stdout: {:?}", stdout_filepath);
        eprintln!("stderr: {:?}", stderr_filepath);

        Ok(Self {
            stdout,
            stderr,
        })
    }
}
