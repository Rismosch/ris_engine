use sdl2::{event::Event, keyboard::Scancode};

use ris_data::input::keyboard_data::KeyboardData;

pub fn pre_events(keyboard_data: &mut KeyboardData) {
    keyboard_data.text_input.clear();
}

pub fn handle_event(keyboard_data: &mut KeyboardData, event: &Event) {
    if let Event::TextInput { text, .. } = event {
        keyboard_data.text_input.push(text.to_owned());
    }
}

pub fn post_events(
    keyboard_data: &mut KeyboardData,
    keyboard_state: sdl2::keyboard::KeyboardState,
    mod_state: sdl2::keyboard::Mod,
) {
    keyboard_data.keys.clear();

    let mut new_button_state = 0;

    let iterator = ScancodeIterator::new(keyboard_state);
    for (scancode, value) in iterator {
        if !value {
            continue;
        }

        keyboard_data.keys.set(scancode);

        for i in 0..32 {
            if keyboard_data.keymask[i] == scancode {
                new_button_state |= 1 << i;
            }
        }
    }

    keyboard_data.buttons.update(new_button_state);
    keyboard_data.mod_state = mod_state;
}

// sdl2::keyboard::KeyboardState::scancodes() is broken due to
// an invalid internal transmute. thus we implement a custom
// iterator ourselves
struct ScancodeIterator<'a> {
    index: i32,
    state: sdl2::keyboard::KeyboardState<'a>,
}

impl<'a> Iterator for ScancodeIterator<'a> {
    type Item = (Scancode, bool);
    fn next(&mut self) -> Option<(Scancode, bool)> {
        let max = Scancode::Num as i32;
        while self.index < max {
            let current = self.index;
            self.index += 1;

            let Some(scancode) = convert_i32_to_scancode(current) else {
                continue;
            };

            let pressed = self.state.is_scancode_pressed(scancode);
            return Some((scancode, pressed));
        }

        None
    }
}

impl<'a> ScancodeIterator<'a> {
    fn new(state: sdl2::keyboard::KeyboardState<'a>) -> Self {
        Self { index: 0, state }
    }
}

