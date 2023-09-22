use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SendError;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

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
    Compiled(usize),
    Directory(String),
}

pub struct Request {
    id: AssetId,
    future: SettableJobFuture<Result<Vec<u8>, LoadError>>,
}

#[derive(Debug)]
pub enum LoadError {
    InvalidId,
    SendFailed,
    LoadFailed,
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
            Self::SendFailed => write!(f, "the request was not able to be send to the loading thread. this usually occurs when the loading doesn't exist"),
            Self::LoadFailed => write!(f, "asset could not be loaded. this may be because it doesn't exist, or because an io error occured when reading the file"),
        }
    }
}

static ASSET_LOADER_SENDER: Mutex<Option<Sender<Request>>> = Mutex::new(None);

pub struct AssetLoaderGuard;

impl Drop for AssetLoaderGuard {
    fn drop(&mut self) {
        match ASSET_LOADER_SENDER.lock() {
            Err(e) => ris_log::error!("error while dropping asset loader: {}", e),
            Ok(mut asset_loader_sender) => *asset_loader_sender = None,
        }
    }
}

pub fn init(app_info: &AppInfo) -> Result<AssetLoaderGuard, RisError> {
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
            return ris_util::result_err!("failed to find assets \"{}\"", &app_info.args.assets);
        }
    }

    // create internal loader
    let metadata = ris_util::unroll!(asset_path.metadata(), "failed to get metadata")?;
    let internal = if metadata.is_file() {
        let loader = ris_util::unroll!(
            AssetLoaderCompiled::new(asset_path),
            "failed to create compiled asset loader"
        )?;
        ris_log::debug!("compiled asset loader was created");
        InternalLoader::Compiled(loader)
    } else if metadata.is_dir() {
        let loader = AssetLoaderDirectory::new(asset_path);
        ris_log::debug!("directory asset loader was created");
        InternalLoader::Directory(loader)
    } else {
        return ris_util::result_err!("assets are neither a file nor a directory");
    };

    // set up thread
    let (sender, receiver) = channel();
    let _ = std::thread::spawn(|| load_asset_thread(receiver, internal));

    {
        let mut asset_loader_sender = ris_util::unroll!(
            ASSET_LOADER_SENDER.lock(),
            "failed to lock asset loader sender"
        )?;
        *asset_loader_sender = Some(sender)
    }

    Ok(AssetLoaderGuard)
}

pub fn load(id: AssetId) -> JobFuture<Result<Vec<u8>, LoadError>> {
    let (settable_job_future, job_future) = SettableJobFuture::new();
    let request = Request {
        id,
        future: settable_job_future,
    };

    let result = {
        match ASSET_LOADER_SENDER.lock() {
            Ok(asset_loader_sender) => match &*asset_loader_sender {
                Some(sender) => sender.send(request),
                None => Err(SendError(request)),
            },
            Err(_) => Err(SendError(request)),
        }
    };

    if let Err(send_error) = result {
        let error = Err(LoadError::SendFailed);
        let request = send_error.0;
        request.future.set(error);
    }

    job_future
}

fn load_asset_thread(receiver: Receiver<Request>, mut loader: InternalLoader) {
    match &mut loader {
        InternalLoader::Compiled(loader) => {
            for request in receiver.iter() {
                if let AssetId::Compiled(id) = request.id {
                    let result = loader.load(id).map_err(|e| {
                        ris_log::error!("{}", e);
                        LoadError::LoadFailed
                    });
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
                    let result = loader.load(id).map_err(|e| {
                        ris_log::error!("{}", e);
                        LoadError::LoadFailed
                    });
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
