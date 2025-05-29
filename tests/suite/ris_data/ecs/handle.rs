use ris_data::ecs::components::script_component::DynScriptComponent;
use ris_data::ecs::game_object::GameObject;
use ris_data::ecs::handle::DynHandle;
use ris_data::ecs::handle::GenericHandle;
use ris_data::ecs::id::SceneId;
use ris_data::ecs::id::SceneKind;

#[test]
fn should_cast_handles() {
    let id = SceneId {
        kind: SceneKind::DynamicGameObject,
        index: 42,
    };
    let generic_handle = GenericHandle::<GameObject>::new(id, 0).unwrap();
    let dyn_handle = DynHandle::from(generic_handle);

    let result1 = GenericHandle::<GameObject>::from_dyn(dyn_handle);
    let result2 = GenericHandle::<DynScriptComponent>::from_dyn(dyn_handle);

    assert!(result1.is_ok());
    assert!(result2.is_err());

    assert_eq!(generic_handle, result1.unwrap());
}
