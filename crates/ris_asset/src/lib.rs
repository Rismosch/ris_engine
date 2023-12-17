pub mod asset_compiler;
pub mod asset_importer;
pub mod asset_loader;
pub mod asset_loader_compiled;
pub mod asset_loader_directory;
pub mod codecs;
pub mod importer;
pub mod loader;
pub mod util;

pub const ADDR_SIZE: usize = std::mem::size_of::<u64>();
pub const FAT_ADDR_SIZE: usize = 2 * ADDR_SIZE;

#[derive(Debug, Clone)]
pub enum AssetId {
    Compiled(usize),
    Directory(String),
}

#[derive(Debug)]
pub struct RisAssetData {
    pub magic: [u8; crate::FAT_ADDR_SIZE],
    pub references: Vec<AssetId>,
    pub content_addr: u64,
}