fn convert_i32_to_scancode(value: i32) -> Option<Scancode> {
    use sdl2::sys::SDL_Scancode;

    let scancode = match value {
        v if v == SDL_Scancode::SDL_SCANCODE_A as i32 => Scancode::A,
        v if v == SDL_Scancode::SDL_SCANCODE_B as i32 => Scancode::B,
        v if v == SDL_Scancode::SDL_SCANCODE_C as i32 => Scancode::C,
        v if v == SDL_Scancode::SDL_SCANCODE_D as i32 => Scancode::D,
        v if v == SDL_Scancode::SDL_SCANCODE_E as i32 => Scancode::E,
        v if v == SDL_Scancode::SDL_SCANCODE_F as i32 => Scancode::F,
        v if v == SDL_Scancode::SDL_SCANCODE_G as i32 => Scancode::G,
        v if v == SDL_Scancode::SDL_SCANCODE_H as i32 => Scancode::H,
        v if v == SDL_Scancode::SDL_SCANCODE_I as i32 => Scancode::I,
        v if v == SDL_Scancode::SDL_SCANCODE_J as i32 => Scancode::J,
        v if v == SDL_Scancode::SDL_SCANCODE_K as i32 => Scancode::K,
        v if v == SDL_Scancode::SDL_SCANCODE_L as i32 => Scancode::L,
        v if v == SDL_Scancode::SDL_SCANCODE_M as i32 => Scancode::M,
        v if v == SDL_Scancode::SDL_SCANCODE_N as i32 => Scancode::N,
        v if v == SDL_Scancode::SDL_SCANCODE_O as i32 => Scancode::O,
        v if v == SDL_Scancode::SDL_SCANCODE_P as i32 => Scancode::P,
        v if v == SDL_Scancode::SDL_SCANCODE_Q as i32 => Scancode::Q,
        v if v == SDL_Scancode::SDL_SCANCODE_R as i32 => Scancode::R,
        v if v == SDL_Scancode::SDL_SCANCODE_S as i32 => Scancode::S,
        v if v == SDL_Scancode::SDL_SCANCODE_T as i32 => Scancode::T,
        v if v == SDL_Scancode::SDL_SCANCODE_U as i32 => Scancode::U,
        v if v == SDL_Scancode::SDL_SCANCODE_V as i32 => Scancode::V,
        v if v == SDL_Scancode::SDL_SCANCODE_W as i32 => Scancode::W,
        v if v == SDL_Scancode::SDL_SCANCODE_X as i32 => Scancode::X,
        v if v == SDL_Scancode::SDL_SCANCODE_Y as i32 => Scancode::Y,
        v if v == SDL_Scancode::SDL_SCANCODE_Z as i32 => Scancode::Z,
        v if v == SDL_Scancode::SDL_SCANCODE_1 as i32 => Scancode::Num1,
        v if v == SDL_Scancode::SDL_SCANCODE_2 as i32 => Scancode::Num2,
        v if v == SDL_Scancode::SDL_SCANCODE_3 as i32 => Scancode::Num3,
        v if v == SDL_Scancode::SDL_SCANCODE_4 as i32 => Scancode::Num4,
        v if v == SDL_Scancode::SDL_SCANCODE_5 as i32 => Scancode::Num5,
        v if v == SDL_Scancode::SDL_SCANCODE_6 as i32 => Scancode::Num6,
        v if v == SDL_Scancode::SDL_SCANCODE_7 as i32 => Scancode::Num7,
        v if v == SDL_Scancode::SDL_SCANCODE_8 as i32 => Scancode::Num8,
        v if v == SDL_Scancode::SDL_SCANCODE_9 as i32 => Scancode::Num9,
        v if v == SDL_Scancode::SDL_SCANCODE_0 as i32 => Scancode::Num0,
        v if v == SDL_Scancode::SDL_SCANCODE_RETURN as i32 => Scancode::Return,
        v if v == SDL_Scancode::SDL_SCANCODE_ESCAPE as i32 => Scancode::Escape,
        v if v == SDL_Scancode::SDL_SCANCODE_BACKSPACE as i32 => Scancode::Backspace,
        v if v == SDL_Scancode::SDL_SCANCODE_TAB as i32 => Scancode::Tab,
        v if v == SDL_Scancode::SDL_SCANCODE_SPACE as i32 => Scancode::Space,
        v if v == SDL_Scancode::SDL_SCANCODE_MINUS as i32 => Scancode::Minus,
        v if v == SDL_Scancode::SDL_SCANCODE_EQUALS as i32 => Scancode::Equals,
        v if v == SDL_Scancode::SDL_SCANCODE_LEFTBRACKET as i32 => Scancode::LeftBracket,
        v if v == SDL_Scancode::SDL_SCANCODE_RIGHTBRACKET as i32 => Scancode::RightBracket,
        v if v == SDL_Scancode::SDL_SCANCODE_BACKSLASH as i32 => Scancode::Backslash,
        v if v == SDL_Scancode::SDL_SCANCODE_NONUSHASH as i32 => Scancode::NonUsHash,
        v if v == SDL_Scancode::SDL_SCANCODE_SEMICOLON as i32 => Scancode::Semicolon,
        v if v == SDL_Scancode::SDL_SCANCODE_APOSTROPHE as i32 => Scancode::Apostrophe,
        v if v == SDL_Scancode::SDL_SCANCODE_GRAVE as i32 => Scancode::Grave,
        v if v == SDL_Scancode::SDL_SCANCODE_COMMA as i32 => Scancode::Comma,
        v if v == SDL_Scancode::SDL_SCANCODE_PERIOD as i32 => Scancode::Period,
        v if v == SDL_Scancode::SDL_SCANCODE_SLASH as i32 => Scancode::Slash,
        v if v == SDL_Scancode::SDL_SCANCODE_CAPSLOCK as i32 => Scancode::CapsLock,
        v if v == SDL_Scancode::SDL_SCANCODE_F1 as i32 => Scancode::F1,
        v if v == SDL_Scancode::SDL_SCANCODE_F2 as i32 => Scancode::F2,
        v if v == SDL_Scancode::SDL_SCANCODE_F3 as i32 => Scancode::F3,
        v if v == SDL_Scancode::SDL_SCANCODE_F4 as i32 => Scancode::F4,
        v if v == SDL_Scancode::SDL_SCANCODE_F5 as i32 => Scancode::F5,
        v if v == SDL_Scancode::SDL_SCANCODE_F6 as i32 => Scancode::F6,
        v if v == SDL_Scancode::SDL_SCANCODE_F7 as i32 => Scancode::F7,
        v if v == SDL_Scancode::SDL_SCANCODE_F8 as i32 => Scancode::F8,
        v if v == SDL_Scancode::SDL_SCANCODE_F9 as i32 => Scancode::F9,
        v if v == SDL_Scancode::SDL_SCANCODE_F10 as i32 => Scancode::F10,
        v if v == SDL_Scancode::SDL_SCANCODE_F11 as i32 => Scancode::F11,
        v if v == SDL_Scancode::SDL_SCANCODE_F12 as i32 => Scancode::F12,
        v if v == SDL_Scancode::SDL_SCANCODE_PRINTSCREEN as i32 => Scancode::PrintScreen,
        v if v == SDL_Scancode::SDL_SCANCODE_SCROLLLOCK as i32 => Scancode::ScrollLock,
        v if v == SDL_Scancode::SDL_SCANCODE_PAUSE as i32 => Scancode::Pause,
        v if v == SDL_Scancode::SDL_SCANCODE_INSERT as i32 => Scancode::Insert,
        v if v == SDL_Scancode::SDL_SCANCODE_HOME as i32 => Scancode::Home,
        v if v == SDL_Scancode::SDL_SCANCODE_PAGEUP as i32 => Scancode::PageUp,
        v if v == SDL_Scancode::SDL_SCANCODE_DELETE as i32 => Scancode::Delete,
        v if v == SDL_Scancode::SDL_SCANCODE_END as i32 => Scancode::End,
        v if v == SDL_Scancode::SDL_SCANCODE_PAGEDOWN as i32 => Scancode::PageDown,
        v if v == SDL_Scancode::SDL_SCANCODE_RIGHT as i32 => Scancode::Right,
        v if v == SDL_Scancode::SDL_SCANCODE_LEFT as i32 => Scancode::Left,
        v if v == SDL_Scancode::SDL_SCANCODE_DOWN as i32 => Scancode::Down,
        v if v == SDL_Scancode::SDL_SCANCODE_UP as i32 => Scancode::Up,
        v if v == SDL_Scancode::SDL_SCANCODE_NUMLOCKCLEAR as i32 => Scancode::NumLockClear,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_DIVIDE as i32 => Scancode::KpDivide,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_MULTIPLY as i32 => Scancode::KpMultiply,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_MINUS as i32 => Scancode::KpMinus,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_PLUS as i32 => Scancode::KpPlus,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_ENTER as i32 => Scancode::KpEnter,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_1 as i32 => Scancode::Kp1,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_2 as i32 => Scancode::Kp2,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_3 as i32 => Scancode::Kp3,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_4 as i32 => Scancode::Kp4,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_5 as i32 => Scancode::Kp5,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_6 as i32 => Scancode::Kp6,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_7 as i32 => Scancode::Kp7,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_8 as i32 => Scancode::Kp8,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_9 as i32 => Scancode::Kp9,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_0 as i32 => Scancode::Kp0,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_PERIOD as i32 => Scancode::KpPeriod,
        v if v == SDL_Scancode::SDL_SCANCODE_NONUSBACKSLASH as i32 => Scancode::NonUsBackslash,
        v if v == SDL_Scancode::SDL_SCANCODE_APPLICATION as i32 => Scancode::Application,
        v if v == SDL_Scancode::SDL_SCANCODE_POWER as i32 => Scancode::Power,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_EQUALS as i32 => Scancode::KpEquals,
        v if v == SDL_Scancode::SDL_SCANCODE_F13 as i32 => Scancode::F13,
        v if v == SDL_Scancode::SDL_SCANCODE_F14 as i32 => Scancode::F14,
        v if v == SDL_Scancode::SDL_SCANCODE_F15 as i32 => Scancode::F15,
        v if v == SDL_Scancode::SDL_SCANCODE_F16 as i32 => Scancode::F16,
        v if v == SDL_Scancode::SDL_SCANCODE_F17 as i32 => Scancode::F17,
        v if v == SDL_Scancode::SDL_SCANCODE_F18 as i32 => Scancode::F18,
        v if v == SDL_Scancode::SDL_SCANCODE_F19 as i32 => Scancode::F19,
        v if v == SDL_Scancode::SDL_SCANCODE_F20 as i32 => Scancode::F20,
        v if v == SDL_Scancode::SDL_SCANCODE_F21 as i32 => Scancode::F21,
        v if v == SDL_Scancode::SDL_SCANCODE_F22 as i32 => Scancode::F22,
        v if v == SDL_Scancode::SDL_SCANCODE_F23 as i32 => Scancode::F23,
        v if v == SDL_Scancode::SDL_SCANCODE_F24 as i32 => Scancode::F24,
        v if v == SDL_Scancode::SDL_SCANCODE_EXECUTE as i32 => Scancode::Execute,
        v if v == SDL_Scancode::SDL_SCANCODE_HELP as i32 => Scancode::Help,
        v if v == SDL_Scancode::SDL_SCANCODE_MENU as i32 => Scancode::Menu,
        v if v == SDL_Scancode::SDL_SCANCODE_SELECT as i32 => Scancode::Select,
        v if v == SDL_Scancode::SDL_SCANCODE_STOP as i32 => Scancode::Stop,
        v if v == SDL_Scancode::SDL_SCANCODE_AGAIN as i32 => Scancode::Again,
        v if v == SDL_Scancode::SDL_SCANCODE_UNDO as i32 => Scancode::Undo,
        v if v == SDL_Scancode::SDL_SCANCODE_CUT as i32 => Scancode::Cut,
        v if v == SDL_Scancode::SDL_SCANCODE_COPY as i32 => Scancode::Copy,
        v if v == SDL_Scancode::SDL_SCANCODE_PASTE as i32 => Scancode::Paste,
        v if v == SDL_Scancode::SDL_SCANCODE_FIND as i32 => Scancode::Find,
        v if v == SDL_Scancode::SDL_SCANCODE_MUTE as i32 => Scancode::Mute,
        v if v == SDL_Scancode::SDL_SCANCODE_VOLUMEUP as i32 => Scancode::VolumeUp,
        v if v == SDL_Scancode::SDL_SCANCODE_VOLUMEDOWN as i32 => Scancode::VolumeDown,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_COMMA as i32 => Scancode::KpComma,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_EQUALSAS400 as i32 => Scancode::KpEqualsAS400,
        v if v == SDL_Scancode::SDL_SCANCODE_INTERNATIONAL1 as i32 => Scancode::International1,
        v if v == SDL_Scancode::SDL_SCANCODE_INTERNATIONAL2 as i32 => Scancode::International2,
        v if v == SDL_Scancode::SDL_SCANCODE_INTERNATIONAL3 as i32 => Scancode::International3,
        v if v == SDL_Scancode::SDL_SCANCODE_INTERNATIONAL4 as i32 => Scancode::International4,
        v if v == SDL_Scancode::SDL_SCANCODE_INTERNATIONAL5 as i32 => Scancode::International5,
        v if v == SDL_Scancode::SDL_SCANCODE_INTERNATIONAL6 as i32 => Scancode::International6,
        v if v == SDL_Scancode::SDL_SCANCODE_INTERNATIONAL7 as i32 => Scancode::International7,
        v if v == SDL_Scancode::SDL_SCANCODE_INTERNATIONAL8 as i32 => Scancode::International8,
        v if v == SDL_Scancode::SDL_SCANCODE_INTERNATIONAL9 as i32 => Scancode::International9,
        v if v == SDL_Scancode::SDL_SCANCODE_LANG1 as i32 => Scancode::Lang1,
        v if v == SDL_Scancode::SDL_SCANCODE_LANG2 as i32 => Scancode::Lang2,
        v if v == SDL_Scancode::SDL_SCANCODE_LANG3 as i32 => Scancode::Lang3,
        v if v == SDL_Scancode::SDL_SCANCODE_LANG4 as i32 => Scancode::Lang4,
        v if v == SDL_Scancode::SDL_SCANCODE_LANG5 as i32 => Scancode::Lang5,
        v if v == SDL_Scancode::SDL_SCANCODE_LANG6 as i32 => Scancode::Lang6,
        v if v == SDL_Scancode::SDL_SCANCODE_LANG7 as i32 => Scancode::Lang7,
        v if v == SDL_Scancode::SDL_SCANCODE_LANG8 as i32 => Scancode::Lang8,
        v if v == SDL_Scancode::SDL_SCANCODE_LANG9 as i32 => Scancode::Lang9,
        v if v == SDL_Scancode::SDL_SCANCODE_ALTERASE as i32 => Scancode::AltErase,
        v if v == SDL_Scancode::SDL_SCANCODE_SYSREQ as i32 => Scancode::SysReq,
        v if v == SDL_Scancode::SDL_SCANCODE_CANCEL as i32 => Scancode::Cancel,
        v if v == SDL_Scancode::SDL_SCANCODE_CLEAR as i32 => Scancode::Clear,
        v if v == SDL_Scancode::SDL_SCANCODE_PRIOR as i32 => Scancode::Prior,
        v if v == SDL_Scancode::SDL_SCANCODE_RETURN2 as i32 => Scancode::Return2,
        v if v == SDL_Scancode::SDL_SCANCODE_SEPARATOR as i32 => Scancode::Separator,
        v if v == SDL_Scancode::SDL_SCANCODE_OUT as i32 => Scancode::Out,
        v if v == SDL_Scancode::SDL_SCANCODE_OPER as i32 => Scancode::Oper,
        v if v == SDL_Scancode::SDL_SCANCODE_CLEARAGAIN as i32 => Scancode::ClearAgain,
        v if v == SDL_Scancode::SDL_SCANCODE_CRSEL as i32 => Scancode::CrSel,
        v if v == SDL_Scancode::SDL_SCANCODE_EXSEL as i32 => Scancode::ExSel,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_00 as i32 => Scancode::Kp00,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_000 as i32 => Scancode::Kp000,
        v if v == SDL_Scancode::SDL_SCANCODE_THOUSANDSSEPARATOR as i32 => {
            Scancode::ThousandsSeparator
        }
        v if v == SDL_Scancode::SDL_SCANCODE_DECIMALSEPARATOR as i32 => Scancode::DecimalSeparator,
        v if v == SDL_Scancode::SDL_SCANCODE_CURRENCYUNIT as i32 => Scancode::CurrencyUnit,
        v if v == SDL_Scancode::SDL_SCANCODE_CURRENCYSUBUNIT as i32 => Scancode::CurrencySubUnit,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_LEFTPAREN as i32 => Scancode::KpLeftParen,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_RIGHTPAREN as i32 => Scancode::KpRightParen,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_LEFTBRACE as i32 => Scancode::KpLeftBrace,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_RIGHTBRACE as i32 => Scancode::KpRightBrace,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_TAB as i32 => Scancode::KpTab,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_BACKSPACE as i32 => Scancode::KpBackspace,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_A as i32 => Scancode::KpA,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_B as i32 => Scancode::KpB,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_C as i32 => Scancode::KpC,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_D as i32 => Scancode::KpD,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_E as i32 => Scancode::KpE,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_F as i32 => Scancode::KpF,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_XOR as i32 => Scancode::KpXor,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_POWER as i32 => Scancode::KpPower,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_PERCENT as i32 => Scancode::KpPercent,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_LESS as i32 => Scancode::KpLess,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_GREATER as i32 => Scancode::KpGreater,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_AMPERSAND as i32 => Scancode::KpAmpersand,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_DBLAMPERSAND as i32 => Scancode::KpDblAmpersand,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_VERTICALBAR as i32 => Scancode::KpVerticalBar,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_DBLVERTICALBAR as i32 => Scancode::KpDblVerticalBar,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_COLON as i32 => Scancode::KpColon,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_HASH as i32 => Scancode::KpHash,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_SPACE as i32 => Scancode::KpSpace,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_AT as i32 => Scancode::KpAt,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_EXCLAM as i32 => Scancode::KpExclam,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_MEMSTORE as i32 => Scancode::KpMemStore,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_MEMRECALL as i32 => Scancode::KpMemRecall,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_MEMCLEAR as i32 => Scancode::KpMemClear,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_MEMADD as i32 => Scancode::KpMemAdd,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_MEMSUBTRACT as i32 => Scancode::KpMemSubtract,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_MEMMULTIPLY as i32 => Scancode::KpMemMultiply,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_MEMDIVIDE as i32 => Scancode::KpMemDivide,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_PLUSMINUS as i32 => Scancode::KpPlusMinus,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_CLEAR as i32 => Scancode::KpClear,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_CLEARENTRY as i32 => Scancode::KpClearEntry,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_BINARY as i32 => Scancode::KpBinary,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_OCTAL as i32 => Scancode::KpOctal,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_DECIMAL as i32 => Scancode::KpDecimal,
        v if v == SDL_Scancode::SDL_SCANCODE_KP_HEXADECIMAL as i32 => Scancode::KpHexadecimal,
        v if v == SDL_Scancode::SDL_SCANCODE_LCTRL as i32 => Scancode::LCtrl,
        v if v == SDL_Scancode::SDL_SCANCODE_LSHIFT as i32 => Scancode::LShift,
        v if v == SDL_Scancode::SDL_SCANCODE_LALT as i32 => Scancode::LAlt,
        v if v == SDL_Scancode::SDL_SCANCODE_LGUI as i32 => Scancode::LGui,
        v if v == SDL_Scancode::SDL_SCANCODE_RCTRL as i32 => Scancode::RCtrl,
        v if v == SDL_Scancode::SDL_SCANCODE_RSHIFT as i32 => Scancode::RShift,
        v if v == SDL_Scancode::SDL_SCANCODE_RALT as i32 => Scancode::RAlt,
        v if v == SDL_Scancode::SDL_SCANCODE_RGUI as i32 => Scancode::RGui,
        v if v == SDL_Scancode::SDL_SCANCODE_MODE as i32 => Scancode::Mode,
        v if v == SDL_Scancode::SDL_SCANCODE_AUDIONEXT as i32 => Scancode::AudioNext,
        v if v == SDL_Scancode::SDL_SCANCODE_AUDIOPREV as i32 => Scancode::AudioPrev,
        v if v == SDL_Scancode::SDL_SCANCODE_AUDIOSTOP as i32 => Scancode::AudioStop,
        v if v == SDL_Scancode::SDL_SCANCODE_AUDIOPLAY as i32 => Scancode::AudioPlay,
        v if v == SDL_Scancode::SDL_SCANCODE_AUDIOMUTE as i32 => Scancode::AudioMute,
        v if v == SDL_Scancode::SDL_SCANCODE_MEDIASELECT as i32 => Scancode::MediaSelect,
        v if v == SDL_Scancode::SDL_SCANCODE_WWW as i32 => Scancode::Www,
        v if v == SDL_Scancode::SDL_SCANCODE_MAIL as i32 => Scancode::Mail,
        v if v == SDL_Scancode::SDL_SCANCODE_CALCULATOR as i32 => Scancode::Calculator,
        v if v == SDL_Scancode::SDL_SCANCODE_COMPUTER as i32 => Scancode::Computer,
        v if v == SDL_Scancode::SDL_SCANCODE_AC_SEARCH as i32 => Scancode::AcSearch,
        v if v == SDL_Scancode::SDL_SCANCODE_AC_HOME as i32 => Scancode::AcHome,
        v if v == SDL_Scancode::SDL_SCANCODE_AC_BACK as i32 => Scancode::AcBack,
        v if v == SDL_Scancode::SDL_SCANCODE_AC_FORWARD as i32 => Scancode::AcForward,
        v if v == SDL_Scancode::SDL_SCANCODE_AC_STOP as i32 => Scancode::AcStop,
        v if v == SDL_Scancode::SDL_SCANCODE_AC_REFRESH as i32 => Scancode::AcRefresh,
        v if v == SDL_Scancode::SDL_SCANCODE_AC_BOOKMARKS as i32 => Scancode::AcBookmarks,
        v if v == SDL_Scancode::SDL_SCANCODE_BRIGHTNESSDOWN as i32 => Scancode::BrightnessDown,
        v if v == SDL_Scancode::SDL_SCANCODE_BRIGHTNESSUP as i32 => Scancode::BrightnessUp,
        v if v == SDL_Scancode::SDL_SCANCODE_DISPLAYSWITCH as i32 => Scancode::DisplaySwitch,
        v if v == SDL_Scancode::SDL_SCANCODE_KBDILLUMTOGGLE as i32 => Scancode::KbdIllumToggle,
        v if v == SDL_Scancode::SDL_SCANCODE_KBDILLUMDOWN as i32 => Scancode::KbdIllumDown,
        v if v == SDL_Scancode::SDL_SCANCODE_KBDILLUMUP as i32 => Scancode::KbdIllumUp,
        v if v == SDL_Scancode::SDL_SCANCODE_EJECT as i32 => Scancode::Eject,
        v if v == SDL_Scancode::SDL_SCANCODE_SLEEP as i32 => Scancode::Sleep,
        v if v == SDL_Scancode::SDL_SCANCODE_APP1 as i32 => Scancode::App1,
        v if v == SDL_Scancode::SDL_SCANCODE_APP2 as i32 => Scancode::App2,
        v if v == SDL_Scancode::SDL_NUM_SCANCODES as i32 => Scancode::Num,
        _ => return None,
    };

    Some(scancode)
}
