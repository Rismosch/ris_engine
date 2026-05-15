use std::path::Path;
use std::path::PathBuf;

use ris_ptr::SyncUnsafeCell;

const NULL_PATH: &str = "NULL";

static ASSET_ID_KIND: SyncUnsafeCell<Option<AssetIdKind>> = SyncUnsafeCell::new(None);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetIdKind {
    Index,
    Path,
}

#[repr(C)]
pub union AssetId {
    index: u64,
    path: *mut PathBuf,
}

impl std::fmt::Debug for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match unsafe {*ASSET_ID_KIND.get()} {
            Some(AssetIdKind::Index) => unsafe {
                write!(f, "AssetId {{ index: {} }}", self.index())
            },
            Some(AssetIdKind::Path) => unsafe {
                write!(f, "AssetId {{ path: {} }}", self.path().display())
            },
            None => ris_error::panic!("asset id kind is not set"),
        }
    }
}

impl Clone for AssetId {
    fn clone(&self) -> Self {
        match unsafe {*ASSET_ID_KIND.get()} {
            Some(AssetIdKind::Index) => unsafe {
                let index = self.index();
                Self::from_index(index)
            },
            Some(AssetIdKind::Path) => unsafe {
                let path = self.path();
                Self::from_path(path)
            },
            None => ris_error::panic!("asset id kind is not set"),
        }
    }
}

impl PartialEq for AssetId {
    fn eq(&self, other: &Self) -> bool {
        match unsafe {*ASSET_ID_KIND.get()} {
            Some(AssetIdKind::Index) => unsafe {
                self.index() == other.index()
            },
            Some(AssetIdKind::Path) => unsafe {
                self.path() == other.path()
            },
            None => ris_error::panic!("asset id kind is not set"),
        }
    }
}

impl Eq for AssetId {}

impl Drop for AssetId {
    fn drop(&mut self) {
        unsafe {
            if *ASSET_ID_KIND.get() == Some(AssetIdKind::Path) {
                _ = Box::from_raw(self.path)
            }
        }
    }
}

impl AssetId {
    // global
    pub unsafe fn set_kind(kind: AssetIdKind) {
        unsafe {
            let current = ASSET_ID_KIND.get();
            *current = Some(kind);
        }
    }

    // constructors
    pub fn from_index(v: u64) -> Self {
        Self { index: v }
    }

    pub fn from_path(p: impl AsRef<Path>) -> Self {
        let p = p.as_ref().to_path_buf();
        let ptr = Box::into_raw(Box::new(p));

        Self { path: ptr }
    }

    pub fn null() -> Self {
        match unsafe {*ASSET_ID_KIND.get()} {
            Some(AssetIdKind::Index) => AssetId::from_index(u64::MAX),
            Some(AssetIdKind::Path) => AssetId::from_path(PathBuf::from(NULL_PATH)),
            None => ris_error::panic!("asset id kind is not set!"),
        }
    }

    // getter
    pub unsafe fn index(&self) -> u64 {
        unsafe { self.index }
    }

    pub unsafe fn path(&self) -> &Path {
        unsafe { &(*self.path) }
    }
}
