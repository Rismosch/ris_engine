use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SendError;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

use ris_data::info::app_info::AppInfo;
use ris_error::RisResult;
use ris_jobs::job_future::JobFuture;
use ris_jobs::job_future::SettableJobFuture;
use ris_jobs::job_system;

use crate::asset_loader_compiled::AssetLoaderCompiled;
use crate::asset_loader_directory::AssetLoaderDirectory;
use crate::AssetId;

const GOD_ASSET_PATH: &str = "god_asset.ris_god_asset";
const UNNAMED_GOD_ASSET_PATH: &str = "asset_0";

enum InternalLoader {
    Compiled(AssetLoaderCompiled),
    Directory(AssetLoaderDirectory),
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

pub struct AssetLoaderGuard {
    pub god_asset_id: AssetId,
}

impl Drop for AssetLoaderGuard {
    fn drop(&mut self) {
        let mut asset_loader_sender = job_system::lock(&ASSET_LOADER_SENDER);
        *asset_loader_sender = None;

        ris_log::info!("asset loader guard dropped!");
    }
}

pub fn init(app_info: &AppInfo) -> RisResult<AssetLoaderGuard> {
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
            return ris_error::new_result!("failed to find assets \"{}\"", &app_info.args.assets);
        }
    }

    // create internal loader
    let metadata = asset_path.metadata()?;
    let (internal_loader, god_asset_id) = if metadata.is_file() {
        // compiled
        let loader = AssetLoaderCompiled::new(asset_path)?;
        let internal_loader = InternalLoader::Compiled(loader);
        let god_asset_id = AssetId::Compiled(0);
        ris_log::debug!("compiled asset loader was created");

        (internal_loader, god_asset_id)
    } else if metadata.is_dir() {
        // directory
        let loader = AssetLoaderDirectory::new(asset_path);
        let internal_loader = InternalLoader::Directory(loader);

        let god_asset_path = if PathBuf::from(asset_path).join(GOD_ASSET_PATH).exists() {
            GOD_ASSET_PATH
        } else if PathBuf::from(asset_path)
            .join(UNNAMED_GOD_ASSET_PATH)
            .exists()
        {
            UNNAMED_GOD_ASSET_PATH
        } else {
            return ris_error::new_result!("failed to locate god asset");
        };

        let god_asset_id = AssetId::Directory(god_asset_path.to_string());
        ris_log::debug!("directory asset loader was created");

        (internal_loader, god_asset_id)
    } else {
        return ris_error::new_result!("assets are neither a file nor a directory");
    };

    // set up thread
    let (sender, receiver) = channel();
    let _ = std::thread::spawn(|| load_asset_thread(receiver, internal_loader));

    {
        let mut asset_loader_sender = job_system::lock(&ASSET_LOADER_SENDER);
        *asset_loader_sender = Some(sender)
    }

    Ok(AssetLoaderGuard { god_asset_id })
}

pub fn load_async(id: AssetId) -> JobFuture<Result<Vec<u8>, LoadError>> {
    let (settable_job_future, job_future) = SettableJobFuture::new();
    let request = Request {
        id,
        future: settable_job_future,
    };

    let result = {
        let asset_loader_sender = job_system::lock(&ASSET_LOADER_SENDER);
        match &*asset_loader_sender {
            Some(sender) => sender.send(request),
            None => Err(SendError(request)),
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
    for request in receiver.iter() {
        ris_log::trace!("loading asset {:?}...", request.id);

        let result = match &mut loader {
            InternalLoader::Compiled(loader) => match &request.id {
                AssetId::Compiled(id) => loader.load(*id).map_err(|e| {
                    ris_log::error!("failed loading {:?}: {}", id, e);
                    LoadError::LoadFailed
                }),
                AssetId::Directory(id) => {
                    ris_log::error!(
                        "invalid id. expected compiled but was directory. id: {:?}",
                        id
                    );
                    Err(LoadError::InvalidId)
                }
            },
            InternalLoader::Directory(loader) => match request.id {
                AssetId::Compiled(id) => {
                    ris_log::error!(
                        "invalid id. expected directory but was compiled. id: {:?}",
                        id
                    );
                    Err(LoadError::InvalidId)
                }
                AssetId::Directory(id) => loader.load(id.clone()).map_err(|e| {
                    ris_log::error!("failed loading {:?}: {}", id, e);
                    LoadError::LoadFailed
                }),
            },
        };

        request.future.set(result);
    }

    ris_log::info!("load asset thread ended");
}
