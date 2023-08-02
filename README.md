# ris_engine

🏗️ **Don't expect this repo to be stable, as I am constantly pushing breaking changes.** 👷

![thumbnail](images/ris_engine_small.png "DALL·E: \"an expressive oil painting of an engine, burning is colourful pigments\"")


## ⚙️ Requirements

To compile this repo, you need a working Rust compiler. I currently use `cargo` and `rustc 1.70.0`. A newer compiler probably works fine.

The current target platform is Windows 64-bit. Other platforms probably wont work and I give no guarantees.


## 🔧 Installation

This engine is using various 3rd party libraries. Trying to build without these will most definitely result in diverse compile and linker errors.

In this repo you will find the `./3rd_party/` directory. It contains all required 3rd party libraries.

For information where I got these libraries from, read [3RD_PARTY_SOURCES.md](3RD_PARTY_SOURCES.md).

### Instructions

#### 1. Copy _EVERY_ `*.dll` in `./3rd_party/bin/` to the root of this repository.

`cargo run` expects all necessary dlls to be in the root directory. Also, the `./ci/build.ps1` script expects these to be in the root directory as well.

#### 2. Move `./3rd_party/bin/shaderc_shared.dll` to a desired directory and set the environment variable `SHADERC_LIB_DIR` to that directory.

[shaderc](https://crates.io/crates/shaderc) requires this dll during build time. It has a niche feature to store shader code in Rust source code, and compile them at build time using macros. Gimmicky and probably intended for small demo projects, but useless bloat if you ask me.

Nevertheless it does try to try to search the dll, and one directory it's searching is the environment variable `SHADERC_LIB_DIR`. 

#### 3. Copy _EVERY_ `*.lib` in `./3rd_party/lib/` to the directory, which the linker searches for static libraries.

If you are on Windows, and are using `rustup`, this directory probably is:

    C:\Users\<your username>\.rustup\toolchains\<current toolchain>\lib\rustlib\<current toolchain>\lib

Rust still needs to link. And this directory is the one that `cargo` searches for libraries.


## 🔨 Building

Assuming everything is installed correctly, you can now simply compile and run the engine with:

    cargo run

Alternatively, you can build a release-ready package, by running the build script found under:

    ./ci/build.ps1


The build script will generate building information, compile the entire workspace and move all required files into a single folder. Once executed, you will find the following files in `./ci_out/build/`:

1. **ris_engine.exe**  
This is the compiled engine. It contains all logic to run the game.

2. **SDL2.dll**  
This is a multi media library, which provides low level access to audio, keyboard, mouse, joystick and graphics.

3. **shaderc_shared.dll**  
This library is a compiler, which compiles [GLSL](https://www.khronos.org/opengl/wiki/Core_Language_(GLSL))/[HLSL](https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl) to [SPIR-V](https://www.khronos.org/spir/).
