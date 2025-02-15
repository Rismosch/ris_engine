use std::path::PathBuf;

fn main() {
    build_imgui();
}

fn build_imgui() {
    let imgui_dir = "third_party/imgui";

    // generate bindings
    let target_dir = "bindings_imgui";
    let vulkan_sdk_dir = std::env::var("VULKAN_SDK").expect("Vulkan SDK not found");
    let vulkan_include_dir = &format!("{}/Include", vulkan_sdk_dir);

    generate_bindings(
        format!("{}/imconfig.h", imgui_dir),
        &[imgui_dir],
        target_dir,
    );
    generate_bindings(format!("{}/imgui.h", imgui_dir), &[imgui_dir], target_dir);
    generate_bindings(
        format!("{}/imgui_internal.h", imgui_dir),
        &[imgui_dir],
        target_dir,
    );
    generate_bindings(
        format!("{}/imstb_rectpack.h", imgui_dir),
        &[imgui_dir],
        target_dir,
    );
    generate_bindings(
        format!("{}/imstb_textedit.h", imgui_dir),
        &[imgui_dir],
        target_dir,
    );
    generate_bindings(
        format!("{}/imstb_truetype.h", imgui_dir),
        &[imgui_dir],
        target_dir,
    );
    generate_bindings(
        format!("{}/backends/imgui_impl_sdl2.h", imgui_dir),
        &[imgui_dir],
        format!("{}/backends", target_dir),
    );
    generate_bindings(
        format!("{}/backends/imgui_impl_vulkan.h", imgui_dir),
        &[imgui_dir, vulkan_include_dir],
        format!("{}/backends", target_dir),
    );

    // compile and link
    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .include(imgui_dir)
        .include(format!("{}/backends", imgui_dir))
        .include(vulkan_include_dir)
        .include(format!("{}/SDL2", vulkan_include_dir))
        .file(format!("{}/imgui.cpp", imgui_dir))
        .file(format!("{}/imgui_demo.cpp", imgui_dir))
        .file(format!("{}/imgui_draw.cpp", imgui_dir))
        .file(format!("{}/imgui_tables.cpp", imgui_dir))
        .file(format!("{}/imgui_widgets.cpp", imgui_dir))
        .file(format!("{}/backends/imgui_impl_sdl2.cpp", imgui_dir))
        .file(format!("{}/backends/imgui_impl_vulkan.cpp", imgui_dir))
        .compile("imgui");

    println!("cargo:rustc-link-lib=static=imgui");
}

fn generate_bindings(
    header: impl AsRef<str>,
    include_dirs: &[impl AsRef<str>],
    target_dir: impl AsRef<str>,
) {
    let header = header.as_ref();
    let include_dirs = include_dirs.iter().map(|x| x.as_ref()).collect::<Vec<_>>();
    let target_dir = target_dir.as_ref();

    println!("cargo:rerun-if-changed={}", header);

    let mut bindgen_builder = bindgen::Builder::default();
    for include_dir in include_dirs {
        bindgen_builder = bindgen_builder.clang_arg(format!("-I{}", include_dir));
    }

    let bindings = bindgen_builder
        .header(header)
        .clang_arg("-xc++")
        .clang_arg("-std=c++17")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate bindings");

    let header_path = PathBuf::from(header);
    let filename = header_path
        .file_stem()
        .expect("header path had no file name")
        .to_str()
        .expect("failed to convert OsStr to str");

    let out_dir = std::env::var("OUT_DIR").expect("failed to resolve env OUT_DIR");
    let target_path = PathBuf::from(out_dir)
        .join(target_dir)
        .join(format!("{}.rs", filename));

    let target_parent = target_path.parent().expect("target_path had no parent");
    std::fs::create_dir_all(target_parent).expect("failed to create target_parent");

    bindings
        .write_to_file(&target_path)
        .expect("failed to write bindings");
}
