use std::marker::PhantomData;
use std::path::Path;
use std::path::PathBuf;

use ris_error::prelude::*;

use crate::codecs::json::JsonObject;

pub const NULL_PATH: &str = "NULL";
pub const NULL_INDEX: u64 = u64::MAX;

//==============================================================================
// Structs
//==============================================================================

// note to self: this must be a union, because asset compiler constructs
// RawAssetIds of both types. the same memory must be used for this
#[repr(C)]
pub union RawAssetId {
    pub path: *mut PathBuf, // used in debug
    pub index: u64,         // used in release
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetId<T> {
    raw: RawAssetId,
    boo: PhantomData<T>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinAssetId(AssetId<Box<[u8]>>);

//==============================================================================
// Traits
//==============================================================================

// drop
impl Drop for RawAssetId {
    fn drop(&mut self) {
        if cfg!(debug_assertions) {
            unsafe { free_path(self.path) };
        }
    }
}

// debug
impl std::fmt::Debug for RawAssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if cfg!(debug_assertions) {
            let value = unsafe { self.path_string() };
            write!(f, "RawAssetId {{ path: {:?} }}", value)
        } else {
            let value = unsafe { self.index };
            write!(f, "RawAssetId {{ index: {:#x} }}", value)
        }
    }
}

// clone
impl Clone for RawAssetId {
    fn clone(&self) -> Self {
        if cfg!(debug_assertions) {
            unsafe { Self::from_path(&*self.path) }
        } else {
            unsafe { Self::from_index(self.index) }
        }
    }
}

// eq
impl PartialEq for RawAssetId {
    fn eq(&self, other: &Self) -> bool {
        if cfg!(debug_assertions) {
            unsafe { *self.path == *other.path }
        } else {
            unsafe { self.index == other.index }
        }
    }
}

impl Eq for RawAssetId {}

// as ref
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
    fn as_ref(&self) -> &BinAssetId {
        self
    }
}

// send
unsafe impl Send for RawAssetId {}
unsafe impl<T> Send for AssetId<T> {}

//==============================================================================
// Impl RawAssetId
//==============================================================================
impl RawAssetId {
    /// # Safety
    ///
    /// only construct in debug mode, or a leak on drop will occur
    pub unsafe fn from_path(v: impl AsRef<Path>) -> Self {
        Self {
            path: alloc_path(v),
        }
    }

    /// # Safety
    ///
    /// dropping in debug builds causes UB. so only construct in release builds
    /// or dont drop
    pub unsafe fn from_index(v: u64) -> Self {
        Self { index: v }
    }

    /// # Safety
    ///
    /// See [`Self::from_path`].
    pub unsafe fn null_path() -> Self {
        unsafe { Self::from_path(NULL_PATH) }
    }

    /// # Safety
    ///
    /// See [`Self::from_index`].
    pub unsafe fn null_index() -> Self {
        unsafe { Self::from_index(NULL_INDEX) }
    }

    /// # Safety
    ///
    /// causes UB when this id does not store a path
    pub unsafe fn is_null_path(&self) -> bool {
        let lhs = unsafe { &*self.path };
        let rhs = Path::new(NULL_PATH);
        *lhs == *rhs
    }

    /// # Safety
    ///
    /// causes UB when this id does not store an index
    pub unsafe fn is_null_index(&self) -> bool {
        let lhs = unsafe { self.index };
        let rhs = NULL_INDEX;
        lhs == rhs
    }
}

impl RawAssetId {
    /// # Safety
    ///
    /// causes UB when this id does not store a path
    pub unsafe fn path_string(&self) -> String {
        let path = unsafe { &*self.path };
        let display = path.display().to_string();
        let replaced = display.replace('\\', "/");
        replaced
    }

    /// # Safety
    ///
    /// does not drop the stored path. only call on uninitialized ids. otherwise
    /// the previous path will be leaked
    pub unsafe fn set_path(&mut self, path: impl AsRef<Path>) {
        self.path = alloc_path(path);
    }
}

