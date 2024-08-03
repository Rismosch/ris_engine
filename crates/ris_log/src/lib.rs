cfg_if::cfg_if! {
    if #[cfg(feature = "testing")] {
        pub mod log;
    } else {
        pub mod no_log;
        pub use no_log as log;
    }
}

pub mod appender;
pub mod color_string;
pub mod log_level;
pub mod log_message;

