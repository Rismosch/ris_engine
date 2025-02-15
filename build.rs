use std::path::PathBuf;

fn main() {
    build_imgui();
}

fn build_imgui() {
    let imgui_dir = "third_party/imgui";

    // generate bindings
    let target_name = "bindings_imgui";

    generate_bindings(format!("{}/imconfig.h", imgui_dir), imgui_dir, target_name);
    generate_bindings(format!("{}/imgui.h", imgui_dir), imgui_dir, target_name);
    generate_bindings(
        format!("{}/imgui_internal.h", imgui_dir),
        imgui_dir,
        target_name,
    );
    generate_bindings(
        format!("{}/imstb_rectpack.h", imgui_dir),
        imgui_dir,
        target_name,
    );
    generate_bindings(
        format!("{}/imstb_textedit.h", imgui_dir),
        imgui_dir,
        target_name,
    );
    generate_bindings(
        format!("{}/imstb_truetype.h", imgui_dir),
        imgui_dir,
        target_name,
    );

    // compile and link
    cc::Build::new()
        .cpp(true)
        .include(imgui_dir)
        .file(format!("{}/imgui.cpp", imgui_dir))
        .file(format!("{}/imgui_demo.cpp", imgui_dir))
        .file(format!("{}/imgui_draw.cpp", imgui_dir))
        .file(format!("{}/imgui_tables.cpp", imgui_dir))
        .file(format!("{}/imgui_widgets.cpp", imgui_dir))
        .flag_if_supported("-std=c++17")
        .compile("imgui");

    println!("cargo:rustc-link-lib=static=imgui");

    //if std::env::var("TARGET").unwrap().contains("windows-msvc") {
    //    println!("cargo:rustc-link-lib=dylib=stdc++");
    //}
}

fn generate_bindings(
    header: impl AsRef<str>,
    include_dir: impl AsRef<str>,
    target_name: impl AsRef<str>,
) {
    let header = header.as_ref();
    let include_dir = include_dir.as_ref();
    let target_name = target_name.as_ref();

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
        .expect("failed to convert OsStr to str");

    let out_dir = std::env::var("OUT_DIR").expect("failed to resolve env OUT_DIR");
    let target_path = PathBuf::from(out_dir)
        .join(target_name)
        .join(format!("{}.rs", filename));

    let target_parent = target_path.parent().expect("target_path had no parent");
    std::fs::create_dir_all(target_parent).expect("failed to create target_parent");

    bindings
        .write_to_file(&target_path)
        .expect("failed to write bindings");
}