fn alloc_path(p: impl AsRef<Path>) -> *mut PathBuf {
    let buf = p.as_ref().to_path_buf();
    let boxed = Box::new(buf);
    let ptr = Box::leak(boxed) as *mut PathBuf;
    //println!("alloc {:?}", ptr);
    ptr
}

/// # Safety
///
/// see [`Box::from_raw`]
unsafe fn free_path(ptr: *mut PathBuf) {
    //println!("free  {:?}", ptr);
    if ptr.is_null() {
        return;
    }

    let _ = unsafe { Box::from_raw(ptr) };
}

//==============================================================================
// Impl AssetId
//==============================================================================
impl<T> AssetId<T> {
    // constructors
    pub fn null() -> Self {
        unsafe {
            let raw = if cfg!(debug_assertions) {
                RawAssetId::null_path()
            } else {
                RawAssetId::null_index()
            };

            Self::from_raw(raw)
        }
    }

    /// # Safety
    ///
    /// `raw` must be properly constructed and the asset it poins to must be of
    /// type `T`. improper usage leads to UB on asset loading
    pub unsafe fn from_raw(raw: impl AsRef<RawAssetId>) -> Self {
        Self {
            raw: raw.as_ref().clone(),
            boo: PhantomData::default(),
        }
    }

    /// # Safety
    ///
    /// intended to initialize uninitialized memory. calling on initialized
    /// memory may cause leaks or UB even
    pub unsafe fn init_from_json(&mut self, json: &JsonObject, name: impl AsRef<str>) -> RisResult<()> {
        let name = name.as_ref();
        let value = json
            .get::<&str>(name)
            .unwrap_or(NULL_PATH);
        unsafe { self.set_path(value) }
    }

    // get/set
    pub fn raw(&self) -> &RawAssetId {
        &self.raw
    }

    /// # Safety
    ///
    /// this function allows modification of the underlying raw id. this must be
    /// used with caution. improper use easily leads to incorrectly configured
    /// AssetIds, creating UB both when dropping the id and when loading assets
    /// with it.
    pub unsafe fn raw_mut(&mut self) -> &mut RawAssetId {
        &mut self.raw
    }

    pub fn path_string(&self) -> RisResult<String> {
        if cfg!(debug_assertions) {
            Ok(unsafe { self.raw.path_string() })
        } else {
            ris_error::new!("not available")
        }
    }

    /// # Safety
    ///
    /// does not drop the stored path. only call on uninitialized ids. otherwise
    /// the previous path will be leaked
    pub unsafe fn set_path(&mut self, path: impl AsRef<Path>) -> RisResult<()> {
        if cfg!(debug_assertions) {
            unsafe { self.raw.set_path(path) };
            Ok(())
        } else {
            ris_error::new!("not available")
        }
    }
}

impl BinAssetId {
    // constructors
    pub fn null() -> Self {
        Self(AssetId::null())
    }

    /// # Safety
    ///
    /// see [`AssetId::from_raw`]
    pub unsafe fn from_raw(raw: impl AsRef<RawAssetId>) -> Self {
        unsafe {Self(AssetId::from_raw(raw))}
    }

    /// # Safety
    ///
    /// see [`AssetId::init_from_json`]
    pub unsafe fn init_from_json(&mut self, json: &JsonObject, name: impl AsRef<str>) -> RisResult<()> {
        unsafe {self.0.init_from_json(json, name)}
    }

    // get/set
    pub fn raw(&self) -> &RawAssetId {
        &self.0.raw
    }

    /// # Safety
    ///
    /// see [`AssetId::raw_mut`]
    pub unsafe fn raw_mut(&mut self) -> &mut RawAssetId {
        &mut self.0.raw
    }

    pub fn path_string(&self) -> RisResult<String> {
        self.0.path_string()
    }

    /// # Safety
    ///
    /// see [`AssetId::set_path`]
    pub unsafe fn set_path(&mut self, path: impl AsRef<Path>) -> RisResult<()> {
        unsafe {self.0.set_path(path)}
    }

}
