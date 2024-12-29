use ris_asset::assets::ris_scene;
use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::registry::Registry;
use ris_data::ecs::scene::Scene;
use ris_data::ecs::scene::SceneCreateInfo;
use ris_data::ecs::script_prelude::*;
use ris_error::RisResult;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;

#[derive(Default, Debug)]
struct TestScript {
    payload: Vec<u8>,
}

impl Script for TestScript {
    fn start(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        Ok(())
    }

    fn update(&mut self, _data: ScriptUpdateData) -> RisResult<()> {
        Ok(())
    }

    fn end(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        Ok(())
    }

    fn serialize(&self, f: &mut SceneWriter) -> RisResult<()> {
        //ris_io::write_uint(f, self.payload.len())?;
        //ris_io::write(f, &self.payload)?;
        Ok(())
    }

    fn deserialize(&mut self, f: &mut SceneReader) -> RisResult<()> {
        //let payload_len = ris_io::read_uint(f)?;
        //self.payload = vec![0; payload_len];
        //ris_io::read(f, &mut self.payload)?;
        Ok(())
    }

    fn inspect(&mut self, _data: ScriptInspectData) -> RisResult<()> {
        Ok(())
    }
}

#[test]
fn should_serialize() {
    let mut rng = Rng::new(Seed::new().unwrap());

    let registry = Registry::new(vec![
        Registry::script::<TestScript>().unwrap()
    ]).unwrap();

    let count = 20;
    let mut scene_create_info = SceneCreateInfo::default();
    scene_create_info.static_chunks = 2;
    scene_create_info.static_game_objects_per_chunk = count;
    scene_create_info.registry = Some(registry);
    let scene = Scene::new(scene_create_info).unwrap();

    // the first chunk will be reserved, such that the serializer doesn't to choose to create game
    // objects there
    assert_eq!(scene.reserve_chunk().unwrap(), 0);

    // mark some game objects as alive. this is the worst possible case, where game objects are not
    // orderered and chunk to be loaded into already has some game objects
    let mut to_unmark_0 = Vec::new();
    let mut to_unmark_1 = Vec::new();
    for _ in 0..(count / 2) {
        let index = rng.next_i32_between(0, count as i32 - 1) as usize;
        let mut aref = scene.static_chunks[0].game_objects[index].borrow_mut();
        aref.is_alive = true;
        to_unmark_0.push(index);
    }

    for _ in 0..(count / 2) {
        let index = rng.next_i32_between(0, count as i32 - 1) as usize;
        let mut aref = scene.static_chunks[1].game_objects[index].borrow_mut();
        aref.is_alive = true;
        to_unmark_1.push(index);
    }

    // setup
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

    for index in to_unmark_0 {
        let mut aref = scene.static_chunks[0].game_objects[index].borrow_mut();
        aref.is_alive = false;
    }

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

    // actual code to be tested
    let serialized = ris_scene::serialize(&scene, 0).unwrap();
    ris_scene::load(&scene, &serialized).unwrap();

    // cleanup
    for index in to_unmark_1 {
        let mut aref = scene.static_chunks[1].game_objects[index].borrow_mut();
        aref.is_alive = false;
    }

    // asserts

    // assert that each chunk has the same amount of alive game objects
    let left_count = scene.static_chunks[0].game_objects
        .iter()
        .filter(|x| x.borrow().is_alive)
        .count();
    let right_count = scene.static_chunks[1].game_objects
        .iter()
        .filter(|x| x.borrow().is_alive)
        .count();
    assert_eq!(left_count, right_count);

    for i in 0..count {
        let left: GameObjectHandle = scene.static_chunks[0].game_objects[i].borrow().handle.into();

        if !left.is_alive(&scene) {
            continue;
        }

        let right: GameObjectHandle = scene.static_chunks[1].game_objects
            .iter()
            .find(|x| {
                let g: GameObjectHandle = x.borrow().handle.into();
                g.name(&scene) == left.name(&scene)
            })
            .unwrap()
            .borrow()
            .handle
            .into();

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

        let left_children = left.children(&scene).unwrap();
        let right_children = right.children(&scene).unwrap();

        assert_eq!(left_children.len(), right_children.len());
        for i in 0..left_children.len() {
            let left_child = left_children[i];
            let right_child = right_children[i];

            assert_eq!(
                left_child.name(&scene).unwrap(),
                right_child.name(&scene).unwrap(),
            );
        }
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

    if rng.next_bool() {
        let script = game_object.add_script::<TestScript>(scene)?;
        let byte_count = rng.next_i32_between(0, 100) as usize;
        let bytes = rng.next_bytes(byte_count);
        script.script_mut(scene).unwrap().payload = bytes;
    }

    Ok(())
}
