use ris_data::game_object::GameObjectHandle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    GameObject(GameObjectHandle)
}

pub struct Selector {
    previous_selection: Option<Selection>,
    current_selection: Option<Selection>,
}

impl Selector {
    pub fn new() -> Self {
        Self{
            previous_selection: None,
            current_selection: None,
        }
    }

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
