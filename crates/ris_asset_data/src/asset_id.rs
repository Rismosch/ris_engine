#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssetId {
    Index(usize),
    Path(String),
}
