pub trait IPackageInfo{
    fn new() -> Self;

    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn author(&self) -> &str;
    fn website(&self) -> &str;
}