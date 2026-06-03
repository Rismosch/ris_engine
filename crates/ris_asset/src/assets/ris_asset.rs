use std::ffi::c_void;
use std::mem::MaybeUninit;

use ris_asset_data::AssetId;
use ris_error::prelude::*;

use crate::codecs::json::JsonObject;

pub trait RisAsset : Clone + Send {
    fn from_json(s: &mut MaybeUninit<Self>, json: &JsonObject) -> RisResult<()>;
    fn to_json(&self) -> RisResult<JsonObject>;
    fn all_references_mut(&mut self) -> Vec<&mut AssetId>;
}

pub unsafe fn impl_alloc<T: RisAsset>() -> *mut c_void {
    let uninit = Box::<T>::new_uninit();
    let ptr = Box::leak(uninit);
    ptr.as_mut_ptr() as *mut c_void
}

pub unsafe fn impl_from_json<T: RisAsset>(ptr: *mut c_void, json: &JsonObject) -> RisResult<()>{
    let ptr = ptr.cast::<MaybeUninit<T>>();
    let t = unsafe {&mut *ptr};
    T::from_json(t, json)
}

pub unsafe fn impl_all_references_mut<T: RisAsset + 'static>(ptr: *mut c_void) -> Vec<&'static mut AssetId> {
    let ptr = ptr.cast::<T>();
    let t = unsafe {&mut *ptr};
    t.all_references_mut()
}

pub unsafe fn impl_destructor<T: RisAsset>(ptr: *mut c_void) {
    let ptr = ptr.cast::<T>();
    let _ = unsafe {Box::from_raw(ptr)};
}
