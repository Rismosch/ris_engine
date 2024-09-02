use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::decl::MeshComponentHandle;
use ris_data::ecs::game_object::GetFrom;
use ris_data::ecs::id::GameObjectKind;
use ris_data::ecs::scene::Scene;
use ris_data::ecs::scene::SceneCreateInfo;

const SCENE_CREATE_INFO: SceneCreateInfo = SceneCreateInfo {
    movable_game_objects: 8,
    static_chunks: 0,
    static_game_objects_per_chunk: 0,
    mesh_components: 8,
    script_components: 0,
};

#[test]
fn should_add() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    let mesh: MeshComponentHandle = g.add_component(&scene).unwrap().into();

    let index = mesh.scene_id().index;
    let ptr = &scene.mesh_components[index];
    let mesh_: MeshComponentHandle = ptr.borrow().handle.into();

    assert!(ptr.borrow().is_alive);
    assert_eq!(mesh, mesh_);
}

fn build_scene() -> (Scene, Vec<GameObjectHandle>, Vec<MeshComponentHandle>) {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let mut game_objects = Vec::new();
    let mut mesh_components = Vec::new();

    for _ in 0..scene.movable_game_objects.len() {
        let game_object = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
        let mesh: MeshComponentHandle = game_object.add_component(&scene).unwrap().into();

        game_objects.push(game_object);
        mesh_components.push(mesh);
    }

    game_objects[1]
        .set_parent(&scene, Some(game_objects[0]), 0, true)
        .unwrap();
    game_objects[2]
        .set_parent(&scene, Some(game_objects[1]), 0, true)
        .unwrap();
    game_objects[3]
        .set_parent(&scene, Some(game_objects[2]), 0, true)
        .unwrap();
    game_objects[4]
        .set_parent(&scene, Some(game_objects[2]), 0, true)
        .unwrap();
    game_objects[5]
        .set_parent(&scene, Some(game_objects[3]), 0, true)
        .unwrap();
    game_objects[6]
        .set_parent(&scene, Some(game_objects[4]), 0, true)
        .unwrap();
    game_objects[7]
        .set_parent(&scene, Some(game_objects[4]), 0, true)
        .unwrap();

    (scene, game_objects, mesh_components)
}

#[test]
fn should_get_from_self() {
    let (scene, game_objects, mesh_components) = build_scene();

    let result: Vec<MeshComponentHandle> = game_objects[2]
        .get_components(&scene, GetFrom::This)
        .unwrap()
        .into_iter()
        .map(|x| x.into())
        .collect();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0], mesh_components[2]);
}

#[test]
fn should_get_from_children() {
    let (scene, game_objects, mesh_components) = build_scene();

    let actual: Vec<MeshComponentHandle> = game_objects[2]
        .get_components(&scene, GetFrom::ThisAndChildren)
        .unwrap()
        .into_iter()
        .map(|x| x.into())
        .collect();

    let expected = vec![
        mesh_components[3],
        mesh_components[4],
        mesh_components[5],
        mesh_components[6],
        mesh_components[7],
    ];

    for expected in expected {
        assert!(actual.iter().any(|&x| x == expected));
    }
}

#[test]
fn should_get_from_parent() {
    let (scene, game_objects, mesh_components) = build_scene();

    let actual: Vec<MeshComponentHandle> = game_objects[2]
        .get_components(&scene, GetFrom::ThisAndParents)
        .unwrap()
        .into_iter()
        .map(|x| x.into())
        .collect();

    let expected = vec![mesh_components[0], mesh_components[1]];

    for expected in expected {
        assert!(actual.iter().any(|&x| x == expected));
    }
}

#[test]
fn should_get_from_self_and_children() {
    let (scene, game_objects, mesh_components) = build_scene();

    let actual: Vec<MeshComponentHandle> = game_objects[2]
        .get_components(&scene, GetFrom::ThisAndChildren)
        .unwrap()
        .into_iter()
        .map(|x| x.into())
        .collect();

    let expected = vec![
        mesh_components[2],
        mesh_components[3],
        mesh_components[4],
        mesh_components[5],
        mesh_components[6],
        mesh_components[7],
    ];

    for expected in expected {
        assert!(actual.iter().any(|&x| x == expected));
    }
}

#[test]
fn should_get_from_self_and_parent() {
    let (scene, game_objects, mesh_components) = build_scene();

    let actual: Vec<MeshComponentHandle> = game_objects[2]
        .get_components(&scene, GetFrom::ThisAndParents)
        .unwrap()
        .into_iter()
        .map(|x| x.into())
        .collect();

    let expected = vec![mesh_components[0], mesh_components[1], mesh_components[2]];

    for expected in expected {
        assert!(actual.iter().any(|&x| x == expected));
    }
}

#[test]
fn should_get_from_all() {
    let (scene, game_objects, mesh_components) = build_scene();

    let actual: Vec<MeshComponentHandle> = game_objects[2]
        .get_components(&scene, GetFrom::All)
        .unwrap()
        .into_iter()
        .map(|x| x.into())
        .collect();

    let expected = mesh_components.clone();

    for expected in expected {
        assert!(actual.iter().any(|&x| x == expected));
    }
}

#[test]
fn should_nothing_when_nothing_is_attached() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let _g0 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g1 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g2 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g3 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let _g4 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    let _mesh: MeshComponentHandle = g2.add_component(&scene).unwrap().into();

    let result_1: Vec<MeshComponentHandle> = g1
        .get_components(&scene, GetFrom::This)
        .unwrap()
        .into_iter()
        .map(|x| x.into())
        .collect();
    let result_2: Vec<MeshComponentHandle> = g1
        .get_components(&scene, GetFrom::ThisAndParents)
        .unwrap()
        .into_iter()
        .map(|x| x.into())
        .collect();
    let result_3: Vec<MeshComponentHandle> = g3
        .get_components(&scene, GetFrom::This)
        .unwrap()
        .into_iter()
        .map(|x| x.into())
        .collect();
    let result_4: Vec<MeshComponentHandle> = g3
        .get_components(&scene, GetFrom::ThisAndChildren)
        .unwrap()
        .into_iter()
        .map(|x| x.into())
        .collect();

    assert!(result_1.is_empty());
    assert!(result_2.is_empty());
    assert!(result_3.is_empty());
    assert!(result_4.is_empty());
}

#[test]
fn should_detach_component_when_destroyed() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let m: MeshComponentHandle = g.add_component(&scene).unwrap().into();
    m.destroy(&scene);

    let actual: Vec<MeshComponentHandle> = g
        .get_components(&scene, GetFrom::This)
        .unwrap()
        .into_iter()
        .map(|x| x.into())
        .collect();

    assert!(actual.is_empty());
}

#[test]
fn should_destroy_components_when_game_object_is_destroyed() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let m: MeshComponentHandle = g.add_component(&scene).unwrap().into();
    g.destroy(&scene);
    assert!(!m.is_alive(&scene));
}
