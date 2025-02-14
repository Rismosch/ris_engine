use std::path::PathBuf;

fn main() {
    link_sdl2();
    compile_and_link_imgui();
    compile_and_link_shaderc();
}

fn link_sdl2() {
    println!("cargo:rustc-link-lib=SDL2");

    println!("cargo:rustc-link-search=native=external/SDL2/lib/x64");
}

fn compile_and_link_imgui() {

}

fn compile_and_link_shaderc() {
    let include_dir = "external/shaderc/libshaderc/include";
    let target_dir = "bindings_shaderc";

    generate_bindings(
        "external/shaderc/libshaderc/include/shaderc/env.h",
        include_dir,
        target_dir,
    );
    generate_bindings(
        "external/shaderc/libshaderc/include/shaderc/shaderc.h",
        include_dir,
        target_dir,
    );
    generate_bindings(
        "external/shaderc/libshaderc/include/shaderc/shaderc.hpp",
        include_dir,
        target_dir,
    );
    generate_bindings(
        "external/shaderc/libshaderc/include/shaderc/status.h",
        include_dir,
        target_dir,
    );
    generate_bindings(
        "external/shaderc/libshaderc/include/shaderc/visibility.h",
        include_dir,
        target_dir,
    );

}

fn generate_bindings(header: &str, include_dir: &str, target_dir: &str) {
    println!("cargo:rerun-if-changed={}", header);

    let bindings = bindgen::Builder::default()
        .header(header)
        .clang_arg(format!("-I{}", include_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate bindings");

    let header_path = PathBuf::from(header);
    let filename = header_path
        .file_name()
        .expect("header path had no file name")
        .to_str()
        .expect("failed to convert OsStr to str")
        .replace('.', "_");

    let out_dir = std::env::var("OUT_DIR").expect("failed to resolve env OUT_DIR");
    let target_path = PathBuf::from(out_dir)
        .join(target_dir)
        .join(format!("{}.rs", filename));

    let target_parent = target_path.parent().expect("target_path had no parent");
    std::fs::create_dir_all(target_parent).expect("failed to create target_parent");

    bindings.write_to_file(&target_path).expect("failed to write bindings");
}
