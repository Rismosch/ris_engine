pub mod asset_compiler;
pub mod asset_importer;
pub mod asset_loader;
pub mod asset_loader_compiled;
pub mod asset_loader_directory;
pub mod byte_stream;
pub mod importer;
pub mod loader;
pub mod util;

pub const ADDR_SIZE: usize = std::mem::size_of::<u64>();
pub const FAT_ADDR_SIZE: usize = 2 * ADDR_SIZE;
