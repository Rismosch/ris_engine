use std::any::TypeId;

use ris_asset::assets::ris_scene;
use ris_data::asset_id::AssetId;
use ris_data::ecs::components::script::DynScriptComponent;
use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::handle::DynComponentHandle;
use ris_data::ecs::id::Component;
use ris_data::ecs::registry::Registry;
use ris_data::ecs::scene::Scene;
use ris_data::ecs::scene::SceneCreateInfo;
use ris_data::ecs::script_prelude::*;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Call {
    Default,
    Start,
    Update,
    End,
    Serialize,
    Deserialize,
    Inspect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TestInner {
    payload: Vec<u8>,
    game_object: GameObjectHandle,
    asset_id: AssetId,
    calls: Vec<Call>,
}

#[derive(Debug)]
struct TestScript {
    inner: TestInner,
}

impl Default for TestScript {
    fn default() -> Self {
        Self {
            inner: TestInner {
                payload: Vec::new(),
                game_object: GameObjectHandle::null(),
                asset_id: AssetId::Index(0),
                calls: vec![Call::Default],
            },
        }
    }
}

impl Script for TestScript {
    fn start(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        self.inner.calls.push(Call::Start);
        Ok(())
    }

    fn update(&mut self, _data: ScriptUpdateData) -> RisResult<()> {
        self.inner.calls.push(Call::Update);
        Ok(())
    }

    fn end(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        self.inner.calls.push(Call::End);
        Ok(())
    }

    fn serialize(&mut self, stream: &mut SceneWriter) -> RisResult<()> {
        self.inner.calls.push(Call::Serialize);
        ris_io::write_uint(stream, self.inner.payload.len())?;
        ris_io::write(stream, &self.inner.payload)?;
        stream.write_game_object(self.inner.game_object)?;
        stream.write_asset_id(self.inner.asset_id.clone())?;
        Ok(())
    }

    fn deserialize(&mut self, stream: &mut SceneReader) -> RisResult<()> {
        self.inner.calls.push(Call::Deserialize);
        let payload_len = ris_io::read_uint(stream)?;
        self.inner.payload = vec![0; payload_len];
        ris_io::read(stream, &mut self.inner.payload)?;
        self.inner.game_object = stream.read_game_object()?;
        self.inner.asset_id = stream.read_asset_id()?;
        Ok(())
    }

    fn inspect(&mut self, _data: ScriptInspectData) -> RisResult<()> {
        self.inner.calls.push(Call::Inspect);
        Ok(())
    }
}

#[test]
fn should_serialize() {
    let mut rng = Rng::new(Seed::new().unwrap());

    let registry = Registry::new(vec![Registry::script::<TestScript>().unwrap()]).unwrap();

    let count = 20;
    let scene_create_info = SceneCreateInfo {
        static_chunks: 2,
        game_objects_per_static_chunk: count,
        registry: Some(registry),
        ..Default::default()
    };
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

    let gs = [g0, g1, g2, g3, g4, g5, g6, g7, g8, g9];
    fill_data(&scene, g0, &mut rng, "zero", &gs).unwrap();
    fill_data(&scene, g1, &mut rng, "one", &gs).unwrap();
    fill_data(&scene, g2, &mut rng, "two", &gs).unwrap();
    fill_data(&scene, g3, &mut rng, "three", &gs).unwrap();
    fill_data(&scene, g4, &mut rng, "four", &gs).unwrap();
    fill_data(&scene, g5, &mut rng, "five", &gs).unwrap();
    fill_data(&scene, g6, &mut rng, "six", &gs).unwrap();
    fill_data(&scene, g7, &mut rng, "seven", &gs).unwrap();
    fill_data(&scene, g8, &mut rng, "eight", &gs).unwrap();
    fill_data(&scene, g9, &mut rng, "nine", &gs).unwrap();

    // actual code to be tested
    let serialized = ris_scene::serialize(&scene, Some(0)).unwrap();
    ris_scene::load(&scene, &serialized).unwrap();

    //// debugging
    //{
    //    let test_dir = ris_util::prep_test_dir!();
    //    let path = std::path::PathBuf::from(test_dir).join("test.ris_scene");
    //    println!("path {:?}", path);
    //    let mut file = std::fs::File::create(path).unwrap();
    //    ris_io::write(&mut file, &serialized).unwrap();
    //}

    // cleanup
    for index in to_unmark_1 {
        let mut aref = scene.static_chunks[1].game_objects[index].borrow_mut();
        aref.is_alive = false;
    }

    // asserts
    let left_count = scene.static_chunks[0]
        .game_objects
        .iter()
        .filter(|x| x.borrow().is_alive)
        .count();
    let right_count = scene.static_chunks[1]
        .game_objects
        .iter()
        .filter(|x| x.borrow().is_alive)
        .count();
    assert_eq!(left_count, right_count);

    for i in 0..count {
        let left: GameObjectHandle = scene.static_chunks[0].game_objects[i]
            .borrow()
            .handle
            .into();

        if !left.is_alive(&scene) {
            continue;
        }

        let right: GameObjectHandle = scene.static_chunks[1]
            .game_objects
            .iter()
            .find(|x| {
                let g: GameObjectHandle = x.borrow().handle.into();
                g.name(&scene) == left.name(&scene)
            })
            .unwrap()
            .borrow()
            .handle
            .into();

        assert_eq!(left.name(&scene).unwrap(), right.name(&scene).unwrap(),);
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

        let left_components = left.components(&scene).unwrap();
        let right_components = right.components(&scene).unwrap();
        assert_eq!(left_components.len(), right_components.len());

        for i in 0..left_components.len() {
            let left_inner = get_inner(&scene, left_components[i]).unwrap();
            let right_inner = get_inner(&scene, right_components[i]).unwrap();

            assert_eq!(left_inner.payload, right_inner.payload);

            assert_eq!(
                left_inner.calls,
                vec![Call::Default, Call::Start, Call::Serialize,]
            );

            assert_eq!(
                right_inner.calls,
                vec![Call::Default, Call::Deserialize, Call::Start,]
            );

            assert_eq!(
                left_inner.game_object.name(&scene).unwrap(),
                right_inner.game_object.name(&scene).unwrap(),
            );

            assert_eq!(left_inner.asset_id, right_inner.asset_id,);
        }
    }
}

fn fill_data(
    scene: &Scene,
    game_object: GameObjectHandle,
    rng: &mut Rng,
    name: impl AsRef<str>,
    game_objects: &[GameObjectHandle],
) -> RisResult<()> {
    let is_active = rng.next_bool();
    let position = rng.next_pos_3();
    let rotation = rng.next_rot();
    let scale = rng.next_f32();

    game_object.set_name(scene, name.as_ref()).unwrap();
    game_object.set_active(scene, is_active).unwrap();
    game_object.set_local_position(scene, position).unwrap();
    game_object.set_local_rotation(scene, rotation).unwrap();
    game_object.set_local_scale(scene, scale).unwrap();

    if rng.next_bool() {
        let script = game_object.add_script::<TestScript>(scene)?;
        let byte_count = rng.next_i32_between(0, 100) as usize;
        let bytes = rng.next_bytes(byte_count);
        let mut ref_mut = script.script_mut(scene).unwrap();
        ref_mut.inner.payload = bytes;
        ref_mut.inner.game_object = *rng.next_in(game_objects);
        ref_mut.inner.asset_id = AssetId::Path(format!("some path for {}", name.as_ref()));
    }

    Ok(())
}

fn get_inner(scene: &Scene, handle: DynComponentHandle) -> RisResult<TestInner> {
    assert_eq!(handle.type_id(), TypeId::of::<DynScriptComponent>());

    let inner = scene
        .deref_mut_component(handle, |x| {
            let dyn_script_component: &mut DynScriptComponent =
                unsafe { &mut *(x as *mut dyn Component as *mut DynScriptComponent) };

            let script_type_id = dyn_script_component.type_id().unwrap();
            assert_eq!(script_type_id, TypeId::of::<TestScript>());

            let boxed = dyn_script_component.script_mut().unwrap();
            let script = boxed.as_ref();
            let test: &TestScript = unsafe { &*(script as *const dyn Script as *const TestScript) };
            test.inner.clone()
        })
        .unwrap();

    Ok(inner)
}
