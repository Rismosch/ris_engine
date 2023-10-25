# ris_engine

Barebones game engine. Home made passion project. 

🏗️ **VERY WIP** 👷

![thumbnail](images/ris_engine_small.png "DALL·E: \"an expressive oil painting of an engine, burning is colourful pigments\"")

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
- [ ] Gameobjects
- [ ] Audio

---

## Requirements

To compile this repo, you need a working Rust compiler. I recommend installing it via [rustup](https://www.rust-lang.org/tools/install).

The current target platform is x86_64, both Windows and Linux.

You also require an internet connection, to download dependencies from [crates.io](https://crates.io/). If you have an internet connection, you have everything you need and you can jump straight to [Installation](#Installation). If you do not have access to the internet, continue reading this section.

You can get an archived repo on [my website archive](https://www.rismosch.com/archive), which contains all required packages. Alternatively, while you have internet access, you can download all dependencies with the following command:

    cargo vendor

This should generate the directory `./vendor/`, which contains all necessary packages.

Then create the file `./cargo/config.toml` with this content:

    [source.crates-io]
    replace-with = "vendored-sources"
    
    [source.vendored-sources]
    directory = "vendor"

This tells `cargo` to use the packages in `./vendor/` instead of searching the internet.

For more information on `cargo vendor` check the following link: https://doc.rust-lang.org/cargo/commands/cargo-vendor.html

---

## Installation

This engine is using various 3rd party libraries. Trying to build without these will most definitely result in diverse compile, linker and runtime errors. Depending on your platform, follow the instructions below.

### Windows

In this repo you will find the   `./3rd_party/` directory. It contains all required 3rd party libraries for Windows.

For information where I got these libraries from, read [./3rd_party/README.md](3rd_party/README.md). 

#### 1. Copy _EVERY_ `*.dll` in `./3rd_party/bin/` to the root of this repository.

`cargo run` expects all necessary dlls to be in the root directory. Also, the `./ci/build.ps1` script expects these to be in the root directory as well.

#### 2. Move `./3rd_party/bin/shaderc_shared.dll` to a desired directory and set the environment variable `SHADERC_LIB_DIR` to that directory.

[shaderc](https://crates.io/crates/shaderc) requires this dll during build time. It has a niche feature to store shader code in Rust source code, and compile them at build time using macros. Gimmicky and probably intended for small demo projects, but useless bloat if you ask me.

Nevertheless, it does try to search the dll, and the environment variable `SHADERC_LIB_DIR` is one directory where it's searching for it. 

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

Assuming everything is installed correctly, you can now simply compile and run the engine with:

    cargo run

Alternatively, you can build a release-ready package, by running a build script found under `./ci/`.

Windows:

    ./ci/build.ps1

Linux:

    TODO

The build script will generate building information, compile the entire workspace and move all required files into a single folder. Once executed, you will find the following files in `./ci_out/build/`:

1. **ris_engine.exe**  
   This is the compiled engine. It contains all logic to run the game.

2. **SDL2.dll**  
   This is a multi media library, which provides low level access to audio, keyboard, mouse, joystick and windowing.

3. **ris_assets**  
   This file contains all assets used by the engine. Without assets, the game cannot be run.
