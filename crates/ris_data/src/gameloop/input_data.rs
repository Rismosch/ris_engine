use crate::input::{keyboard_data::KeyboardData, mouse_data::MouseData};

pub struct InputData {
    mouse: Option<MouseData>,
    keyboard: Option<KeyboardData>,
}

impl InputData {
    pub fn get_mouse(&self) -> &MouseData {
        match &self.mouse {
            Some(data) => data,
            None => panic!(),
        }
    }

    pub fn take_mouse(&mut self) -> MouseData {
        match self.mouse.take() {
            Some(data) => data,
            None => panic!(),
        }
    }

    pub fn set_mouse(&mut self, data: MouseData) {
        self.mouse = Some(data);
    }

    pub fn get_keyboard(&self) -> &KeyboardData {
        match &self.keyboard {
            Some(data) => data,
            None => panic!(),
        }
    }

    pub fn take_keyboard(&mut self) -> KeyboardData {
        match self.keyboard.take() {
            Some(data) => data,
            None => panic!(),
        }
    }

    pub fn set_keyboard(&mut self, data: KeyboardData) {
        self.keyboard = Some(data);
    }
}

impl Default for InputData {
    fn default() -> Self {
        Self {
            mouse: Some(MouseData::default()),
            keyboard: Some(KeyboardData::default()),
        }
    }
}
