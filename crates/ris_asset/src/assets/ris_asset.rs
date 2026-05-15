use ris_data::ris_yaml::RisYaml;

pub trait RisAsset : Clone + Copy + Send + Sync {
    fn from_yaml(yaml: &RisYaml) -> Self;
    fn to_yaml(&self) -> RisYaml;
}
