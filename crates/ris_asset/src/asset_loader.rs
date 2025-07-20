use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SendError;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

use ris_asset_data::asset_id::AssetId;
use ris_async::oneshot_channel;
use ris_async::OneshotReceiver;
use ris_async::OneshotSender;
use ris_async::ThreadPool;
use ris_data::info::app_info::AppInfo;
use ris_error::prelude::*;

use crate::asset_loader_compiled::AssetLoaderCompiled;
use crate::asset_loader_directory::AssetLoaderDirectory;
use crate::assets::ris_god_asset;

trait LoadRequest: Send {
    fn id(&self) -> AssetId;
    fn deserialize_and_send(&mut self, data: RisResult<Vec<u8>>);
}

struct GenericLoadRequest<T: Send, F: Send + FnOnce(Vec<u8>) -> RisResult<T>> {
    id: AssetId,
    inner: Option<GenericLoadRequestInner<T, F>>,
}

struct GenericLoadRequestInner<T: Send, F: Send + FnOnce(Vec<u8>) -> RisResult<T>> {
    deserializer: F,
    sender: OneshotSender<RisResult<T>>,
}

impl<T: Send, F: Send + FnOnce(Vec<u8>) -> RisResult<T>> LoadRequest for GenericLoadRequest<T, F> {
    fn id(&self) -> AssetId {
        self.id.clone()
    }

    fn deserialize_and_send(&mut self, data: RisResult<Vec<u8>>) {
        let Some(inner) = self.inner.take() else {
            ris_error::throw!("attempted to send load request multiple times");
        };

        let result = match data {
            Ok(bytes) => (inner.deserializer)(bytes),
            Err(e) => Err(e),
        };

        inner.sender.send(result)
    }
}

enum InternalLoader {
    Compiled(AssetLoaderCompiled),
    Directory(AssetLoaderDirectory),
}

static ASSET_LOADER_SENDER: Mutex<Option<Sender<Box<dyn LoadRequest>>>> = Mutex::new(None);

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

pub fn load_raw_async(id: AssetId) -> OneshotReceiver<RisResult<Vec<u8>>> {
    load_async(id, Ok)
}

pub fn load_async<T, F>(id: AssetId, deserializer: F) -> OneshotReceiver<RisResult<T>>
where
    T: Send + 'static,
    F: FnOnce(Vec<u8>) -> RisResult<T> + Send + 'static,
{
    let (sender, receiver) = oneshot_channel();

    let request: Box<dyn LoadRequest> = Box::new(GenericLoadRequest {
        id,
        inner: Some(GenericLoadRequestInner {
            deserializer,
            sender,
        }),
    });

    let result = {
        let asset_loader_sender = ThreadPool::lock(&ASSET_LOADER_SENDER);
        match &*asset_loader_sender {
            Some(sender) => sender.send(request),
            None => Err(SendError(request)),
        }
    };

    if let Err(send_error) = result {
        let error = ris_error::new_result!("failed to send: {}", send_error);
        let mut request = send_error.0;
        request.deserialize_and_send(error);
    }

    receiver
}

fn load_asset_thread(receiver: Receiver<Box<dyn LoadRequest>>, mut loader: InternalLoader) {
    for mut request in receiver.iter() {
        //ris_log::trace!("loading asset {:?}...", request.id());

        let result = match &mut loader {
            InternalLoader::Compiled(loader) => match request.id() {
                AssetId::Index(id) => loader.load(id),
                AssetId::Path(id) => ris_error::new_result!(
                    "invalid id. expected compiled but was directory. id: {:?}",
                    id
                ),
            },
            InternalLoader::Directory(loader) => match request.id() {
                AssetId::Index(id) => ris_error::new_result!(
                    "invalid id. expected directory but was compiled. id: {:?}",
                    id
                ),
                AssetId::Path(id) => loader.load(id.clone()),
            },
        };

        request.deserialize_and_send(result);
    }

    ris_log::info!("load asset thread ended");
}
