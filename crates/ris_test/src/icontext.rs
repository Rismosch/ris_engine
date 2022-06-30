pub trait IContext {
    fn setup() -> Self;
    fn teardown(&mut self);
}