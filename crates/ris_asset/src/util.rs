use ris_asset_data::AssetId;
use ris_error::{RisError, RisResult};

use crate::codecs::json::JsonObject;

pub fn add_alpha_channel(pixels: &[u8]) -> RisResult<Vec<u8>> {
    ris_error::assert!(pixels.len().is_multiple_of(3))?;
    let pixels_rgba_len = (pixels.len() * 4) / 3;
    let mut pixels_rgba = Vec::with_capacity(pixels_rgba_len);

    for chunk in pixels.chunks_exact(3) {
        let r = chunk[0];
        let g = chunk[1];
        let b = chunk[2];
        let a = u8::MAX;

        pixels_rgba.push(r);
        pixels_rgba.push(g);
        pixels_rgba.push(b);
        pixels_rgba.push(a);
    }

    Ok(pixels_rgba)
}

pub trait InitFromJson {
    /// # Safety
    ///
    /// intended to initialize uninitialized memory. calling on initialized
    /// memory may cause leaks or UB even
    unsafe fn init_from_json(&mut self, json: &JsonObject, name: impl AsRef<str>) -> RisResult<()>;
}

impl<T> InitFromJson for AssetId<T> {
    unsafe fn init_from_json(&mut self, json: &JsonObject, name: impl AsRef<str>) -> RisResult<()> {
        let name = name.as_ref();
        let value = json
            .get::<&str>(name)
            .unwrap_or(ris_asset_data::asset_id::NULL_PATH);
        unsafe { self.set_path(value) }
    }
}
