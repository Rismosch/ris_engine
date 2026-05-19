use std::ffi::c_void;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;

use ris_asset_data::asset_id::AssetId;
use ris_asset_data::asset_id::AssetIdKind;
use ris_async::JobFuture;
use ris_data::info::app_info::AppInfo;
use ris_error::prelude::*;
use ris_ptr::StrongPtr;

use crate::assets::ris_asset::RisAsset;
use crate::assets::ris_god_asset;

// requests
struct CompiledLoadRequest {
    asset_id: AssetId,
    p_asset: *mut c_void,
    callback: Box<dyn FnOnce()>,
}

struct DirectoryLoadRequest {
    asset_id: AssetId,
    callback: Box<dyn FnOnce(&[u8])>,
}

// loader
enum RequestSender {
    Compiled(Sender<CompiledLoadRequest>),
    Directory(Sender<DirectoryLoadRequest>),
}

pub struct AssetLoader {
    sender: RequestSender,
    god_asset_id: AssetId,
}

impl AssetLoader {
    pub unsafe fn new(app_info: &AppInfo) -> RisResult<StrongPtr<AssetLoader>> {
        let asset_path = app_info.asset_path()?;
        let asset_path = Path::new(&asset_path);

        // create internal loader
        let metadata = asset_path.metadata()?;
        if metadata.is_file() {
            // compiled
            unsafe {AssetId::set_kind(AssetIdKind::Index)};

            // setup channel and thread
            let (sender, receiver) = channel();
            let sender = RequestSender::Compiled(sender);
            let _ = std::thread::spawn(|| load_compiled_asset_thread(receiver));

            // find god asset
            let god_asset_id = AssetId::from_index(0);
            ris_log::debug!("compiled asset loader was created");

            // return
            Ok(AssetLoader{
                god_asset_id,
                sender,
            })
        } else if metadata.is_dir() {
            // directory
            unsafe {AssetId::set_kind(AssetIdKind::Path)};

            // setup channel and thread
            let (sender, receiver) = channel();
            let sender = RequestSender::Directory(sender);
            let _ = std::thread::spawn(|| load_directory_asset_thread(receiver));

            // find god asset
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

            let god_asset_id = AssetId::from_path(god_asset_path);
            ris_log::debug!("directory asset loader was created");

            // return
            Ok(AssetLoader{
                god_asset_id,
                sender,
            })
        } else {
            return ris_error::new_result!("assets are neither a file nor a directory");
        };

        // return
    }

    pub fn god_asset_id(&self) -> AssetId {
        self.god_asset_id.clone()
    }

    pub fn load_async<T: RisAsset>(&self, asset_id: AssetId) -> JobFuture<T> {
        match &self.sender {
            // load compiled
            RequestSender::Compiled(sender) => {
                let asset: T = unsafe {std::mem::zeroed()};

                let (future, settable_future) = JobFuture::new();

                let align = std::mem::align_of::<T>();
                let callback = Box::new(|| {

                });

                let request = CompiledLoadRequest {
                    asset_id,
                    align,
                    callback,
                };

                sender.send(request);
                future
            },

            // load directory
            RequestSender::Directory(sender) => {
                todo!();
            },
        }
    }

    pub fn load_bin_async(&self, id: AssetId) -> JobFuture<Vec<u8>> {
        todo!();
    }
}

fn load_compiled_asset_thread(receiver: Receiver<CompiledLoadRequest>) {
    for mut request in receiver.iter() {
        ris_log::trace!("loading asset {:?}...", request.id());


                    // read asset size
                    // alloc asset
                    // read bytes

        //let result = match &mut loader {
        //    InternalLoader::Compiled(loader) => match request.id() {
        //        AssetId::Index(id) => loader.load(id),
        //        AssetId::Path(id) => ris_error::new_result!(
        //            "invalid id. expected compiled but was directory. id: {:?}",
        //            id
        //        ),
        //    },
        //    InternalLoader::Directory(loader) => match request.id() {
        //        AssetId::Index(id) => ris_error::new_result!(
        //            "invalid id. expected directory but was compiled. id: {:?}",
        //            id
        //        ),
        //        AssetId::Path(id) => loader.load(id.clone()),
        //    },
        //};

        //request.deserialize_and_send(result);
    }

    ris_log::info!("load asset thread ended");
}

fn load_directory_asset_thread(receiver: Receiver<DirectoryLoadRequest>) {
    for mut request in receiver.iter() {
        ////ris_log::trace!("loading asset {:?}...", request.id());

        //let result = match &mut loader {
        //    InternalLoader::Compiled(loader) => match request.id() {
        //        AssetId::Index(id) => loader.load(id),
        //        AssetId::Path(id) => ris_error::new_result!(
        //            "invalid id. expected compiled but was directory. id: {:?}",
        //            id
        //        ),
        //    },
        //    InternalLoader::Directory(loader) => match request.id() {
        //        AssetId::Index(id) => ris_error::new_result!(
        //            "invalid id. expected directory but was compiled. id: {:?}",
        //            id
        //        ),
        //        AssetId::Path(id) => loader.load(id.clone()),
        //    },
        //};

        //request.deserialize_and_send(result);
    }

    ris_log::info!("load asset thread ended");
}
