use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::game_object::GetFrom;
use ris_data::ecs::id::GameObjectKind;
use ris_data::ecs::scene::Scene;
use ris_data::ecs::scene::SceneCreateInfo;
use ris_data::ecs::script::prelude::*;

const SCENE_CREATE_INFO: SceneCreateInfo = SceneCreateInfo {
    movable_game_objects: 5,
    static_chunks: 0,
    static_game_objects_per_chunk: 0,
    mesh_components: 0,
    script_components: 5,
};

#[derive(Debug)]
struct TestScriptString {
    value: String,
}

#[derive(Debug)]
struct TestScriptISize {
    value: isize,
}

impl Script for TestScriptString {
    fn id() -> Sid {
        ris_debug::fsid!()
    }

    fn start(_data: ScriptStartData) -> RisResult<Self> {
        Ok(Self {
            value: String::new(),
        })
    }

    fn update(&mut self, _data: ScriptUpdateData) -> RisResult<()> {
        self.value.push_str("\nupdate");
        Ok(())
    }

    fn end(&mut self, _data: ScriptEndData) -> RisResult<()> {
        self.value.push_str("\nend");
        Ok(())
    }
}

impl Script for TestScriptISize {
    fn id() -> Sid {
        ris_debug::fsid!()
    }

    fn start(_data: ScriptStartData) -> RisResult<Self> {
        Ok(Self { value: 0 })
    }

    fn update(&mut self, _data: ScriptUpdateData) -> RisResult<()> {
        self.value += 1;
        Ok(())
    }

    fn end(&mut self, _data: ScriptEndData) -> RisResult<()> {
        self.value *= -1;
        Ok(())
    }
}

#[test]
fn should_add_script() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let _script_string = g.add_script::<TestScriptString>(&scene).unwrap();
    let _script_isize = g.add_script::<TestScriptISize>(&scene).unwrap();
}

#[test]
fn should_deref_handle() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let script = g.add_script::<TestScriptISize>(&scene).unwrap();

    let value_1 = script.script(&scene).unwrap().value;
    script.script_mut(&scene).unwrap().value = 42;
    let value_2 = script.script(&scene).unwrap().value;
    script.destroy(&scene);

    assert_eq!(value_1, 0);
    assert_eq!(value_2, 42);
}

#[test]
fn should_not_deref_handle_when_script_is_destroyed() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let script = g.add_script::<TestScriptISize>(&scene).unwrap();

    script.destroy(&scene);
    let reference = script.script_mut(&scene);

    assert!(reference.is_err());
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn should_panic_when_deref_while_reference_exists() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let script = g.add_script::<TestScriptISize>(&scene).unwrap();

    let _reference1 = script.script(&scene);
    let _reference2 = script.script_mut(&scene); // panics
}

#[test]
fn should_allow_multiple_references() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let script = g.add_script::<TestScriptISize>(&scene).unwrap();

    let _reference1 = script.script(&scene);
    let _reference2 = script.script(&scene);
    let _reference3 = script.script(&scene);
}

#[test]
fn should_get_scripts() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let script1 = g.add_script::<TestScriptISize>(&scene).unwrap();
    let script2 = g.add_script::<TestScriptString>(&scene).unwrap();
    let script3 = g.add_script::<TestScriptISize>(&scene).unwrap();
    let script4 = g.add_script::<TestScriptString>(&scene).unwrap();
    let script5 = g.add_script::<TestScriptISize>(&scene).unwrap();

    script1.script_mut(&scene).unwrap().value = 1;
    script2.script_mut(&scene).unwrap().value = 2.to_string();
    script3.script_mut(&scene).unwrap().value = 3;
    script4.script_mut(&scene).unwrap().value = 4.to_string();
    script5.script_mut(&scene).unwrap().value = 5;

    let scripts_isize = g
        .get_scripts::<TestScriptISize>(&scene, GetFrom::This)
        .unwrap();
    let scripts_string = g
        .get_scripts::<TestScriptString>(&scene, GetFrom::This)
        .unwrap();

    assert_eq!(scripts_isize.len(), 3);
    assert_eq!(scripts_isize[0].script(&scene).unwrap().value, 1);
    assert_eq!(scripts_isize[1].script(&scene).unwrap().value, 3);
    assert_eq!(scripts_isize[2].script(&scene).unwrap().value, 5);

    assert_eq!(scripts_string.len(), 2);
    assert_eq!(scripts_string[0].script(&scene).unwrap().value, "2");
    assert_eq!(scripts_string[1].script(&scene).unwrap().value, "4");
}

#[test]
fn should_get_first_script() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let script1 = g.add_script::<TestScriptISize>(&scene).unwrap();
    let script2 = g.add_script::<TestScriptISize>(&scene).unwrap();
    let script3 = g.add_script::<TestScriptISize>(&scene).unwrap();

    script1.script_mut(&scene).unwrap().value = 1;
    script2.script_mut(&scene).unwrap().value = 2;
    script3.script_mut(&scene).unwrap().value = 3;

    let script1 = g
        .get_script::<TestScriptISize>(&scene, GetFrom::This)
        .unwrap()
        .unwrap();
    let script2 = g
        .get_script::<TestScriptString>(&scene, GetFrom::This)
        .unwrap();

    assert_eq!(script1.script(&scene).unwrap().value, 1);
    assert!(script2.is_none());
}
