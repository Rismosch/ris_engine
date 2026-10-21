#[macro_export]
macro_rules! panic {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        let backtrace = $crate::get_backtrace!();

        unsafe {
            if let Some(ref mut hook) = $crate::FATAL_LOG_HOOK {
                hook(&format!("{}\nbacktrace:\n{}", message, backtrace));
            }
        }

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

pub fn show_panic_message_box(message: &str) -> bool {
    unsafe {
        if let Some(ref mut hook) = crate::MESSAGE_BOX_HOOK {
            hook(message);
            true
        } else {
            false
        }
    }
}
