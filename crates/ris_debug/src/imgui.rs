use std::time::Instant;

use imgui::BackendFlags;
use imgui::ConfigFlags;
use imgui::Context;
use imgui::Io;
use imgui::MouseCursor;
use imgui::Ui;
use sdl2::event::Event;
use sdl2::keyboard::Mod;
use sdl2::keyboard::Scancode;
use sdl2::mouse::Cursor;
use sdl2::mouse::SystemCursor;
use sdl2::video::Window;

use ris_data::info::app_info::AppInfo;
use ris_data::input::mouse_data::MouseData;

fn handle_key(io: &mut Io, key: &Scancode, pressed: bool) {
    let igkey = match key {
        Scancode::A => imgui::Key::A,
        Scancode::B => imgui::Key::B,
        Scancode::C => imgui::Key::C,
        Scancode::D => imgui::Key::D,
        Scancode::E => imgui::Key::E,
        Scancode::F => imgui::Key::F,
        Scancode::G => imgui::Key::G,
        Scancode::H => imgui::Key::H,
        Scancode::I => imgui::Key::I,
        Scancode::J => imgui::Key::J,
        Scancode::K => imgui::Key::K,
        Scancode::L => imgui::Key::L,
        Scancode::M => imgui::Key::M,
        Scancode::N => imgui::Key::N,
        Scancode::O => imgui::Key::O,
        Scancode::P => imgui::Key::P,
        Scancode::Q => imgui::Key::Q,
        Scancode::R => imgui::Key::R,
        Scancode::S => imgui::Key::S,
        Scancode::T => imgui::Key::T,
        Scancode::U => imgui::Key::U,
        Scancode::V => imgui::Key::V,
        Scancode::W => imgui::Key::W,
        Scancode::X => imgui::Key::X,
        Scancode::Y => imgui::Key::Y,
        Scancode::Z => imgui::Key::Z,
        Scancode::Num1 => imgui::Key::Keypad1,
        Scancode::Num2 => imgui::Key::Keypad2,
        Scancode::Num3 => imgui::Key::Keypad3,
        Scancode::Num4 => imgui::Key::Keypad4,
        Scancode::Num5 => imgui::Key::Keypad5,
        Scancode::Num6 => imgui::Key::Keypad6,
        Scancode::Num7 => imgui::Key::Keypad7,
        Scancode::Num8 => imgui::Key::Keypad8,
        Scancode::Num9 => imgui::Key::Keypad9,
        Scancode::Num0 => imgui::Key::Keypad0,
        Scancode::Return => imgui::Key::Enter, // TODO: Should this be treated as alias?
        Scancode::Escape => imgui::Key::Escape,
        Scancode::Backspace => imgui::Key::Backspace,
        Scancode::Tab => imgui::Key::Tab,
        Scancode::Space => imgui::Key::Space,
        Scancode::Minus => imgui::Key::Minus,
        Scancode::Equals => imgui::Key::Equal,
        Scancode::LeftBracket => imgui::Key::LeftBracket,
        Scancode::RightBracket => imgui::Key::RightBracket,
        Scancode::Backslash => imgui::Key::Backslash,
        Scancode::Semicolon => imgui::Key::Semicolon,
        Scancode::Apostrophe => imgui::Key::Apostrophe,
        Scancode::Grave => imgui::Key::GraveAccent,
        Scancode::Comma => imgui::Key::Comma,
        Scancode::Period => imgui::Key::Period,
        Scancode::Slash => imgui::Key::Slash,
        Scancode::CapsLock => imgui::Key::CapsLock,
        Scancode::F1 => imgui::Key::F1,
        Scancode::F2 => imgui::Key::F2,
        Scancode::F3 => imgui::Key::F3,
        Scancode::F4 => imgui::Key::F4,
        Scancode::F5 => imgui::Key::F5,
        Scancode::F6 => imgui::Key::F6,
        Scancode::F7 => imgui::Key::F7,
        Scancode::F8 => imgui::Key::F8,
        Scancode::F9 => imgui::Key::F9,
        Scancode::F10 => imgui::Key::F10,
        Scancode::F11 => imgui::Key::F11,
        Scancode::F12 => imgui::Key::F12,
        Scancode::PrintScreen => imgui::Key::PrintScreen,
        Scancode::ScrollLock => imgui::Key::ScrollLock,
        Scancode::Pause => imgui::Key::Pause,
        Scancode::Insert => imgui::Key::Insert,
        Scancode::Home => imgui::Key::Home,
        Scancode::PageUp => imgui::Key::PageUp,
        Scancode::Delete => imgui::Key::Delete,
        Scancode::End => imgui::Key::End,
        Scancode::PageDown => imgui::Key::PageDown,
        Scancode::Right => imgui::Key::RightArrow,
        Scancode::Left => imgui::Key::LeftArrow,
        Scancode::Down => imgui::Key::DownArrow,
        Scancode::Up => imgui::Key::UpArrow,
        Scancode::KpDivide => imgui::Key::KeypadDivide,
        Scancode::KpMultiply => imgui::Key::KeypadMultiply,
        Scancode::KpMinus => imgui::Key::KeypadSubtract,
        Scancode::KpPlus => imgui::Key::KeypadAdd,
        Scancode::KpEnter => imgui::Key::KeypadEnter,
        Scancode::Kp1 => imgui::Key::Keypad1,
        Scancode::Kp2 => imgui::Key::Keypad2,
        Scancode::Kp3 => imgui::Key::Keypad3,
        Scancode::Kp4 => imgui::Key::Keypad4,
        Scancode::Kp5 => imgui::Key::Keypad5,
        Scancode::Kp6 => imgui::Key::Keypad6,
        Scancode::Kp7 => imgui::Key::Keypad7,
        Scancode::Kp8 => imgui::Key::Keypad8,
        Scancode::Kp9 => imgui::Key::Keypad9,
        Scancode::Kp0 => imgui::Key::Keypad0,
        Scancode::KpPeriod => imgui::Key::KeypadDecimal,
        Scancode::Application => imgui::Key::Menu,
        Scancode::KpEquals => imgui::Key::KeypadEqual,
        Scancode::Menu => imgui::Key::Menu,
        Scancode::LCtrl => imgui::Key::LeftCtrl,
        Scancode::LShift => imgui::Key::LeftShift,
        Scancode::LAlt => imgui::Key::LeftAlt,
        Scancode::LGui => imgui::Key::LeftSuper,
        Scancode::RCtrl => imgui::Key::RightCtrl,
        Scancode::RShift => imgui::Key::RightShift,
        Scancode::RAlt => imgui::Key::RightAlt,
        Scancode::RGui => imgui::Key::RightSuper,
        _ => {
            ris_log::warning!("key ignored: unkown scancode");
            return;
        }
    };

    io.add_key_event(igkey, pressed);
}

