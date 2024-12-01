use std::ffi::CString;

use imgui::Ui;

use ris_error::RisResult;
use ris_math::vector::Vec2;
use ris_math::vector::Vec3;
use ris_math::vector::Vec4;

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

pub fn drag_vec3(label: impl AsRef<str>, value: &mut Vec3) -> RisResult<bool> {
    let label_cstring = CString::new(label.as_ref())?;
    let mut array: [f32; 3] = (*value).into();
    let format = CString::new("%.3f")?;

    purge_negative_0(&mut array);

    let changed = unsafe {
        imgui::sys::igDragFloat3(
            label_cstring.as_ptr(),
            array.as_mut_ptr(),
            0.01,
            0.0,
            0.0,
            format.as_ptr(),
            0,
        )
    };

    *value = array.into();
    Ok(changed)
}

pub fn purge_negative_0(value: &mut [f32]) {
    let tolerance = 0.000_01;

    for item in value.iter_mut() {
        if item.abs() < tolerance {
            *item = 0.0;
        }
    }
}
