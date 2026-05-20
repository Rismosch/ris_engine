use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::mem::MaybeUninit;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;

use ris_asset_data::asset_id::AssetId;
use ris_asset_data::asset_id::AssetIdKind;
use ris_async::JobFuture;
use ris_async::JobFutureSetter;
use ris_async::UnsafeSender;
use ris_data::info::app_info::AppInfo;
use ris_error::prelude::*;
use ris_ptr::StrongPtr;

use crate::assets::ris_god_asset;
use crate::asset_future::AssetFuture;
use crate::assets::ris_asset::RisAsset;

// requests
enum CompiledLoadRequest {
    RisAsset(CompiledRisAssetLoadRequest),
    BinAsset(CompiledBinAssetLoadRequest),
}

struct CompiledRisAssetLoadRequest {
    asset_id: AssetId,
    size: usize,
    sender: UnsafeSender,
}


struct CompiledBinAssetLoadRequest {
    asset_id: AssetId,
    sender: JobFutureSetter<Box<[u8]>>,
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
        let asset_loader = if metadata.is_file() {
            // compiled
            unsafe {AssetId::set_kind(AssetIdKind::Index)};

            // open file
            let mut file = std::fs::File::open(asset_path)?;

            // setup channel and thread
            let (sender, receiver) = channel();
            let sender = RequestSender::Compiled(sender);

            let _ = std::thread::spawn(move || {
                let test = receiver;
                //load_compiled_asset_thread(file, receiver)
            });

            // find god asset
            let god_asset_id = AssetId::from_index(0);
            ris_log::debug!("compiled asset loader was created");

            // return
            AssetLoader{
                god_asset_id,
                sender,
            }
        } else if metadata.is_dir() {
            todo!();
            //// directory
            //unsafe {AssetId::set_kind(AssetIdKind::Path)};

            //// setup channel and thread
            //let (sender, receiver) = channel();
            //let sender = RequestSender::Directory(sender);
            //let _ = std::thread::spawn(|| load_directory_asset_thread(receiver));

            //// find god asset
            //let god_asset_path = if PathBuf::from(asset_path).join(ris_god_asset::PATH).exists() {
            //    ris_god_asset::PATH
            //} else if PathBuf::from(asset_path)
            //    .join(ris_god_asset::UNNAMED_PATH)
            //    .exists()
            //{
            //    ris_god_asset::UNNAMED_PATH
            //} else {
            //    return ris_error::new_result!("failed to locate god asset");
            //};

            //let god_asset_id = AssetId::from_path(god_asset_path);
            //ris_log::debug!("directory asset loader was created");

            //// return
            //AssetLoader{
            //    god_asset_id,
            //    sender,
            //}
        } else {
            return ris_error::new_result!("assets are neither a file nor a directory");
        };

        // return
        Ok(StrongPtr::new(asset_loader))
    }

    pub fn god_asset_id(&self) -> AssetId {
        self.god_asset_id.clone()
    }

    /// # Safety
    ///
    /// `asset_id` must point to an asset that stores `T`
    pub unsafe fn load_async<T: RisAsset>(&self, asset_id: AssetId) -> AssetFuture<T> {
        match &self.sender {
            // load compiled
            RequestSender::Compiled(sender) => {
                let size = std::mem::size_of::<T>();
                let (future, setter) = AssetFuture::new();

                let request = CompiledRisAssetLoadRequest {
                    asset_id,
                    size,
                    sender: setter,
                };

                sender.send(CompiledLoadRequest::RisAsset(request));
                future
            },

            // load directory
            RequestSender::Directory(sender) => {
                todo!();
            },
        }
    }

    /// # Safety
    ///
    /// `asset_id` must point to a binary asset
    pub unsafe fn load_bin_async(&self, asset_id: AssetId) -> JobFuture<Box<[u8]>> {
        match &self.sender {
            // load compiled binary
            RequestSender::Compiled(sender) => {
                let (future, setter) = JobFuture::new();

                let request = CompiledBinAssetLoadRequest {
                    asset_id,
                    sender: setter,
                };

                sender.send(CompiledLoadRequest::BinAsset(request));
                future
            },

            // load directory binary
            RequestSender::Directory(sender) => {
                todo!();
            },
        }
    }
}

fn load_compiled_asset_thread(
    mut file: std::fs::File,
    receiver: Receiver<CompiledLoadRequest>,
) -> RisResult<()> {
    for request in receiver.iter() {
        match request {
            CompiledLoadRequest::RisAsset(request) => {
                let CompiledRisAssetLoadRequest { 
                    asset_id,
                    size,
                    sender,
                } = request;
                ris_log::trace!("loading asset {:?}...", asset_id);

                // prepare
                let index = unsafe {asset_id.index()};
                let p_data = sender.as_mut() as *mut u8;
                let data = unsafe {std::slice::from_raw_parts_mut(p_data, size)};

                // read
                file.seek(SeekFrom::Start(index))?;
                file.read_exact(data)?;

                // finalize
                unsafe {sender.assume_init()};
            },
            CompiledLoadRequest::BinAsset(request) => {
                let CompiledBinAssetLoadRequest {
                    asset_id,
                    sender,
                } = request;

                // prepare
                let index = unsafe {asset_id.index()};

                // read
                file.seek(SeekFrom::Start(index));
                let data = read_bin(&mut file)?;

                // finalize
                sender.set(data);
            },
        }
    }

    ris_log::info!("load asset thread ended");
    Ok(())
}

fn load_directory_asset_thread(receiver: Receiver<DirectoryLoadRequest>) {
    for request in receiver.iter() {
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

fn read_bin(file: &mut std::fs::File) -> RisResult<Box<[u8]>> {
    let mut size = MaybeUninit::<u32>::uninit();
    let buf = unsafe {std::slice::from_raw_parts_mut(
        size.as_mut_ptr() as *mut u8,
        std::mem::size_of::<u32>(),
    )};
    file.read_exact(buf)?;
    let size = unsafe {size.assume_init()};

    let mut data: Box<[MaybeUninit<u8>]> = Box::new_uninit_slice(size.try_into()?);
    let p_data = data.as_mut_ptr() as *mut u8;
    let mut buf = unsafe {std::slice::from_raw_parts_mut(p_data, data.len())};
    file.read_exact(&mut buf)?;

    Ok(unsafe {data.assume_init()})
}
