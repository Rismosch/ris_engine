use ris_error::RisResult;

pub trait ISerializable {
    fn serialize(&self) -> RisResult<Vec<u8>>;
    fn deserialize(bytes: &[u8]) -> RisResult<Self> where Self: Sized;
}
