use std::io::Read;
use std::mem::MaybeUninit;
use std::path::Path;
use std::path::PathBuf;

use ris_asset_data::RawAssetId;
use ris_asset_data::asset_id::AssetId;
use ris_asset_data::asset_id::BinAssetId;
use ris_async::SingleUseReceiver;
use ris_error::prelude::*;

use crate::RisGodAsset;
use crate::asset_future::AssetFuture;
use crate::assets::ris_asset::RisAsset;
use crate::assets::ris_god_asset;

//==============================================================================
// Compiled Asset Loader
//==============================================================================

#[cfg(not(debug_assertions))]
mod compiled {
    use std::io::Read;
    use std::io::Seek;
    use std::io::SeekFrom;
    use std::path::Path;
    use std::sync::mpsc::Receiver;
    use std::sync::mpsc::Sender;
    use std::sync::mpsc::channel;

    use ris_async::SingleUseReceiver;
    use ris_async::SingleUseSender;
    use ris_async::UnsafeSender;
    use ris_error::prelude::*;

    use crate::asset_future::AssetFuture;
    use crate::assets::ris_asset::RisAsset;

    enum CompiledLoadRequest {
        RisAsset(CompiledRisAssetLoadRequest),
        BinAsset(CompiledBinAssetLoadRequest),
    }

    struct CompiledRisAssetLoadRequest {
        asset_id: u64,
        sender: UnsafeSender,
        size: usize,
    }

    struct CompiledBinAssetLoadRequest {
        asset_id: u64,
        sender: SingleUseSender<Box<[u8]>>,
    }

    pub struct CompiledAssetLoader {
        sender: Sender<CompiledLoadRequest>,
    }

    impl CompiledAssetLoader {
        pub fn new(asset_path: impl AsRef<Path>) -> RisResult<Self> {
            let asset_path = asset_path.as_ref();

            let metadata = asset_path.metadata()?;
            if !metadata.is_file() {
                return ris_error::new!("asset \"{}\" is not a file", asset_path.display());
            }

            // open file
            let file = std::fs::File::open(asset_path)?;

            // setup channel and thread
            let (sender, receiver) = channel();
            let _ = std::thread::spawn(move || {
                if let Err(e) = load_compiled_asset_thread(file, receiver) {
                    e.panic()
                }
            });

            // return
            ris_log::debug!("compiled asset loader was created");
            Ok(CompiledAssetLoader { sender })
        }

        pub unsafe fn load_async<T: RisAsset>(&self, asset_id: u64) -> RisResult<AssetFuture<T>> {
            let (future, sender) = AssetFuture::new();
            let size = std::mem::size_of::<T>();

            let request = CompiledRisAssetLoadRequest {
                asset_id,
                sender,
                size,
            };

            self.sender.send(CompiledLoadRequest::RisAsset(request))?;
            Ok(future)
        }

        pub unsafe fn load_bin_async(
            &self,
            asset_id: u64,
        ) -> RisResult<SingleUseReceiver<Box<[u8]>>> {
            let (sender, receiver) = ris_async::single_use_channel();

            let request = CompiledBinAssetLoadRequest { asset_id, sender };

            self.sender.send(CompiledLoadRequest::BinAsset(request))?;
            Ok(receiver)
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
                    let index = asset_id;
                    let p_data = sender.as_mut() as *mut u8;
                    let data = unsafe { std::slice::from_raw_parts_mut(p_data, size) };

                    // read
                    file.seek(SeekFrom::Start(index))?;
                    file.read_exact(data)?;

                    // finalize
                    unsafe { sender.assume_init() };
                }
                CompiledLoadRequest::BinAsset(request) => {
                    let CompiledBinAssetLoadRequest { asset_id, sender } = request;

                    // prepare
                    let index = asset_id;

                    // read
                    file.seek(SeekFrom::Start(index))?;
                    let data = super::read_compiled_bin(&mut file)?;

                    // finalize
                    sender.send(data);
                }
            }
        }

        ris_log::info!("load asset thread ended");
        Ok(())
    }
}

//==============================================================================
// Directory Asset Loader
//==============================================================================

