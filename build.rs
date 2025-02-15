use std::path::PathBuf;

fn main() {
    build_imgui();
}

fn build_imgui() {
    // generate bindings
    let include_dir = "third_party/imgui";
    let target_dir = "bindings_imgui";

    generate_bindings(
        "third_party/imgui/imconfig.h",
        include_dir,
        target_dir,
    );
    generate_bindings(
        "third_party/imgui/imgui.h",
        include_dir,
        target_dir,
    );
    generate_bindings(
        "third_party/imgui/imgui_internal.h",
        include_dir,
        target_dir,
    );
    generate_bindings(
        "third_party/imgui/imstb_rectpack.h",
        include_dir,
        target_dir,
    );
    generate_bindings(
        "third_party/imgui/imstb_textedit.h",
        include_dir,
        target_dir,
    );
    generate_bindings(
        "third_party/imgui/imstb_truetype.h",
        include_dir,
        target_dir,
    );

    // compile and link
    cc::Build::new()
        .cpp(true)
        .include(include_dir)
        .file("third_party/imgui/imgui.cpp")
        .file("third_party/imgui/imgui_demo.cpp")
        .file("third_party/imgui/imgui_draw.cpp")
        .file("third_party/imgui/imgui_tables.cpp")
        .file("third_party/imgui/imgui_widgets.cpp")
        .flag_if_supported("-std=c++17")
        .compile("imgui");

    println!("cargo:rustc-link-lib=static=imgui");

    //if std::env::var("TARGET").unwrap().contains("windows-msvc") {
    //    println!("cargo:rustc-link-lib=dylib=stdc++");
    //}
}

fn generate_bindings(header: &str, include_dir: &str, target_dir: &str) {
    println!("cargo:rerun-if-changed={}", header);

    let bindings = bindgen::Builder::default()
        .header(header)
        .clang_arg(format!("-I{}", include_dir))
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
