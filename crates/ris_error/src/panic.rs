use sdl2::messagebox::MessageBoxFlag;

pub static mut SHOW_MESSAGE_BOX_ON_PANIC: bool = true;

#[macro_export]
macro_rules! panic {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        let backtrace = $crate::get_backtrace!();

        ris_log::fatal!("{} backtrace:\n{}", message, backtrace);
        $crate::panic::show_panic_message_box(&message);
        std::panic!("{}", message);
    }};
}

#[macro_export]
macro_rules! unwrap {
    ($result:expr, $($arg:tt)*) => {{
        match $result {
            Ok(value) => value,
            Err(error) => {
                let client_message = format!($($arg)*);
                $crate::panic!("{}: {}", client_message, error);
            }
        }
    }};
}

#[macro_export]
macro_rules! panic_assert {
    ($value:expr) => {{
        $crate::panic_assert!($value, "");
    }};
    ($value:expr, $($arg:tt)*) => {{
        #[cfg(not(debug_assertions))]
        {
            let _ = $value;
        }

        #[cfg(debug_assertions)]
        {
            if !$value {
                let message = format!($($arg)*);
                if message.len() == 0 {
                    $crate::panic!(
                        "assertion failed: `{}` was false",
                        stringify!($value),
                    );
                } else {
                    $crate::panic!(
                        "assertion failed: {}",
                        message,
                    );
                }

            }
        }
    }};
}

pub fn show_panic_message_box(message: &str) {
    if unsafe { !SHOW_MESSAGE_BOX_ON_PANIC } {
        return;
    }

    let _ = sdl2::messagebox::show_simple_message_box(
        MessageBoxFlag::ERROR,
        "Fatal Error",
        message,
        None,
    );
}