#[cfg(debug_assertions)]
mod directory {
    use std::ffi::c_void;
    use std::io::Read;
    use std::io::Seek;
    use std::io::SeekFrom;
    use std::path::Path;
    use std::path::PathBuf;
    use std::sync::mpsc::Receiver;
    use std::sync::mpsc::Sender;
    use std::sync::mpsc::channel;

    use ris_async::SingleUseReceiver;
    use ris_async::SingleUseSender;
    use ris_async::UnsafeSender;
    use ris_error::prelude::*;

    use crate::asset_future::AssetFuture;
    use crate::assets::ris_asset::RisAsset;
    use crate::codecs::json::JsonObject;
    use crate::codecs::json::JsonValue;

    enum DirectoryLoadRequest {
        RisAsset(DirectoryRisAssetLoadRequest),
        BinAsset(DirectoryBinAssetLoadRequest),
    }

    struct DirectoryRisAssetLoadRequest {
        asset_id: PathBuf,
        sender: UnsafeSender,
        init_callback: unsafe fn(*mut c_void, &JsonObject) -> RisResult<()>,
    }

    struct DirectoryBinAssetLoadRequest {
        asset_id: PathBuf,
        sender: SingleUseSender<Box<[u8]>>,
    }

    pub struct DirectoryAssetLoader {
        sender: Sender<DirectoryLoadRequest>,
        pub root_dir: PathBuf,
    }

    impl DirectoryAssetLoader {
        pub fn new(asset_path: impl AsRef<Path>) -> RisResult<Self> {
            let asset_path = asset_path.as_ref();

            let metadata = asset_path.metadata()?;
            if !metadata.is_dir() {
                return ris_error::new!("asset \"{}\" is not a directory", asset_path.display());
            }
            // copy path
            let asset_path_for_thread = asset_path.to_path_buf();

            // setup channel and thread
            let (sender, receiver) = channel();
            let _ = std::thread::spawn(move || {
                if let Err(e) = load_directory_asset_thread(asset_path_for_thread, receiver) {
                    e.panic();
                }
            });

            // return
            ris_log::debug!("directory asset loader was created");
            Ok(DirectoryAssetLoader {
                sender,
                root_dir: asset_path.to_path_buf(),
            })
        }

        pub unsafe fn load_async<T: RisAsset>(
            &self,
            asset_id: PathBuf,
        ) -> RisResult<AssetFuture<T>> {
            let (future, sender) = AssetFuture::new();
            let init_callback = crate::assets::ris_asset::impl_from_json::<T>;

            let request = DirectoryRisAssetLoadRequest {
                asset_id,
                sender,
                init_callback,
            };

            self.sender.send(DirectoryLoadRequest::RisAsset(request))?;
            Ok(future)
        }

        pub unsafe fn load_bin_async(
            &self,
            asset_id: PathBuf,
        ) -> RisResult<SingleUseReceiver<Box<[u8]>>> {
            let (sender, receiver) = ris_async::single_use_channel();

            let request = DirectoryBinAssetLoadRequest { asset_id, sender };

            self.sender.send(DirectoryLoadRequest::BinAsset(request))?;
            Ok(receiver)
        }
    }

    fn load_directory_asset_thread(
        root: PathBuf,
        receiver: Receiver<DirectoryLoadRequest>,
    ) -> RisResult<()> {
        for request in receiver.iter() {
            match request {
                DirectoryLoadRequest::RisAsset(request) => {
                    let DirectoryRisAssetLoadRequest {
                        asset_id,
                        sender,
                        init_callback,
                    } = request;

                    // prepare
                    let asset_path = root.join(asset_id);
                    let mut file = std::fs::File::open(asset_path)?;

                    // read
                    let mut file_content = String::new();
                    file.read_to_string(&mut file_content)?;

                    // deserialize
                    let ptr = sender.as_mut();
                    let JsonValue::Object(json) = JsonValue::deserialize(file_content)? else {
                        return ris_error::new!("file content is not a JsonObject");
                    };

                    unsafe { init_callback(ptr, &json) }?;

                    // finalize
                    unsafe { sender.assume_init() };
                }
                DirectoryLoadRequest::BinAsset(request) => {
                    let DirectoryBinAssetLoadRequest { asset_id, sender } = request;

                    // prepare
                    let asset_path = root.join(asset_id);
                    let mut file = std::fs::File::open(asset_path)?;
                    let size = file.seek(SeekFrom::End(0))?;
                    file.seek(SeekFrom::Start(0))?;

                    // read
                    let data = super::read_bin(&mut file, size.try_into()?)?;

                    // finalize
                    sender.send(data);
                }
            }
        }

        ris_log::info!("load asset thread ended");
        Ok(())
    }
}

