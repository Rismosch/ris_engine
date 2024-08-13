use std::ffi::CString;

use imgui::Ui;

use ris_error::RisResult;

pub fn help_marker(ui: &Ui, text: &str) {
    ui.text_disabled("(?)");
    if ui.is_item_hovered() {
        ui.tooltip(|| {
            let token = ui.push_text_wrap_pos_with_pos(ui.current_font_size() * 35.0);
            ui.text_wrapped(text);
            token.end();
        });
    }
}

