pub mod asset_compiler;
pub mod asset_importer;
pub mod asset_loader;
pub mod asset_loader_compiled;
pub mod asset_loader_directory;
pub mod assets;
pub mod codecs;
pub mod importer;

pub use assets::ris_god_asset::RisGodAsset;
pub use assets::ris_header::RisHeader;

pub use asset_loader::load_async;

#[derive(Debug, Clone)]
pub enum AssetId {
    Compiled(usize),
    Directory(String),
}
