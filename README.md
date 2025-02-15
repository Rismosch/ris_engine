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
- [x] Scene editing, saving and loading
- [ ] Collisions
- [ ] Animations
- [ ] 3d Sound

**Legend**:
- [x] implemented
- [ ] planned

## Requirements

The target platform is x86_64. ris_engine compiles both on Windows and Linux. Other platforms may work, but I haven't tested platforms outside of these.

Your hardware must support [Vulkan](https://www.vulkan.org/). Most modern GPUs work.

You also require an internet connection, to download dependencies from [crates.io](https://crates.io/). You can [vendor](https://doc.rust-lang.org/cargo/commands/cargo-vendor.html) crates for offline use or download an archived repo from [my website](https://www.rismosch.com/archive). Note that I make these archives sporadically, meaning they may not be up to date.


## Setup

This engine is using various 3rd party libraries. While most are provided via [crates.io](https://crates.io/), some require prebuild binaries. Trying to build without these will most definitely result in diverse compile, linker and runtime errors.

You will need binaries for:
- [SDL2](https://www.libsdl.org/)
- [Shaderc](https://github.com/google/shaderc)

For information on how to get and install these, click to reveal the instructions for the given platform.

### Windows

<details>
  <summary>click to reveal</summary>

  The easies way to get the required binaries is to install the [Vulkan SDK](https://vulkan.lunarg.com/). The installation of the Vulkan SDK should also configure your environment correctly. To see whether the Vulkan SDK is installed properly, check if the environment variables `$VULKAN_SDK` and `$VK_SDK_PATH` are set, and check if they pointing to the location of your Vulkan SDK installation.

  If you don't want to use the Vulkan SDK, you must download the required binaries seperately.

  All further instructions assume `$VULKAN_SDK` is properly set.

  #### SDL2

  SDL2 needs to link statically. Assuming you are using `rustup`, copy `$VULKAN_SDK\Lib\SDL2.lib` into the following directory:

  ```powershell
  C:\Users\<your username>\.rustup\toolchains\<toolchain channel>\lib\rustlib\<current toolchain>\lib
  ```

  If you are not using `rustup`, you need to figure out how to link against `$VULKAN_SDK\Lib\SDL2.lib`.

  SDL2 also needs `$VULKAN_SDK\Bin\SDL2.dll` at runtime. There are many ways on how to accomplish this. One way is to copy the DLL to the root of this repo. But I recommend you to add `$VULKAN_SDK\Bin` to your environemnt variables. The Vulkan SDK provides useful tools that you may want in your environment.

  #### Shaderc

  When you are using a properly installed Vulkan SDK, Shaderc should work out of the box.

  If you are not using the Vulkan SDK, you must configure Shaderc manually. Here are instructions on the needed setup: https://github.com/google/shaderc-rs/blob/4e0441563a49009a24916a0e4c6577532bf990a0/README.md#setup

</details>

### Arch Linux

<details>
  <summary>click to reveal</summary>
  
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

Passing the `-r` flag is discouraged, because asset discovery works differently in release builds. If you want to pass the `-r` flag to cargo, you must import and compile the assets manually. Infos and how to do can be found in [`./assets/README.md`](./assets/README.md).

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
