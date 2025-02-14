fn main() {
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-search=native=external/SDL2/lib/x64");
}
