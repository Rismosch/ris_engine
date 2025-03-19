use std::sync::Arc;

use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::decl::MeshRendererComponentHandle;
use ris_data::ecs::game_object::GetFrom;
use ris_data::ecs::handle::ComponentHandle;
use ris_data::ecs::registry::Registry;
use ris_data::ecs::scene::Scene;
use ris_data::ecs::scene::SceneCreateInfo;

fn scene_create_info() -> SceneCreateInfo {
    let mut info = SceneCreateInfo::empty();
    info.dynamic_game_objects = 8;
    info.mesh_renderer_components = 8;
    info.registry = Some(Arc::new(Registry::new(Vec::new()).unwrap()));
    info
}

#[test]
fn should_add() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let g = GameObjectHandle::new(&scene).unwrap();

    let mesh: MeshRendererComponentHandle = g.add_component(&scene).unwrap().into();

    let index = mesh.scene_id().index;
    let ptr = &scene.mesh_renderer_components[index];
    let mesh_: MeshRendererComponentHandle = ptr.borrow().handle.into();

    assert!(ptr.borrow().is_alive);
    assert_eq!(mesh, mesh_);
}

fn build_scene() -> (
    Scene,
    Vec<GameObjectHandle>,
    Vec<MeshRendererComponentHandle>,
) {
    let scene = Scene::new(scene_create_info()).unwrap();
    let mut game_objects = Vec::new();
    let mut mesh_components = Vec::new();

    for _ in 0..scene.dynamic_game_objects.len() {
        let game_object = GameObjectHandle::new(&scene).unwrap();
        let mesh: MeshRendererComponentHandle = game_object.add_component(&scene).unwrap().into();

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

    let result: Vec<MeshRendererComponentHandle> = game_objects[2]
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

    let actual: Vec<MeshRendererComponentHandle> = game_objects[2]
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

    let actual: Vec<MeshRendererComponentHandle> = game_objects[2]
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

    let actual: Vec<MeshRendererComponentHandle> = game_objects[2]
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

    let actual: Vec<MeshRendererComponentHandle> = game_objects[2]
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

    let actual: Vec<MeshRendererComponentHandle> = game_objects[2]
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
fn should_get_nothing_when_nothing_is_attached() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let _g0 = GameObjectHandle::new(&scene).unwrap();
    let g1 = GameObjectHandle::new(&scene).unwrap();
    let g2 = GameObjectHandle::new(&scene).unwrap();
    let g3 = GameObjectHandle::new(&scene).unwrap();
    let _g4 = GameObjectHandle::new(&scene).unwrap();

    let _mesh: MeshRendererComponentHandle = g2.add_component(&scene).unwrap().into();

    let result_1: Vec<MeshRendererComponentHandle> = g1
        .get_components(&scene, GetFrom::This)
        .unwrap()
        .into_iter()
        .map(|x| x.into())
        .collect();
    let result_2: Vec<MeshRendererComponentHandle> = g1
        .get_components(&scene, GetFrom::ThisAndParents)
        .unwrap()
        .into_iter()
        .map(|x| x.into())
        .collect();
    let result_3: Vec<MeshRendererComponentHandle> = g3
        .get_components(&scene, GetFrom::This)
        .unwrap()
        .into_iter()
        .map(|x| x.into())
        .collect();
    let result_4: Vec<MeshRendererComponentHandle> = g3
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
fn should_get_first_component() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let g = GameObjectHandle::new(&scene).unwrap();
    let m1: MeshRendererComponentHandle = g.add_component(&scene).unwrap().into();
    let m2: MeshRendererComponentHandle = g
        .get_component(&scene, GetFrom::This)
        .unwrap()
        .unwrap()
        .into();

    assert_eq!(m1, m2);
}

#[test]
fn should_detach_component_when_destroyed() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let g = GameObjectHandle::new(&scene).unwrap();
    let m: MeshRendererComponentHandle = g.add_component(&scene).unwrap().into();
    m.destroy(&scene);

    let actual: Vec<MeshRendererComponentHandle> = g
        .get_components(&scene, GetFrom::This)
        .unwrap()
        .into_iter()
        .map(|x| x.into())
        .collect();

    assert!(actual.is_empty());
}

#[test]
fn should_destroy_components_when_game_object_is_destroyed() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let g = GameObjectHandle::new(&scene).unwrap();
    let m: MeshRendererComponentHandle = g.add_component(&scene).unwrap().into();
    g.destroy(&scene);
    assert!(!m.is_alive(&scene));
}
