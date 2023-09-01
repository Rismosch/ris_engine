use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::thread::JoinHandle;

use ris_data::info::app_info::AppInfo;
use ris_jobs::job_future::JobFuture;
use ris_jobs::job_future::SettableJobFuture;
use ris_util::ris_error::RisError;

use crate::asset_id::AssetId;
use crate::asset_loader_compiled::AssetLoaderCompiled;
use crate::asset_loader_directory::AssetLoaderDirectory;

enum InternalLoader{
    Compiled(AssetLoaderCompiled),
    Directory(AssetLoaderDirectory),
}

pub struct AssetLoader {
    sender: Sender<AssetId>,
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
                return ris_util::result_err!("failed to find assets \"{}\"", &app_info.args.assets);
            }
        }

        // create internal loader
        let metadata = ris_util::unroll!(asset_path.metadata(), "failed to get metadata")?;
        let internal = if metadata.is_file() {
            let loader = AssetLoaderCompiled::new(asset_path);
            InternalLoader::Compiled(loader)
        } else if metadata.is_dir() {
            let loader = AssetLoaderDirectory::new(asset_path);
            InternalLoader::Directory(loader)
        } else {
            return ris_util::result_err!("assets are neither a file nor a directory");
        };

        // set up thread
        let (sender, receiver) = channel();
        let _  = std::thread::spawn(|| load_asset_thread(receiver, internal));

        Ok(Self{
            sender,
        })
    }

    //pub fn load(id: AssetId) -> JobFuture<Box<[u8]>> {
    //    let (job_future, settable_job_future) = SettableJobFuture::new();
    //    job_future
    //}
}

fn load_asset_thread(receiver: Receiver<AssetId>, loader: InternalLoader){
    match loader {
        InternalLoader::Compiled(loader) => {
            for request in receiver.iter() {
                if let AssetId::Compiled(id) = request {
                    let result = loader.load(id);
                } else {
                    // error
                }
            }
        },
        InternalLoader::Directory(loader) => {
            for request in receiver.iter() {
                if let AssetId::Directory(id) = request {
                    let result = loader.load(id);
                } else {
                    // error
                }
            }
        },
    }

    ris_log::info!("load asset thread ended");
}
