use sdl2::messagebox::MessageBoxFlag;

#[macro_export]
macro_rules! throw {
    ($($arg:tt)*) => {
        {
            let panic_message = format!($($arg)*);
            ris_util::throw::log_fatal(&panic_message);
            ris_util::throw::show_panic_message_box(&panic_message);
            panic!("{}", panic_message);
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_throw {
    ($result:expr, $($arg:tt)*) => {
        match $result {
            Ok(value) => value,
            Err(error) => {
                let client_message = format!($($arg)*);
                ris_util::throw!("{}: {}", client_message, error);
            }
        }
    };
}

pub fn log_fatal(message: &str) {
    ris_log::fatal!("{}", message);
}

pub fn show_panic_message_box(message: &str) {
    let _ = sdl2::messagebox::show_simple_message_box(
        MessageBoxFlag::ERROR,
        "Fatal Error",
        message,
        None,
    );
}
