# ris_engine

Barebones game engine. Home made passion project. 

🏗️ **VERY WIP** 👷

![thumbnail](raw_assets/images/ris_engine_small.png "Generated by DALL·E - Prompt: \"an expressive oil painting of an engine, burning is colourful pigments\"")

---

## Features:

- [x] Startup, shutdown, mainloop and error handling
- [x] Logging, to console and file
- [x] Threadpool based concurrency
- [x] Remappable controls
  - [x] Mouse
  - [x] Keyboard
  - [x] Gamepad
- [x] 3d math
- [x] Basic Vulkan renderer
  - [x] Vertex and index buffers
  - [x] Depth and Stencil buffer
  - [ ] Texture sampling
  - [ ] Phong shading
  - [x] Hotswappable shaders during runtime
- [x] Asset System
  - [x] Importing (convert raw assets to usable form)
  - [x] Loading (use in engine)
  - [x] (De)compiling
- [x] Global mutable state
  - [x] Settings/Configuration
  - [ ] Gameobjects
- [ ] Debug GUI
  - [ ] Labels
  - [ ] Buttons
  - [ ] Input fields
- [ ] Debug gizmos
  - [ ] Point
  - [ ] Line/ray
  - [ ] Sphere
  - [ ] Bounding box
  - [ ] Text
- [ ] Audio

---

## Requirements

To compile this repo, you need a working Rust compiler. I recommend installing via [rustup](https://www.rust-lang.org/tools/install).

The current target platform is x86_64, both Windows and Linux.

You also require an internet connection, to download dependencies from [crates.io](https://crates.io/). If you have an internet connection, you have everything you need and you can jump straight to [Installation](#Installation). If you do not have access to the internet, choose one of the following methods:

### Method 1: cargo vendor

While you have internet access, you can download all dependencies with the following command:

    cargo vendor

This should generate the directory `./vendor/`, which contains all necessary packages.

Then create the file `./cargo/config.toml` with this content:

    [source.crates-io]
    replace-with = "vendored-sources"
    
    [source.vendored-sources]
    directory = "vendor"

This tells `cargo` to use the packages in `./vendor/` instead of searching the internet.

For more information on `cargo vendor` check the following link: https://doc.rust-lang.org/cargo/commands/cargo-vendor.html

### Method 2: Get an archived repo

An archived repo contains all required packages. You can get one from my website:

https://www.rismosch.com/archive

Note that I make these archives sporadically, meaning they may not be up to date. Check the date, when the archives have been generated.

---

## Installation

This engine is using various 3rd party libraries. Trying to build without these will most definitely result in diverse compile, linker and runtime errors. Depending on your platform, follow the instructions below.

### Windows

In this repo you will find the   `./3rd_party/` directory. It contains all required libraries for Windows.

For information where I got these from, read [./3rd_party/README.md](3rd_party/README.md). 

#### 1. Copy _EVERY_ `*.dll` in `./3rd_party/bin/` to the root of this repository.

`cargo run` expects all necessary DLLs to be in the root directory. Also, the `./ci/build.ps1` script expects these to be in the root directory as well.

#### 2. Set the environment variable `SHADERC_LIB_DIR` to `./3rd_party/bin/`

[shaderc](https://crates.io/crates/shaderc) requires the DLL `shaderc_shared.dll` during build time. It has a feature, which stores and compiles shader code inside Rust source files. `ris_engine` does not use this feature, but nevertheless, `shaderc` does try to search the DLL while building. The environment variable `SHADERC_LIB_DIR` is the directory where `shaderc` searches for the DLL.

#### 3. Copy _EVERY_ `*.lib` in `./3rd_party/lib/` to the directory, which the linker searches for static libraries.

If you are using `rustup`, this directory probably is:

    C:\Users\<your username>\.rustup\toolchains\<current toolchain>\lib\rustlib\<current toolchain>\lib

Rust still needs to link. And this directory is the one that `cargo` searches for libraries.

---

### Linux

Examples use the `pacman` package manager from Arch.

#### 1. Install [SDL2](https://archlinux.org/packages/extra/x86_64/sdl2/)

    sudo pacman -S sdl2

#### 2. Install [shaderc](https://archlinux.org/packages/extra/x86_64/shaderc/)

    sudo pacman -S shaderc

#### 3. Install [Vulkan](https://wiki.archlinux.org/title/Vulkan)

Depending on your graphics card, you need to install a different package. Follow the instructions in the link below:

https://wiki.archlinux.org/title/Vulkan#Installation

---

## Building

Assuming everything is installed correctly, you can now compile and run the engine with:

    cargo run

Alternatively, you can build a release-ready package, by running a build script found under `./ci/`.

Windows:

    ./ci/build.ps1

Linux:

    bash ./ci/build.sh

The build script will generate building information, compile the entire workspace and move all required files into a single folder. Once executed, you will find the following files in `./ci_out/build/`:

1. **ris_engine.exe**  
   This is the compiled engine. It contains all logic to run the game.

2. **ris_assets**  
   This file contains all assets used by the engine. Without assets, the game cannot be run.
   
3. **SDL2.dll** (only on windows)  
   This is a multi media library, which provides low level access to audio, keyboard, mouse, joystick and windowing.

## Testing

All tests are found under `./tests/suite/` and can be run with:

    cargo test

If you have [miri](https://github.com/rust-lang/miri) installed, instead tests can be run with:

    cargo miri test

Running tests with `miri` is significantly slower and may take a few minutes. Also note that you need to switch to a nightly toolchain. At the time of writing, `ris_engine` only compiles and runs on a stable toolchain; use nightly only for `miri`. Assuming stable and nightly toolchains are installed, switching toolchains can be achieved by running:

    rustup override set <toolchain>

Where `<toolchain>` is either `stable` or `nightly`.
