# ris_engine

Barebones game engine. Home made passion project.

![thumbnail](screenshot.png)

## Features:

- [x] Startup, shutdown, mainloop and error handling
- [x] Logging, to console and file
- [x] Threadpool based concurrency
- [x] Remappable controls
  - [x] Mouse
  - [x] Keyboard
  - [x] Gamepad
- [x] 3d math
  - [x] Vectors and Matrices
  - [x] Quaternions
  - [x] Color
    - [x] RGB
    - [x] OkLab
- [x] Basic 3d renderer via Vulkan
- [x] Debugging
  - [x] GUI via Dear ImGui
  - [x] Profiling
  - [x] Gizmos
  - [x] const hashed string ids
- [x] Asset System
  - [x] Importing (convert raw assets to usable form)
  - [x] Loading (use in engine)
  - [x] (De)compiling
- [x] Codecs
  - [x] GLSL to SpirV, with custom pre processor
  - [x] QOI
  - [ ] glTF
- [x] Settings/Configuration
- [x] Gameobjects and components
  - [x] Mesh renderer
    - [ ] Materials
  - [x] Scripting
- [ ] Scene editing, saving and loading
- [ ] Collisions
- [ ] Animations
- [ ] 3d Sound

**Legend**:
- [x] implemented
- [ ] planned

## Requirements

|          |                          | Notes                                                    |
| -------- | ------------------------ | -------------------------------------------------------- |
| Compiler | rustc 1.77.2             | [Download Link](https://www.rust-lang.org/tools/install) |
| Platform | x86_64 Windows and Linux | may or may not compile on other platforms                |
| Graphics | Vulkan capable Hardware  |                                                          |

You also require an internet connection, to download dependencies from [crates.io](https://crates.io/). You can [vendor](https://doc.rust-lang.org/cargo/commands/cargo-vendor.html) crates for offline use or download an archived repo from [my website](https://www.rismosch.com/archive). Note that I make these archives sporadically, meaning they may not be up to date.

## Installation

This engine is using various 3rd party libraries. Trying to build without these will most definitely result in diverse compile, linker and runtime errors. Click to reveal the instructions for the given platform.

<details>
  <summary>Windows</summary>

  ### Windows
  
  In this repo you will find the `./external/` directory. It contains all required libraries. If you don't want to use the binaries in this repo, you can install the Vulkan SDK, which provides binaries for `SDL2` and `shaderc`.
  
  #### 1. Copy _EVERY_ `*.dll` in `./external/bin/` to the root of this repo.
  
  These DLLs need to be available in your environment. So either assign it to your environment variables or move them to the root of the directory.
  
  #### 2. Set the environment variable `SHADERC_LIB_DIR`
  
  [shaderc](https://crates.io/crates/shaderc) requires the DLL `shaderc_shared.dll` during build time. `shaderc` allows to store and compile shader code inside Rust source files. `ris_engine` does not use this feature, but `shaderc` requires this dependency nonetheless. It searches the DLL in `SHADERC_LIB_DIR`.
  
  For more info, check this link: https://docs.rs/shaderc/0.8.3/shaderc/index.html
  
  So, if `shaderc_shared.dll` sits inside directory `/path/to/shaderc/`, then set `SHADERC_LIB_DIR` to `/path/to/shaderc/`. If you don't want to move the DLL, you can simply set `SHADERC_LIB_DIR` to `<path of this repo>/external/bin/`.
  
  #### 3. Copy _EVERY_ `*.lib` in `./external/lib/` to
  
  ```powershell
  C:\Users\<your username>\.rustup\toolchains\<current toolchain>\lib\rustlib\<current toolchain>\lib
  ```
  
  Rust still needs to link. If you are using `rustup`, the linker will search for LIBs in the directory above. If you are not using `rustup`, you must figure out how to link against the required LIBs.
</details>

<details>
  <summary>Arch Linux</summary>
  
  ### Arch Linux
  
  #### 1. Install [SDL2](https://archlinux.org/packages/extra/x86_64/sdl2/)
  
  ```bash
  sudo pacman -S sdl2
  ```
  
  #### 2. Install [shaderc](https://archlinux.org/packages/extra/x86_64/shaderc/)
  
  ```bash
  sudo pacman -S shaderc
  ```
  
  #### 3. Install [Vulkan](https://wiki.archlinux.org/title/Vulkan)
  
  Depending on your graphics card, you need to install a different package. Follow the instructions in the link below:
  
  https://wiki.archlinux.org/title/Vulkan#Installation
</details>

## Building

Assuming everything is installed correctly, you can now compile and run the engine with:

```bash
cargo run
```

Alternatively, you can build a release-ready package, by running the command below. Note that this builds with all optimizations enabled, which may take longer than just using `cargo run`.

```bash
cargo run -p cli build
```

Passing the `-r` flag is discouraged, because asset discovery works differently in release builds. If you want to pass the `-r` flag to cargo, you must import and compile the assets manually. To do so, run the following two commands:

```bash
cargo run -p cli asset import
cargo run -p cli asset compile
```

After compiling, you will find the file `ris_assets` in the root of this repo. It contains all assets used by `ris_engine`.

## Testing

All tests are found under `./tests/` and can be run with:

```bash
cargo test
```

Alternatively, to run **much** more extensive tests, you can run the command below. Note that this may take several minutes.

```bash
cargo run -p cli pipeline all
```

Using the command above, some tests run [miri](https://github.com/rust-lang/miri). If miri is not installed, then the according tests will fail.

## Cli

For more info about the command `cargo run -p cli`, see [`./cli/README.md`](./cli/README.md).
