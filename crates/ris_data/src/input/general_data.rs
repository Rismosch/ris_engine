use super::buttons::Buttons;

pub struct GeneralData {
    pub buttons: Buttons,
}

impl Default for GeneralData {
    fn default() -> Self {
        Self {
            buttons: Buttons::default(),
        }
    }
}