fn handle_key_modifier(io: &mut Io, keymod: &Mod) {
    io.add_key_event(
        imgui::Key::ModShift,
        keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD),
    );
    io.add_key_event(
        imgui::Key::ModCtrl,
        keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD),
    );
    io.add_key_event(
        imgui::Key::ModAlt,
        keymod.intersects(Mod::LALTMOD | Mod::RALTMOD),
    );
    io.add_key_event(
        imgui::Key::ModSuper,
        keymod.intersects(Mod::LGUIMOD | Mod::RGUIMOD),
    );
}

fn to_sdl_cursor(cursor: MouseCursor) -> SystemCursor {
    match cursor {
        MouseCursor::Arrow => SystemCursor::Arrow,
        MouseCursor::TextInput => SystemCursor::IBeam,
        MouseCursor::ResizeAll => SystemCursor::SizeAll,
        MouseCursor::ResizeNS => SystemCursor::SizeNS,
        MouseCursor::ResizeEW => SystemCursor::SizeWE,
        MouseCursor::ResizeNESW => SystemCursor::SizeNESW,
        MouseCursor::ResizeNWSE => SystemCursor::SizeNWSE,
        MouseCursor::Hand => SystemCursor::Hand,
        MouseCursor::NotAllowed => SystemCursor::No,
    }
}

fn handle_mouse_button(
    io: &mut Io,
    button: &sdl2::mouse::MouseButton,
    pressed: bool,
) {
    match button {
        sdl2::mouse::MouseButton::Left => {
            io.add_mouse_button_event(imgui::MouseButton::Left, pressed)
        }
        sdl2::mouse::MouseButton::Right => {
            io.add_mouse_button_event(imgui::MouseButton::Right, pressed)
        }
        sdl2::mouse::MouseButton::Middle => {
            io.add_mouse_button_event(imgui::MouseButton::Middle, pressed)
        }
        sdl2::mouse::MouseButton::X1 => {
            io.add_mouse_button_event(imgui::MouseButton::Extra1, pressed)
        }
        sdl2::mouse::MouseButton::X2 => {
            io.add_mouse_button_event(imgui::MouseButton::Extra2, pressed)
        }
        _ => {}
    }
}

