use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssetId {
    Index(usize),
    Path(String),
}

impl AssetId {
    pub fn has_extension(&self, extension: impl AsRef<str>) -> bool {
        let AssetId::Path(path) = &self else {
            ris_log::error!("cannot determine extension on index asset id");
            return false;
        };

        let mut splits = path.split('.');
        let Some(last) = splits.next_back() else {
            ris_log::error!("asset has no extension");
            return false;
        };

        last.to_lowercase() == extension.as_ref().to_lowercase()
    }
}

pub static ASSET_ID_2_KIND: Option<AssetId2Kind> = None;

fn assert_asset_id_2_kind(kind: AssetId2Kind) {
    let actual = ASSET_ID_2_KIND;
    let expected = Some(kind);
    if actual != expected {
        ris_error::panic!(
            "expected ASSET_ID_2_KIND to be {:?} but was {:?}",
            expected,
            actual,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetId2Kind {
    Index,
    Path,
}

#[repr(C)]
pub union AssetId2 {
    index: u64,
    path: *mut PathBuf,
}

impl Drop for AssetId2 {
    fn drop(&mut self) {
        if ASSET_ID_2_KIND == Some(AssetId2Kind::Path) {
            _ = unsafe {Box::from_raw(self.path)}
        }
    }
}

impl AssetId2 {
    pub fn from_index(v: u64) -> Self {
        assert_asset_id_2_kind(AssetId2Kind::Index);

        Self { index: v }
    }

    pub fn from_path(p: impl AsRef<Path>) -> Self {
        assert_asset_id_2_kind(AssetId2Kind::Path);

        let p = p.as_ref().to_path_buf();
        let ptr = Box::into_raw(Box::new(p));

        Self { path: ptr }
    }

    pub fn index(&self) -> u64 {
        assert_asset_id_2_kind(AssetId2Kind::Index);
        unsafe {self.index}
    }

    pub fn path(&self) -> &Path {
        assert_asset_id_2_kind(AssetId2Kind::Path);
        unsafe {&(*self.path)}
    }
}

