use std::sync::Arc;

use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::game_object::GetFrom;
use ris_data::ecs::registry::Registry;
use ris_data::ecs::scene::Scene;
use ris_data::ecs::scene::SceneCreateInfo;
use ris_data::ecs::script_prelude::*;

fn scene_create_info() -> SceneCreateInfo {
    let mut info = SceneCreateInfo::empty();
    info.dynamic_game_objects = 5;
    info.script_components = 5;
    info.registry = Some(
        Arc::new(Registry::new(vec![
            Registry::script::<TestScriptString>().unwrap(),
            Registry::script::<TestScriptISize>().unwrap(),
        ])
        .unwrap())
    );
    info
}

#[derive(Debug, Default)]
struct TestScriptString {
    value: String,
}

#[derive(Debug, Default)]
struct TestScriptISize {
    value: isize,
}

impl Script for TestScriptString {
    fn start(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        Ok(())
    }

    fn update(&mut self, _data: ScriptUpdateData) -> RisResult<()> {
        self.value.push_str("\nupdate");
        Ok(())
    }

    fn end(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        self.value.push_str("\nend");
        Ok(())
    }

    fn serialize(&mut self, _stream: &mut SceneWriter) -> RisResult<()> {
        ris_error::new_result!("not implemented")
    }

    fn deserialize(&mut self, _stream: &mut SceneReader) -> RisResult<()> {
        ris_error::new_result!("not implemented")
    }

    fn inspect(&mut self, _data: ScriptInspectData) -> RisResult<()> {
        ris_error::new_result!("not implementd")
    }
}

impl Script for TestScriptISize {
    fn start(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        Ok(())
    }

    fn update(&mut self, _data: ScriptUpdateData) -> RisResult<()> {
        self.value += 1;
        Ok(())
    }

    fn end(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        self.value *= -1;
        Ok(())
    }

    fn serialize(&mut self, _stream: &mut SceneWriter) -> RisResult<()> {
        ris_error::new_result!("not implemented")
    }

    fn deserialize(&mut self, _stream: &mut SceneReader) -> RisResult<()> {
        ris_error::new_result!("not implemented")
    }

    fn inspect(&mut self, _data: ScriptInspectData) -> RisResult<()> {
        ris_error::new_result!("not implementd")
    }
}

#[test]
fn should_add_script() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let g = GameObjectHandle::new(&scene).unwrap();
    let _script_string = g.add_script::<TestScriptString>(&scene).unwrap();
    let _script_isize = g.add_script::<TestScriptISize>(&scene).unwrap();
}

#[test]
fn should_deref_handle() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let g = GameObjectHandle::new(&scene).unwrap();
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
    let scene = Scene::new(scene_create_info()).unwrap();
    let g = GameObjectHandle::new(&scene).unwrap();
    let script = g.add_script::<TestScriptISize>(&scene).unwrap();

    script.destroy(&scene);
    let reference = script.script_mut(&scene);

    assert!(reference.is_err());
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn should_panic_when_deref_while_reference_exists() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let g = GameObjectHandle::new(&scene).unwrap();
    let script = g.add_script::<TestScriptISize>(&scene).unwrap();

    let _reference1 = script.script(&scene);
    let _reference2 = script.script_mut(&scene); // panics
}

#[test]
fn should_allow_multiple_references() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let g = GameObjectHandle::new(&scene).unwrap();
    let script = g.add_script::<TestScriptISize>(&scene).unwrap();

    let _reference1 = script.script(&scene);
    let _reference2 = script.script(&scene);
    let _reference3 = script.script(&scene);
}

#[test]
fn should_get_scripts() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let g = GameObjectHandle::new(&scene).unwrap();
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
    let scene = Scene::new(scene_create_info()).unwrap();
    let g = GameObjectHandle::new(&scene).unwrap();
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