pub fn filter_event(window: &Window, event: &Event) -> bool {
    Some(window.id()) == event.get_window_id()
}

pub struct Imgui {
    context: Context,
    cursor_instance: Option<Cursor>, /* to avoid dropping cursor instances */
    last_frame: Instant,
}

impl Imgui {
    pub fn new(app_info: &AppInfo) -> Self {
        let mut context = Context::create();

        context.set_ini_filename(None);
        context.set_log_filename(None);

        let font_atlas = context.fonts();
        font_atlas.add_font(&[imgui::FontSource::DefaultFontData{config: None}]);
        let texture = font_atlas.build_rgba32_texture();

        let io = context.io_mut();

        io.backend_flags.insert(BackendFlags::HAS_MOUSE_CURSORS);
        io.backend_flags.insert(BackendFlags::HAS_SET_MOUSE_POS);

        context.set_platform_name(Some(format!(
            "imgui sdl2 platform for {} {}",
            app_info.package.name,
            app_info.package.version
        )));
        context.set_renderer_name(Some(format!(
            "imgui vulkano renderer for {} {}",
            app_info.package.name,
            app_info.package.version
        )));

        Self {
            context,
            cursor_instance: None,
            last_frame: Instant::now(),
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> bool {
        let io = self.context.io_mut();

        match *event {
            Event::MouseWheel { x, y, .. } => {
                io.add_mouse_wheel_event([x as f32, y as f32]);
                true
            }

            Event::MouseButtonDown { mouse_btn, .. } => {
                handle_mouse_button(io, &mouse_btn, true);
                true
            }

            Event::MouseButtonUp { mouse_btn, .. } => {
                handle_mouse_button(io, &mouse_btn, false);
                true
            }

            Event::TextInput { ref text, .. } => {
                text.chars().for_each(|c| io.add_input_character(c));
                true
            }

            Event::KeyDown {
                scancode: Some(key),
                keymod,
                ..
            } => {
                handle_key_modifier(io, &keymod);
                handle_key(io, &key, true);
                true
            }

            Event::KeyUp {
                scancode: Some(key),
                keymod,
                ..
            } => {
                handle_key_modifier(io, &keymod);
                handle_key(io, &key, false);
                true
            }

            _ => false,
        }
    }

    pub fn prepare_and_create_new_frame(
        &mut self,
        window: &Window,
        mouse_data: &MouseData,
    ) -> &mut Ui {
        let mouse_cursor = self.context.mouse_cursor();
        let io = self.context.io_mut();

        // Update delta time
        let now = Instant::now();
        io.update_delta_time(now.duration_since(self.last_frame));
        self.last_frame = now;

        let window_size = window.size();
        let window_drawable_size = window.drawable_size();

        // Set display size and scale here, since SDL 2 doesn't have
        // any easy way to get the scale factor, and changes in said
        // scale factor
        io.display_size = [window_size.0 as f32, window_size.1 as f32];
        io.display_framebuffer_scale = [
            (window_drawable_size.0 as f32) / (window_size.0 as f32),
            (window_drawable_size.1 as f32) / (window_size.1 as f32),
        ];

        // Set mouse position if requested by imgui-rs
        if io.want_set_mouse_pos {
            let mouse_util = window.subsystem().sdl().mouse();
            mouse_util.warp_mouse_in_window(window, io.mouse_pos[0] as i32, io.mouse_pos[1] as i32);
        }

        // Update mouse cursor position
        io.mouse_pos = [mouse_data.x as f32, mouse_data.y as f32];

        // Update mouse cursor icon if requested
        if !io
            .config_flags
            .contains(ConfigFlags::NO_MOUSE_CURSOR_CHANGE)
        {
            let mouse_util = window.subsystem().sdl().mouse();

            match mouse_cursor {
                Some(mouse_cursor) if !io.mouse_draw_cursor => {
                    let cursor = Cursor::from_system(to_sdl_cursor(mouse_cursor)).unwrap();
                    cursor.set();

                    mouse_util.show_cursor(true);
                    self.cursor_instance = Some(cursor);
                }

                _ => {
                    mouse_util.show_cursor(false);
                    self.cursor_instance = None;
                }
            }
        }

        self.context.new_frame()
    }

    pub fn render(&mut self) {
        let draw_data = self.context.render();
        render
    }
}
