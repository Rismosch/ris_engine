use ris_error::prelude::*;
use ris_io::FatPtr;
use crate::codecs::json::JsonObject;

//==============================================================================
// FatPtr
//==============================================================================
pub const ADDR: &str = "addr";
pub const LEN: &str = "len";

pub fn json_to_fatptr(json: &JsonObject) -> RisResult<FatPtr> {
    let addr = json.get::<u64>(ADDR).ris_expect("addr to exist")?;
    let len = json.get::<u64>(LEN).ris_expect("len to exist")?;
    Ok(FatPtr { addr, len })
}

pub fn fatptr_to_json(ptr: FatPtr) -> JsonObject {
    let mut json = JsonObject::default();
    json.push(ADDR, ptr.addr);
    json.push(LEN, ptr.len);
    json
}

