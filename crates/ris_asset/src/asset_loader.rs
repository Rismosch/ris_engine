use std::ffi::c_void;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::mem::MaybeUninit;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;

use ris_asset_data::asset_id;
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
use crate::codecs::json::JsonObject;
use crate::codecs::json::JsonValue;

// requests
enum CompiledLoadRequest {
    RisAsset(CompiledRisAssetLoadRequest),
    BinAsset(BinAssetLoadRequest),
}

enum DirectoryLoadRequest {
    RisAsset(DirectoryRisAssetLoadRequest),
    BinAsset(BinAssetLoadRequest),
}

struct CompiledRisAssetLoadRequest {
    asset_id: AssetId,
    sender: UnsafeSender,
    size: usize,
}

struct DirectoryRisAssetLoadRequest {
    asset_id: AssetId,
    sender: UnsafeSender,
    init_callback: unsafe fn(*mut c_void, &JsonObject) -> RisResult<()>,
}

struct BinAssetLoadRequest {
    asset_id: AssetId,
    sender: JobFutureSetter<Box<[u8]>>,
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
    pub fn new(app_info: &AppInfo) -> RisResult<StrongPtr<AssetLoader>> {
        let current_kind = AssetId::kind();
        if current_kind != None {
            ris_error::panic!("expected asset id kind to be None but was {:?}", current_kind);
        }

        let asset_path = app_info.asset_path()?;
        let asset_path = Path::new(&asset_path);

        // create internal loader
        let metadata = asset_path.metadata()?;
        let asset_loader = if metadata.is_file() {
            // compiled
            unsafe {AssetId::set_kind(AssetIdKind::Index)};

            // open file
            let file = std::fs::File::open(asset_path)?;

            // setup channel and thread
            let (sender, receiver) = channel();
            let sender = RequestSender::Compiled(sender);

            let _ = std::thread::spawn(move || {
                if let Err(e) = load_compiled_asset_thread(file, receiver) {
                    e.panic()
                }
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
            // directory
            unsafe {AssetId::set_kind(AssetIdKind::Path)};

            // copy path
            let asset_path_for_thread = asset_path.to_path_buf();

            // setup channel and thread
            let (sender, receiver) = channel();
            let sender = RequestSender::Directory(sender);
            let _ = std::thread::spawn(move || {
                if let Err(e) = load_directory_asset_thread(asset_path_for_thread, receiver) {
                    e.panic();
                }
            });

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
            AssetLoader{
                god_asset_id,
                sender,
            }
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
    pub unsafe fn load_async<T: RisAsset>(&self, asset_id: impl AsRef<AssetId>) -> RisResult<AssetFuture<T>> {
        let asset_id = asset_id.as_ref().clone();

        let (future, sender) = AssetFuture::new();

        match &self.sender {
            // load compiled
            RequestSender::Compiled(request_sender) => {
                let size = std::mem::size_of::<T>();

                let request = CompiledRisAssetLoadRequest {
                    asset_id,
                    sender,
                    size,
                };

                request_sender.send(CompiledLoadRequest::RisAsset(request))?;
            },

            // load directory
            RequestSender::Directory(request_sender) => {
                let init_callback = impl_from_json::<T>;

                let request = DirectoryRisAssetLoadRequest {
                    asset_id: asset_id.clone(),
                    sender,
                    init_callback,
                };

                request_sender.send(DirectoryLoadRequest::RisAsset(request))?;
            },
        }

        Ok(future)
    }

    /// # Safety
    ///
    /// `asset_id` must point to a binary asset
    pub unsafe fn load_bin_async(&self, asset_id: impl AsRef<AssetId>) -> RisResult<JobFuture<Box<[u8]>>> {
        let asset_id = asset_id.as_ref().clone();

        let (future, setter) = JobFuture::new();

        let request = BinAssetLoadRequest {
            asset_id,
            sender: setter,
        };

        match &self.sender {
            RequestSender::Compiled(sender) => sender.send(CompiledLoadRequest::BinAsset(request))?,
            RequestSender::Directory(sender) => sender.send(DirectoryLoadRequest::BinAsset(request))?,
        }

        Ok(future)
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
                    sender,
                    size,
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
                let BinAssetLoadRequest {
                    asset_id,
                    sender,
                } = request;

                // prepare
                let index = unsafe {asset_id.index()};

                // read
                file.seek(SeekFrom::Start(index))?;
                let data = read_bin(&mut file)?;

                // finalize
                sender.set(data);
            },
        }
    }

    ris_log::info!("load asset thread ended");
    Ok(())
}

fn load_directory_asset_thread(root: PathBuf, receiver: Receiver<DirectoryLoadRequest>) -> RisResult<()> {
    for request in receiver.iter() {
        match request {
            DirectoryLoadRequest::RisAsset(request) => {
                let DirectoryRisAssetLoadRequest {
                    asset_id,
                    sender,
                    init_callback,
                } = request;

                // prepare
                let asset_path = root.join(unsafe {asset_id.path()});
                let mut file = std::fs::File::open(asset_path)?;

                // read
                let mut file_content = String::new();
                file.read_to_string(&mut file_content)?;

                // deserialize
                let ptr = sender.as_mut();
                let JsonValue::Object(json) = JsonValue::deserialize(file_content)? else {
                    return ris_error::new_result!("file content is not a JsonObject");
                };

                unsafe {init_callback(ptr, &json)}?;

                // finalize
                unsafe {sender.assume_init()};
            },
            DirectoryLoadRequest::BinAsset(request) => {
                let BinAssetLoadRequest {
                    asset_id,
                    sender,
                } = request;

                // prepare
                let asset_path = root.join(unsafe {asset_id.path()});
                let mut file = std::fs::File::open(asset_path)?;

                // read
                let data = read_bin(&mut file)?;

                // finalize
                sender.set(data);
            },
        }
    }

    ris_log::info!("load asset thread ended");
    Ok(())
}

fn impl_from_json<T: RisAsset>(ptr: *mut c_void, json: &JsonObject) -> RisResult<()>{
    let ptr = ptr.cast::<MaybeUninit<T>>();
    let t = unsafe {&mut *ptr};
    T::from_json(t, json)
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
