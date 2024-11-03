use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::decl::MeshComponentHandle;
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
struct TestScriptString{
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

    fn start(data: ScriptStartData) -> RisResult<Self> {
        Ok(Self{value: String::new()})
    }

    fn update(&mut self, data: ScriptUpdateData) -> RisResult<()> {
        self.value.push_str("\nupdate");
        Ok(())
    }

    fn end(&mut self, data: ScriptEndData) -> RisResult<()> {
        self.value.push_str("\nend");
        Ok(())
    }
}

impl Script for TestScriptISize {
    fn id() -> Sid {
        ris_debug::fsid!()
    }

    fn start(data: ScriptStartData) -> RisResult<Self> {
        Ok(Self{value: 0})
    }

    fn update(&mut self, data: ScriptUpdateData) -> RisResult<()> {
        self.value += 1;
        Ok(())
    }

    fn end(&mut self, data: ScriptEndData) -> RisResult<()> {
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

    let reference = &mut script.script_mut(&scene).unwrap().value;
    

    panic!("reached end");
}

#[test]
fn should_get_scripts() {
    panic!()
}

#[test]
fn should_get_first_script() {
    panic!()
}

