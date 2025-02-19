cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
        pub use windows::*;
    } else if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::*;
    } else {
        compile_error!("imgui bindings haven't been build this os yet");
    }
}