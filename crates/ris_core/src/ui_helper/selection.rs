use std::path::PathBuf;

use ris_data::ecs::decl::GameObjectHandle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    GameObject(GameObjectHandle),
    AssetPath(PathBuf),
}

#[derive(Default, Debug)]
pub struct Selector {
    changed: bool,
    previous: Option<Selection>,
    current: Option<Selection>,
}

impl Selector {
    pub fn update(&mut self) {
        self.changed = self.current != self.previous;
        self.previous.clone_from(&self.current);
    }

    pub fn selection_changed(&self) -> bool {
        self.changed
    }

    pub fn get_selection(&self) -> Option<Selection> {
        self.current.clone()
    }

    pub fn set_selection(&mut self, value: Option<Selection>) {
        self.current = value;
    }
}
