use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SendError;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

use ris_asset_data::asset_id::AssetId;
use ris_async::JobFuture;
use ris_async::JobFutureSetter;
use ris_async::ThreadPool;
use ris_data::info::app_info::AppInfo;
use ris_error::RisResult;

use crate::asset_loader_compiled::AssetLoaderCompiled;
use crate::asset_loader_directory::AssetLoaderDirectory;
use crate::assets::ris_god_asset;

enum InternalLoader {
    Compiled(AssetLoaderCompiled),
    Directory(AssetLoaderDirectory),
}

pub struct Request {
    id: AssetId,
    setter: JobFutureSetter<Result<Vec<u8>, LoadError>>,
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
        let mut asset_loader_sender = ThreadPool::lock(&ASSET_LOADER_SENDER);
        *asset_loader_sender = None;

        ris_log::info!("asset loader guard dropped!");
    }
}

pub fn init(app_info: &AppInfo) -> RisResult<AssetLoaderGuard> {
    let asset_path = app_info.asset_path()?;
    let asset_path = Path::new(&asset_path);

    // create internal loader
    let metadata = asset_path.metadata()?;
    let (internal_loader, god_asset_id) = if metadata.is_file() {
        // compiled
        let loader = AssetLoaderCompiled::new(asset_path)?;
        let internal_loader = InternalLoader::Compiled(loader);
        let god_asset_id = AssetId::Index(0);
        ris_log::debug!("compiled asset loader was created");

        (internal_loader, god_asset_id)
    } else if metadata.is_dir() {
        // directory
        let loader = AssetLoaderDirectory::new(asset_path);
        let internal_loader = InternalLoader::Directory(loader);

        let god_asset_path = if PathBuf::from(asset_path).join(ris_god_asset::PATH).exists() {
            ris_god_asset::PATH
        } else if PathBuf::from(asset_path)
            .join(ris_god_asset::UNNAMED_PATH)
            .exists()
        {
            ris_god_asset::UNNAMED_PATH
        } else {
            return ris_error::new_result!("failed to locate god asset");
        };

        let god_asset_id = AssetId::Path(god_asset_path.to_string());
        ris_log::debug!("directory asset loader was created");

        (internal_loader, god_asset_id)
    } else {
        return ris_error::new_result!("assets are neither a file nor a directory");
    };

    // set up thread
    let (sender, receiver) = channel();
    let _ = std::thread::spawn(|| load_asset_thread(receiver, internal_loader));

    {
        let mut asset_loader_sender = ThreadPool::lock(&ASSET_LOADER_SENDER);
        *asset_loader_sender = Some(sender)
    }

    Ok(AssetLoaderGuard { god_asset_id })
}

pub fn load_async(id: AssetId) -> JobFuture<Result<Vec<u8>, LoadError>> {
    let (future, setter) = JobFuture::new();

    let request = Request { id, setter };

    let result = {
        let asset_loader_sender = ThreadPool::lock(&ASSET_LOADER_SENDER);
        match &*asset_loader_sender {
            Some(sender) => sender.send(request),
            None => Err(SendError(request)),
        }
    };

    if let Err(send_error) = result {
        let error = Err(LoadError::SendFailed);
        let request = send_error.0;
        request.setter.set(error);
    }

    future
}

fn load_asset_thread(receiver: Receiver<Request>, mut loader: InternalLoader) {
    for request in receiver.iter() {
        ris_log::trace!("loading asset {:?}...", request.id);

        let result = match &mut loader {
            InternalLoader::Compiled(loader) => match &request.id {
                AssetId::Index(id) => loader.load(*id).map_err(|e| {
                    ris_log::error!("failed loading {:?}: {}", id, e);
                    LoadError::LoadFailed
                }),
                AssetId::Path(id) => {
                    ris_log::error!(
                        "invalid id. expected compiled but was directory. id: {:?}",
                        id
                    );
                    Err(LoadError::InvalidId)
                }
            },
            InternalLoader::Directory(loader) => match request.id {
                AssetId::Index(id) => {
                    ris_log::error!(
                        "invalid id. expected directory but was compiled. id: {:?}",
                        id
                    );
                    Err(LoadError::InvalidId)
                }
                AssetId::Path(id) => loader.load(id.clone()).map_err(|e| {
                    ris_log::error!("failed loading {:?}: {}", id, e);
                    LoadError::LoadFailed
                }),
            },
        };

        request.setter.set(result);
    }

    ris_log::info!("load asset thread ended");
}
