use ris_data::ecs::handle::GameObjectHandle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    GameObject(GameObjectHandle),
}

#[derive(Default)]
pub struct Selector {
    previous_selection: Option<Selection>,
    current_selection: Option<Selection>,
}

impl Selector {
    pub fn update(&mut self) {
        self.previous_selection = self.current_selection.clone();
    }

    pub fn selection_changed(&self) -> bool {
        self.current_selection != self.previous_selection
    }

    pub fn get_selection(&self) -> Option<Selection> {
        self.current_selection.clone()
    }

    pub fn set_selection(&mut self, value: Option<Selection>) {
        self.current_selection = value;
    }
}
