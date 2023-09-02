use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use ris_data::info::app_info::AppInfo;
use ris_jobs::job_future::JobFuture;
use ris_jobs::job_future::SettableJobFuture;
use ris_util::ris_error::RisError;

use crate::asset_loader_compiled::AssetLoaderCompiled;
use crate::asset_loader_directory::AssetLoaderDirectory;

enum InternalLoader {
    Compiled(AssetLoaderCompiled),
    Directory(AssetLoaderDirectory),
}

pub enum AssetId {
    Compiled(u32),
    Directory(String),
}

pub struct Request {
    id: AssetId,
    future: SettableJobFuture<Response>,
}

pub type Response = Result<Box<[u8]>, LoadError>;

#[derive(Debug)]
pub enum LoadError {
    InvalidId,
    AssetNotFound,
    SendFailed,
}

impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidId => write!(f, "the wrong id has been passed to the currently loaded loader"),
            Self::AssetNotFound => write!(f, "no asset was found under the provided id"),
            Self::SendFailed => write!(f, "the request was not able to be send to the loading thread. this usually occurs when the loading thread doesn't exist anymore"),
        }
    }
}

pub struct AssetLoader {
    sender: Sender<Request>,
}

impl AssetLoader {
    pub fn new(app_info: &AppInfo) -> Result<Self, RisError> {
        let asset_path;

        // search for assets relative
        let mut path_buf = PathBuf::new();
        path_buf.push(&app_info.file.base_path);
        path_buf.push(String::from(&app_info.args.assets));
        let path = Path::new(&path_buf);
        if path.exists() {
            asset_path = path;
        } else {
            // relative assets not found
            // search for assets absolute
            path_buf = PathBuf::new();
            path_buf.push(String::from(&app_info.args.assets));
            let path = Path::new(&path_buf);
            if path.exists() {
                asset_path = path;
            } else {
                return ris_util::result_err!(
                    "failed to find assets \"{}\"",
                    &app_info.args.assets
                );
            }
        }

        // create internal loader
        let metadata = ris_util::unroll!(asset_path.metadata(), "failed to get metadata")?;
        let internal = if metadata.is_file() {
            let loader = AssetLoaderCompiled::new(asset_path)?;
            InternalLoader::Compiled(loader)
        } else if metadata.is_dir() {
            let loader = AssetLoaderDirectory::new(asset_path);
            InternalLoader::Directory(loader)
        } else {
            return ris_util::result_err!("assets are neither a file nor a directory");
        };

        // set up thread
        let (sender, receiver) = channel();
        let _ = std::thread::spawn(|| load_asset_thread(receiver, internal));

        Ok(Self { sender })
    }

    pub fn load(&self, id: AssetId) -> JobFuture<Response> {
        let (settable_job_future, job_future) = SettableJobFuture::new();
        let request = Request {
            id,
            future: settable_job_future,
        };
        let result = self.sender.send(request);
        if let Err(send_error) = result {
            let error = Err(LoadError::SendFailed);
            let request = send_error.0;
            request.future.set(error);
        }
        job_future
    }
}

fn load_asset_thread(receiver: Receiver<Request>, loader: InternalLoader) {
    match loader {
        InternalLoader::Compiled(loader) => {
            for request in receiver.iter() {
                if let AssetId::Compiled(id) = request.id {
                    let result = loader.load(id);
                    request.future.set(result);
                } else {
                    let error = Err(LoadError::InvalidId);
                    request.future.set(error);
                }
            }
        }
        InternalLoader::Directory(loader) => {
            for request in receiver.iter() {
                if let AssetId::Directory(id) = request.id {
                    let result = loader.load(id);
                    request.future.set(result);
                } else {
                    let error = Err(LoadError::InvalidId);
                    request.future.set(error);
                }
            }
        }
    }

    ris_log::info!("load asset thread ended");
}
