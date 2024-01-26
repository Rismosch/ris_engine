use sdl2::messagebox::MessageBoxFlag;

pub static mut SHOW_MESSAGE_BOX_ON_THROW: bool = true;

#[macro_export]
macro_rules! throw {
    ($($arg:tt)*) => {{
        let panic_message = format!($($arg)*);
        ris_log::fatal!("{}", panic_message);
        $crate::throw::show_panic_message_box(&panic_message);
        panic!("{}", panic_message);
    }};
}

#[macro_export]
macro_rules! unwrap_or_throw {
    ($result:expr, $($arg:tt)*) => {{
        match $result {
            Ok(value) => value,
            Err(error) => {
                let client_message = format!($($arg)*);
                $crate::throw!("{}: {}", client_message, error);
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_or_throw {
    ($result:expr, $($arg:tt)*) => {{
        if !$result {
            let client_message = format!($($arg)*);
            $crate::throw!("{}", client_message);
        }
    }};
}

pub fn show_panic_message_box(message: &str) {
    if unsafe { !SHOW_MESSAGE_BOX_ON_THROW } {
        return;
    }

    let _ = sdl2::messagebox::show_simple_message_box(
        MessageBoxFlag::ERROR,
        "Fatal Error",
        message,
        None,
    );
}
