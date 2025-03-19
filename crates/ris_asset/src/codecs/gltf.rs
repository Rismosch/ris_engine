use ris_error::prelude::*;

use super::json::JsonObject;
use super::json::JsonValue;

// implementations nots:
//
// gltf coordinate system: right handed
// +Y up, +Z forward, -X right
// units for linear distances are meters
// angles are radians
// positive rotation is counterclockwise
//
// trs matrices cannot skew or shear
// 0 scale can be used to enable/disable rendering as an optimization
//
// all buffer data must be little endian

pub struct Gltf {

}

impl Gltf {
    pub fn deserialize(json: impl AsRef<str>, buffers: Vec<Vec<u8>>) -> RisResult<Self> {
        let json = json.as_ref();

        let json_value = JsonValue::deserialize(json)?;
        let JsonValue::Object(json) = json_value else {
            return ris_error::new_result!("gltf json is not an object");
        };

        // asset
        let asset: &JsonObject = json.get("asset").into_ris_error()?;

        let version: &str = asset.get("version").into_ris_error()?;
        let min_version = asset.get::<&str>("minVersion");
        let _generator = asset.get::<&str>("generator");
        let _copyright = asset.get::<&str>("copyright");

        ris_error::assert!(version == "2.0")?;
        if let Some(min_version) = min_version {
            ris_error::assert!(version == min_version)?;
        }

        ris_error::new_result!("reached end")
    }
}
