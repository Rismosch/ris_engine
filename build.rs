#[cfg(target_os = "windows")]
fn main() {
    use std::path::PathBuf;

    // environment variables
    let out_dir = std::env::var("OUT_DIR").expect("failed to find environment variable OUT_DIR");
    let profile = std::env::var("PROFILE").expect("failed to find_environment variable PROFILE");
    let vulkan_sdk_dir =
        std::env::var("VULKAN_SDK").expect("failed to find the environment variable VULKAN_SDK");

    // find target dir
    let out_dir = PathBuf::from(out_dir);
    let mut target_dir = None;
    let mut target_dir_candidate = out_dir.as_path();
    while let Some(parent) = target_dir_candidate.parent() {
        if parent.ends_with(&profile) {
            target_dir = Some(parent);
            break;
        }

        target_dir_candidate = parent;
    }
    let target_dir = target_dir.expect("failed to find target dir");

    // define dirs
    let vulkan_bin_dir = PathBuf::from(&vulkan_sdk_dir).join("Bin");
    let vulkan_lib_dir = PathBuf::from(&vulkan_sdk_dir).join("Lib");
    let sdl2_filename = "SDL2";
    let sdl2_dll_source_path = PathBuf::from(&vulkan_bin_dir).join(sdl2_filename);

    // link
    println!(
        "cargo:rustc-link-search=native={}",
        vulkan_bin_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        vulkan_lib_dir.display()
    );
    println!("cargo:rustc-link-lib=dylib={}", sdl2_filename);

    // copy dlls
    copy_dll(sdl2_dll_source_path, target_dir);
}

#[cfg(target_os = "windows")]
fn copy_dll(source_path: impl AsRef<std::path::Path>, target_dir: impl AsRef<std::path::Path>) {
    let source_path = source_path.as_ref();
    let target_dir = target_dir.as_ref();
    let extension = ".dll";

    let filename = if source_path.ends_with(extension) {
        source_path.file_stem()
    } else {
        source_path.file_name()
    };

    let filename = filename
        .unwrap_or_else(|| panic!("failed to get file_stem of \"{}\"", source_path.display()))
        .to_str()
        .unwrap_or_else(|| {
            panic!(
                "failed to convert OsStr to str of \"{}\"",
                source_path.display()
            )
        });
    let filestem = format!("{}{}", filename, extension);

    let source_dir = source_path
        .parent()
        .unwrap_or_else(|| panic!("failed to get parent of \"{}\"", source_path.display()));
    let source_path = source_dir.join(&filestem);
    let target_path = target_dir.join(&filestem);
    //let result = std::fs::copy(&source_path, &target_path).expect(&format!("failed to copy \"{}\" to \"{}\"", source_path.display(), target_path.display()));
    let copy_result = std::fs::copy(&source_path, &target_path);
    if let Err(e) = copy_result {
        // only throw error if file does not exist. chances are the copy failed because it is used by another process, most likely `cargo run -p cli pipeline`
        if !target_path.exists() {
            panic!(
                "failed to copy \"{}\" to \"{}\": {}",
                source_path.display(),
                target_path.display(),
                e,
            )
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}
