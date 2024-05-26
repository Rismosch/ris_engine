use sdl2::messagebox::MessageBoxFlag;

pub static mut SHOW_MESSAGE_BOX_ON_THROW: bool = true;

#[macro_export]
macro_rules! throw {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        let backtrace = $crate::get_backtrace!();

        ris_log::fatal!("{} backtrace:\n{}", message, backtrace);
        $crate::throw::show_panic_message_box(&message);
        panic!("{}", message);
    }};
}

#[macro_export]
macro_rules! unwrap {
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
macro_rules! throw_assert {
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