//==============================================================================
// Generic Asset Loader
//==============================================================================
pub struct AssetLoader {
    #[cfg(not(debug_assertions))]
    inner: compiled::CompiledAssetLoader,
    #[cfg(debug_assertions)]
    inner: directory::DirectoryAssetLoader,
}

impl AssetLoader {
    pub fn new(asset_path: impl AsRef<Path>) -> RisResult<AssetLoader> {
        let inner;

        #[cfg(not(debug_assertions))]
        {
            inner = compiled::CompiledAssetLoader::new(asset_path)?;
        }

        #[cfg(debug_assertions)]
        {
            inner = directory::DirectoryAssetLoader::new(asset_path)?;
        }

        Ok(Self { inner })
    }

    pub fn god_asset_id(&self) -> RisResult<AssetId<RisGodAsset>> {
        #[cfg(not(debug_assertions))]
        {
            let raw_god_asset_id_value = 0;
            let raw_god_asset_id = unsafe { RawAssetId::from_index(raw_god_asset_id_value) };
            let god_asset_id = unsafe { AssetId::from_raw(raw_god_asset_id) };
            Ok(god_asset_id)
        }

        #[cfg(debug_assertions)]
        {
            let god_asset_path = if self.inner.root_dir.join(ris_god_asset::PATH).exists() {
                ris_god_asset::PATH
            } else if self
                .inner
                .root_dir
                .join(ris_god_asset::UNNAMED_PATH)
                .exists()
            {
                ris_god_asset::UNNAMED_PATH
            } else {
                return ris_error::new!("failed to locate god asset");
            };

            let raw_god_asset_id_value = PathBuf::from(god_asset_path);
            let raw_god_asset_id = unsafe { RawAssetId::from_path(raw_god_asset_id_value) };
            let god_asset_id = unsafe { AssetId::from_raw(raw_god_asset_id) };
            Ok(god_asset_id)
        }
    }

    pub fn load_async<T: RisAsset>(
        &self,
        asset_id: impl AsRef<AssetId<T>>,
    ) -> RisResult<AssetFuture<T>> {
        #[cfg(not(debug_assertions))]
        {
            let asset_id = unsafe { asset_id.as_ref().raw().index };
            unsafe { self.inner.load_async(asset_id) }
        }

        #[cfg(debug_assertions)]
        {
            let asset_id = unsafe { &*asset_id.as_ref().raw().path };
            unsafe { self.inner.load_async(asset_id.clone()) }
        }
    }

    pub fn load_bin_async(
        &self,
        asset_id: impl AsRef<BinAssetId>,
    ) -> RisResult<SingleUseReceiver<Box<[u8]>>> {
        #[cfg(not(debug_assertions))]
        {
            let asset_id = unsafe { asset_id.as_ref().raw().index };
            unsafe { self.inner.load_bin_async(asset_id) }
        }

        #[cfg(debug_assertions)]
        {
            let asset_id = unsafe { &*asset_id.as_ref().raw().path };
            unsafe { self.inner.load_bin_async(asset_id.clone()) }
        }
    }
}

#[cfg(not(debug_assertions))]
fn read_compiled_bin(s: &mut impl Read) -> RisResult<Box<[u8]>> {
    let mut size = MaybeUninit::<u32>::uninit();
    let buf = unsafe {
        std::slice::from_raw_parts_mut(size.as_mut_ptr() as *mut u8, std::mem::size_of::<u32>())
    };
    s.read_exact(buf)?;
    let size = unsafe { size.assume_init() };

    read_bin(s, size.try_into()?)
}

fn read_bin(s: &mut impl Read, size: usize) -> RisResult<Box<[u8]>> {
    let mut data: Box<[MaybeUninit<u8>]> = Box::new_uninit_slice(size);
    let p_data = data.as_mut_ptr() as *mut u8;
    let mut buf = unsafe { std::slice::from_raw_parts_mut(p_data, data.len()) };
    s.read_exact(&mut buf)?;

    Ok(unsafe { data.assume_init() })
}
