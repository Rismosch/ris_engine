use ris_ecs::ecs::Ecs;

#[test]
fn should_allocate() {
    let capacity = 42;
    let ecs = Ecs::new(capacity);
    assert!(ecs.len() == capacity);
    assert!(ecs.transform_components.len() == capacity);
}
