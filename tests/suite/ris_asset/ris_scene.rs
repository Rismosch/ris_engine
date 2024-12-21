use ris_asset::assets::ris_scene;
use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::scene::Scene;
use ris_data::ecs::scene::SceneCreateInfo;
use ris_error::RisResult;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;

#[test]
fn should_serialize() {
    let mut rng = Rng::new(Seed::new().unwrap());

    let mut scene_create_info = SceneCreateInfo::default();
    scene_create_info.static_chunks = 2;
    scene_create_info.static_game_objects_per_chunk = 10;
    let scene = Scene::new(scene_create_info).unwrap();
    let g0 = GameObjectHandle::new_static(&scene, 0).unwrap();
    let g1 = GameObjectHandle::new_static(&scene, 0).unwrap();
    let g2 = GameObjectHandle::new_static(&scene, 0).unwrap();
    let g3 = GameObjectHandle::new_static(&scene, 0).unwrap();
    let g4 = GameObjectHandle::new_static(&scene, 0).unwrap();
    let g5 = GameObjectHandle::new_static(&scene, 0).unwrap();
    let g6 = GameObjectHandle::new_static(&scene, 0).unwrap();
    let g7 = GameObjectHandle::new_static(&scene, 0).unwrap();
    let g8 = GameObjectHandle::new_static(&scene, 0).unwrap();
    let g9 = GameObjectHandle::new_static(&scene, 0).unwrap();

    // -g0
    //   -g1
    //     -g2
    //   -g3
    //     -g4
    // -g5
    //   -g6
    //   -g7
    //     -g8
    //       -g9
    //

    g1.set_parent(&scene, Some(g0), 0, false).unwrap();
    g2.set_parent(&scene, Some(g1), 0, false).unwrap();
    g3.set_parent(&scene, Some(g0), 1, false).unwrap();
    g4.set_parent(&scene, Some(g3), 0, false).unwrap();
    g6.set_parent(&scene, Some(g5), 0, false).unwrap();
    g7.set_parent(&scene, Some(g5), 1, false).unwrap();
    g8.set_parent(&scene, Some(g7), 0, false).unwrap();
    g9.set_parent(&scene, Some(g8), 0, false).unwrap();

    fill_data(&scene, g0, &mut rng, "zero").unwrap();
    fill_data(&scene, g1, &mut rng, "one").unwrap();
    fill_data(&scene, g2, &mut rng, "two").unwrap();
    fill_data(&scene, g3, &mut rng, "three").unwrap();
    fill_data(&scene, g4, &mut rng, "four").unwrap();
    fill_data(&scene, g5, &mut rng, "five").unwrap();
    fill_data(&scene, g6, &mut rng, "six").unwrap();
    fill_data(&scene, g7, &mut rng, "seven").unwrap();
    fill_data(&scene, g8, &mut rng, "eight").unwrap();
    fill_data(&scene, g9, &mut rng, "nine").unwrap();

    let serialized = ris_scene::serialize(&scene, 0).unwrap();
    ris_scene::load(&scene, &serialized).unwrap();

    for i in 0..scene_create_info.static_game_objects_per_chunk {
        let left: GameObjectHandle = scene.static_chunks[0].game_objects[i].borrow().handle.into();
        let right: GameObjectHandle = scene.static_chunks[1].game_objects[i].borrow().handle.into();

        assert_eq!(
            left.name(&scene).unwrap(),
            right.name(&scene).unwrap(),
        );
        assert_eq!(
            left.is_active(&scene).unwrap(),
            right.is_active(&scene).unwrap(),
        );
        ris_util::assert_vec3_eq!(
            left.local_position(&scene).unwrap(),
            right.local_position(&scene).unwrap(),
        );
        ris_util::assert_quat_eq!(
            left.local_rotation(&scene).unwrap(),
            right.local_rotation(&scene).unwrap(),
        );
        ris_util::assert_feq!(
            left.local_scale(&scene).unwrap(),
            right.local_scale(&scene).unwrap(),
        );
    }

    panic!("reached end");
}

fn fill_data(
    scene: &Scene,
    game_object: GameObjectHandle,
    rng: &mut Rng,
    name: impl AsRef<str>,
) -> RisResult<()> {
    let is_active = rng.next_bool();
    let position = rng.next_pos_3();
    let rotation = rng.next_rot();
    let scale = rng.next_f32();

    game_object.set_name(scene, name).unwrap();
    game_object.set_active(scene, is_active).unwrap();
    game_object.set_local_position(scene, position).unwrap();
    game_object.set_local_rotation(scene, rotation).unwrap();
    game_object.set_local_scale(scene, scale).unwrap();

    Ok(())
}
