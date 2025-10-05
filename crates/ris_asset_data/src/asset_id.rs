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
