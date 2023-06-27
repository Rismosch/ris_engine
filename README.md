# ris_engine

🏗️ **very WIP** 👷

Barebones game engine, based on a thread-pool-like job system.

![thumbnail](images/ris_engine_small.png "DALL·E: \"an expressive oil painting of an engine, burning is colourful pigments\"")

## ⚙️ Requirements

To compile this repo, you need a working Rust compiler. I currently use `rustc 1.66.0`. A newer one probably works fine.

The current target platform is Windows 64-bit. Other platform most likely wont work and I give no guarantees.

## 🔧 Installation

This engine is using various 3rd party libraries. Trying to compile it without these will most definitely result in diverse build, compile and linker errors.

In this repo you will find the `./external/` directory. It contains all required 3rd party libraries. To install them, follow these instructions:

1. Copy _EVERY_ `*.dll` in `./external/bin/` to the root of this repository.
2. Move `./external/bin/shaderc_shared.dll` to a desired directory and set the environment variable `SHADERC_LIB_DIR` to that directory.
3. Copy _EVERY_ `*.lib` in `./external/lib/` to the directory, which the linker searches for static libraries.  If you are on Windows, and are using `rustup`, this directory probably is:

> C:\\Users\\\<Your Username\>\\.rustup\\toolchains\\\<current toolchain\>\\lib\\rustlib\\\<current toolchain\>\\lib

For information where I got these libraries come from, read [external_sources.md](external_sources.md).


## 🔨 Building

Assuming everything is installed correctly, you can now simply compile and run the engine with:

    cargo run

Alternatively, you can build a release-ready package, by running the build script found under:

> ./CI/build_release.ps1


**Note:** The build script compiles with _ALL_ optimizations enabled, and thus may take much much longer than running `cargo run`.

The build script will generate building information, compile the entire workspace and move all required files into a single folder. Once executed, you will find the following files in `./release/`:

1. **ris_engine.exe**  
This is the compiled engine. It contains all logic to run the game.

2. **SDL2.dll**  
This contains all functionality of SDL2. It must always be found in the same directory as `ris_engine.exe`, otherwise you will get a runtime error.
