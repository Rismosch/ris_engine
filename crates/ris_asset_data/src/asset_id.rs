use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::path::Path;
use std::path::PathBuf;

use ris_error::prelude::*;
use ris_ptr::SyncUnsafeCell;

pub const NULL_PATH: &str = "NULL";

pub static ASSET_ID_KIND: SyncUnsafeCell<Option<AssetIdKind>> = SyncUnsafeCell::new(None);

//==============================================================================
// Structs
//==============================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetIdKind {
    Index,
    Path,
}

#[repr(C)]
pub union RawAssetId {
    index: u64,
    path: *mut PathBuf,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetId<T> {
    raw: RawAssetId,
    boo: PhantomData<T>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinAssetId {
    raw: RawAssetId,
}

//==============================================================================
// Traits
//==============================================================================
impl Drop for RawAssetId {
    fn drop(&mut self) {
        unsafe {
            if *ASSET_ID_KIND.get() == Some(AssetIdKind::Path) {
                _ = Box::from_raw(self.path)
            }
        }
    }
}

impl std::fmt::Debug for RawAssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match unsafe{*ASSET_ID_KIND.get()} {
            Some(AssetIdKind::Index) => unsafe {
                write!(f, "RawAssetId {{ index: {} }}", self.index())
            },
            Some(AssetIdKind::Path) => unsafe {
                write!(f, "RawAssetId {{ path: {} }}", self.path().display())
            },
            None => unsafe {
                write!(f, "RawAssetId {{ ?: {} }}", self.index())
            },
        }
    }
}

impl Clone for RawAssetId {
    fn clone(&self) -> Self {
        match unsafe{*ASSET_ID_KIND.get()} {
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

impl PartialEq for RawAssetId {
    fn eq(&self, other: &Self) -> bool {
        match unsafe{*ASSET_ID_KIND.get()} {
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

impl Eq for RawAssetId {}

impl AsRef<RawAssetId> for RawAssetId {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T> AsRef<AssetId<T>> for AssetId<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<BinAssetId> for BinAssetId {
    fn as_ref(&self) -> &Self {
        self
    }
}

unsafe impl Send for RawAssetId {}
unsafe impl<T> Send for AssetId<T> {}
unsafe impl Send for BinAssetId {}

//==============================================================================
// Impl RawAssetId
//==============================================================================
impl RawAssetId {
    // constructors
    pub unsafe fn from_index_unchecked(v: u64) -> Self {
        Self { index: v }
    }

    pub unsafe fn from_path_unchecked(p: impl AsRef<Path>) -> Self {
        let mut id = MaybeUninit::<Self>::uninit();

        unsafe {
            (*id.as_mut_ptr()).set_path(p);
            id.assume_init()
        }
    }

    pub fn from_index(v: u64) -> Self {
        ris_error::panic_assert!(unsafe{*ASSET_ID_KIND.get()} == Some(AssetIdKind::Index));
        unsafe {Self::from_index_unchecked(v)}
    }

    pub fn from_path(p: impl AsRef<Path>) -> Self {
        ris_error::panic_assert!(unsafe{*ASSET_ID_KIND.get()} == Some(AssetIdKind::Path));
        unsafe{Self::from_path_unchecked(p)}
    }

    pub unsafe fn null_index() -> Self {
        RawAssetId::from_index(u64::MAX)
    }

    pub fn null() -> Self {
        match unsafe{*ASSET_ID_KIND.get()} {
            Some(AssetIdKind::Index) => unsafe {RawAssetId::from_index_unchecked(u64::MAX)},
            Some(AssetIdKind::Path) => unsafe {RawAssetId::from_path_unchecked(PathBuf::from(NULL_PATH))},
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

    pub fn path_string(&self) -> RisResult<String> {
        ris_error::assert!(unsafe{*ASSET_ID_KIND.get()} == Some(AssetIdKind::Path))?;

        let path = unsafe {self.path()};
        let display = path.display().to_string();
        let replaced = display.replace('\\', "/");

        Ok(replaced)
    }

    // setter

    /// # Safety
    /// 
    /// this function overwrites the path ptr **without** freeing the previous value. this will
    /// result in a leak, so only call this method on uninitialized AssetIds!
    pub unsafe fn set_path(&mut self, p: impl AsRef<Path>) {
        let p = p.as_ref().to_path_buf();
        let ptr = Box::into_raw(Box::new(p));
        self.path = ptr
    }

    // methods
    pub unsafe fn is_null_path(&self) -> bool {
        let path = unsafe {self.path()};
        path.display().to_string() == NULL_PATH
    }
}

//==============================================================================
// Impl AssetId & BinAssetId
//==============================================================================
impl<T> AssetId<T> {
    // constructors

    /// # Safety
    /// 
    /// `T` determines what struct the asset loader allocates and populates. 
    /// to prevent UB, the asset this id is pointing to must absolutely be of 
    /// type `T`.
    pub unsafe fn from_raw(raw: impl AsRef<RawAssetId>) -> Self {
        Self {
            raw: raw.as_ref().clone(),
            boo: PhantomData::default(),
        }
    }

    pub fn null() -> Self {
        let raw = RawAssetId::null();
        unsafe {Self::from_raw(raw)}
    }

    // getters
    pub fn raw(&self) -> &RawAssetId {
        &self.raw
    }

    pub fn raw_mut(&mut self) -> &mut RawAssetId {
        &mut self.raw
    }

    pub unsafe fn index(&self) -> u64 {
        unsafe { self.raw.index }
    }

    pub unsafe fn path(&self) -> &Path {
        unsafe { &(*self.raw.path) }
    }

    pub fn path_string(&self) -> RisResult<String> {
        self.raw.path_string()
    }

    // setter

    /// # Safety
    /// 
    /// see [`RawAssetId::set_path`]
    pub unsafe fn set_path(&mut self, p: impl AsRef<Path>) {
        unsafe {self.raw.set_path(p)};
    }
}

impl BinAssetId {
    // constructors
    pub fn from_raw(raw: impl AsRef<RawAssetId>) -> Self {
        Self {
            raw: raw.as_ref().clone(),
        }
    }

    pub fn null() -> Self {
        let raw = RawAssetId::null();
        unsafe {Self::from_raw(raw)}
    }

    // getters
    pub fn raw(&self) -> &RawAssetId {
        &self.raw
    }

    pub fn raw_mut(&mut self) -> &mut RawAssetId {
        &mut self.raw
    }

    pub unsafe fn index(&self) -> u64 {
        unsafe { self.raw.index }
    }

    pub unsafe fn path(&self) -> &Path {
        unsafe { &(*self.raw.path) }
    }

    pub fn path_string(&self) -> RisResult<String> {
        self.raw.path_string()
    }

    // setter

    /// # Safety
    /// 
    /// see [`RawAssetId::set_path`]
    pub unsafe fn set_path(&mut self, p: impl AsRef<Path>) {
        unsafe {self.raw.set_path(p)};
    }
}
