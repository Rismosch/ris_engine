# ris_engine

🏗️ **Don't expect this repo to be stable, as I am constantly pushing breaking changes.** 👷

![thumbnail](images/ris_engine_small.png "DALL·E: \"an expressive oil painting of an engine, burning is colourful pigments\"")

## ⚙️ Requirements

To compile this repo, you need a working Rust compiler. I currently use `cargo` and `rustc 1.70.0`. A newer compiler probably works fine.

The current target platform is Windows 64-bit. Other platforms probably wont work and I give no guarantees.

## 🔧 Installation

This engine is using various 3rd party libraries. Trying to build without these will most definitely result in diverse compile and linker errors.

In this repo you will find the `./external/` directory. It contains all required 3rd party libraries. To install them, follow these instructions:

1. Copy _EVERY_ `*.dll` in `./external/bin/` to the root of this repository.
2. Move `./external/bin/shaderc_shared.dll` to a desired directory and set the environment variable `SHADERC_LIB_DIR` to that directory.
3. Copy _EVERY_ `*.lib` in `./external/lib/` to the directory, which the linker searches for static libraries.  If you are on Windows, and are using `rustup`, this directory probably is:

> C:\\Users\\\<your username\>\\.rustup\\toolchains\\\<current toolchain\>\\lib\\rustlib\\\<current toolchain\>\\lib

For information where I got these libraries come from, read [external_sources.md](EXTERNAL_SOURCES.md).


## 🔨 Building

Assuming everything is installed correctly, you can now simply compile and run the engine with:

    cargo run

Alternatively, you can build a release-ready package, by running the build script found under:

> ./ci/build.ps1


The build script will generate building information, compile the entire workspace and move all required files into a single folder. Once executed, you will find the following files in `./build/`:

1. **ris_engine.exe**  
This is the compiled engine. It contains all logic to run the game.

2. **SDL2.dll**  
This is a multi media library, which provides low level access to audio, keyboard, mouse, joystick and graphics.

3. **shaderc_shared.dll**  
This library is a compiler, which compiles [GLSL](https://www.khronos.org/opengl/wiki/Core_Language_(GLSL))/[HLSL](https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl) to [SPIR-V](https://www.khronos.org/spir/).
