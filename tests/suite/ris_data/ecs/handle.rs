use ris_data::ecs::game_object::GameObject;
use ris_data::ecs::handle::DynHandle;
use ris_data::ecs::handle::GenericHandle;
use ris_data::ecs::id::SceneId;
use ris_data::ecs::id::GameObjectId;
use ris_data::ecs::id::GameObjectKind;
use ris_data::ecs::script_component::ScriptComponent;

#[test]
fn should_cast_handles() {
    let game_object_id = SceneId::GameObject(GameObjectId{
        kind: GameObjectKind::Movable,
        index: 0,
    });
    let generic_handle = GenericHandle::<GameObject>::new(game_object_id, 0).unwrap();
    let dyn_handle = DynHandle::from(generic_handle);

    let result1 = GenericHandle::<GameObject>::try_from(dyn_handle);
    let result2 = GenericHandle::<ScriptComponent>::try_from(dyn_handle);

    assert!(result1.is_ok());
    assert!(result2.is_err());

    assert_eq!(generic_handle, result1.unwrap());
}
