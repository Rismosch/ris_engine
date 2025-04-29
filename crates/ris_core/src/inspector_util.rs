use std::ffi::CString;

use imgui::Ui;

use ris_asset_data::AssetId;
use ris_error::RisResult;
use ris_math::vector::Vec3;

use crate::inspector_util;
use crate::ui_helper::SharedStateWeakPtr;

// widgets
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

// fields
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

pub fn asset_field(
    label: impl AsRef<str>,
    shared_state: SharedStateWeakPtr,
    value: &mut Option<AssetId>,
    extension: Option<&str>,
) -> RisResult<bool> {
    let path = match &value {
        Some(AssetId::Path(path)) => path.as_str(),
        _ => {
            *value = None;
            "<none>"
        }
    };

    let label_cstring = CString::new(label.as_ref())?;
    let mut buf = path.to_string();
    buf.push('\0');
    let buf_ptr = buf.as_mut_ptr() as *mut i8;
    let buf_capacity = buf.capacity();

    let mut flags = 0;
    flags |= imgui::sys::ImGuiInputTextFlags_ReadOnly;

    let button_label = c"clear";

    unsafe {
        imgui::sys::igInputText(
            label_cstring.as_ptr(),
            buf_ptr,
            buf_capacity,
            flags as i32,
            None,
            std::ptr::null_mut(),
        )
    };

    let mut changed = false;

    if let Some(guard) = inspector_util::drag_drop_target() {
        let mut aref_mut = shared_state.borrow_mut();
        let payload = aref_mut.accept_drag_drop_payload::<AssetId>(&guard, "asset")?;

        if let Some(payload_data) = payload {
            let extension_matches = if let Some(extension) = extension {
                payload_data.has_extension(extension)
            } else {
                true
            };

            if extension_matches {
                *value = Some(payload_data);
                changed = true;
            }
        }
    }

    let clear_pressed = unsafe {
        imgui::sys::igSameLine(0.0, -1.0);
        imgui::sys::igButton(button_label.as_ptr(), [0.0, 0.0].into())
    };

    if clear_pressed {
        *value = None;
        changed = true;
    }

    Ok(changed)
}

// util
pub fn purge_negative_0(value: &mut [f32]) {
    let tolerance = 0.000_01;

    for item in value.iter_mut() {
        if item.abs() < tolerance {
            *item = 0.0;
        }
    }
}

// drag and drop
pub struct DragDropSourceGuard(());
pub struct DragDropTargetGuard(());

impl Drop for DragDropSourceGuard {
    fn drop(&mut self) {
        unsafe { imgui::sys::igEndDragDropSource() };
    }
}

impl Drop for DragDropTargetGuard {
    fn drop(&mut self) {
        unsafe { imgui::sys::igEndDragDropTarget() };
    }
}

pub fn drag_drop_source() -> Option<DragDropSourceGuard> {
    if unsafe { imgui::sys::igBeginDragDropSource(0) } {
        Some(DragDropSourceGuard(()))
    } else {
        None
    }
}

pub fn drag_drop_target() -> Option<DragDropTargetGuard> {
    if unsafe { imgui::sys::igBeginDragDropTarget() } {
        Some(DragDropTargetGuard(()))
    } else {
        None
    }
}

#[repr(C)]
struct AnyPayload {
    type_id: std::any::TypeId,
}

#[repr(C)]
struct GenericPayload<T: Copy> {
    type_id: std::any::TypeId,
    data: T,
}

pub fn set_drag_drop_payload<T: Copy + 'static>(
    _guard: &DragDropSourceGuard,
    type_str: impl AsRef<str>,
    data: T,
) -> RisResult<bool> {
    let type_str = type_str.as_ref();
    ris_error::assert!(!type_str.starts_with('_'))?;
    let type_cstring = CString::new(type_str)?;
    ris_error::assert!(type_cstring.as_bytes().len() <= 32)?;
    let type_ = type_cstring.as_ptr();

    let payload = GenericPayload {
        type_id: std::any::TypeId::of::<T>(),
        data,
    };
    let data = &payload as *const GenericPayload<T> as *const std::ffi::c_void;

    let payload_was_accepted = unsafe {
        imgui::sys::igSetDragDropPayload(type_, data, std::mem::size_of::<GenericPayload<T>>(), 0)
    };

    Ok(payload_was_accepted)
}

pub fn accept_drag_drop_payload<T: Copy + 'static>(
    _guard: &DragDropTargetGuard,
    type_str: impl AsRef<str>,
) -> RisResult<Option<T>> {
    let type_str = type_str.as_ref();
    ris_error::assert!(!type_str.starts_with('_'))?;
    let type_cstring = CString::new(type_str)?;
    ris_error::assert!(type_cstring.as_bytes().len() <= 32)?;
    let type_ = type_cstring.as_ptr();

    let payload = unsafe { imgui::sys::igAcceptDragDropPayload(type_, 0) };
    if payload.is_null() {
        return Ok(None);
    }

    let type_id = std::any::TypeId::of::<T>();
    let payload_data = unsafe { (*payload).Data };
    let payload_type_id = unsafe { &*(payload_data as *const AnyPayload) }.type_id;
    ris_error::assert!(type_id == payload_type_id)?;

    let data = unsafe { &*(payload_data as *const GenericPayload<T>) }.data;
    Ok(Some(data))
}
