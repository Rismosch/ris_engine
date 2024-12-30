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
  
  In this repo you will find the `./external/` directory. It contains all required binaries. To install them, simply run `./INSTALL.ps1`.
  
  If you don't trust the prebuild binaries, which is understandable, the steps below instruct you on how to manually set up your environment manually. If you use the `./INSTALL.ps1` script, you can skip this rest of this section.

  #### 1. Download the necessary dependencies

  You need DLLs and LIBs for [SDL2](https://www.libsdl.org/) and [Shaderc](https://github.com/google/shaderc). I recommend getting them by installing the [Vulkan SDK](https://vulkan.lunarg.com/).

  #### 2. Make the LIBs available for your linker

  Rust still needs to link. The four LIBs you need are:
   - `SDL2.lib`
   - `SDL2_test.lib`
   - `SDL2main.lib`
   - `shaderc_shared.lib`

  If you are using `rustup`, the linker will search for LIBs in the directory below. Copy the required LIBs into this directory.

  ```powershell
  C:\Users\<your username>\.rustup\toolchains\<current toolchain>\lib\rustlib\<current toolchain>\lib
  ```

  If you are not using `rustup`, you need to figure out how to link the required LIBs.

  #### 3. Make the DLLs available in your environment

  The two DLLs you need are
  - `SDL2.dll`
  - `shaderc_shared.dll`.
  
  The easiest way to make them available in your environment is to copy them to the root of this repo. This isn't recommended however, because it doesn't help for installation step 4. Also they aren't tracked by git and thus are deleted when running `git clean` or any command to restore this repo to its initial state.
  
  I recommend adding required directories to the `PATH` environment variable. The commands below may take a few seconds to execute.
  
  If you have installed the Vulkan SDK, you can copy and paste the following commands:

  ```powershell
  $oldPath = [Environment]::GetEnvironmentVariable("PATH", "User"); `
  $newPath = $oldPath + ";$env:VK_SDK_PATH\Bin"; `
  [Environment]::SetEnvironmentVariable("PATH", $newPath, "User");
  ```

  If you have not installed the Vulkan SDK, or downloaded the DLLs seperately, you can modify the commands like below. Make sure to use the actual paths to your directories.

  ```powershell
  $oldPath = [Environment]::GetEnvironmentVariable("PATH", "User"); `
  $newPath = $oldPath + ";C:\path\to\SDL2\bin"; `
  $newPath = $newPath + ";C:\path\to\shaderc\bin"; `
  [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
  ```

  #### 4. Assign `SHADERC_LIB_DIR`

  [Shaderc](https://crates.io/crates/shaderc) requires the DLL `shaderc_shared.dll` during build time. shaderc allows to store and compile shader code inside Rust source files. `ris_engine` does not use this feature, but Shaderc requires this dependency regardless. It searches the DLL in `SHADERC_LIB_DIR`. If this variable is not set, Shaderc will try to compile from source, which is quite slow requires you to have a C++ build tools available in your environment.
  
  For more info, check this link: https://docs.rs/shaderc/0.8.3/shaderc/index.html
  
  I recommend just setting the environment variable. To create it, run one of the the following powershell commands below. Again, these may take a few seconds to execute.
  
  If you have the Vulkan SDK installed:

  ```powershell
  [Environment]::SetEnvironmentVariable("SHADERC_LIB_DIR", "$env:VK_SDK_PATH\Lib", "User")
  ```

  If you have not the Vulkan SDK installed, make sure to use the actual path to your directory:
  
  ```powershell
  [Environment]::SetEnvironmentVariable("SHADERC_LIB_DIR", "C:\path\to\shaderc\bin", "User")
  ```
  
  #### 5. Restart your terminal

  Make sure to restart your terminals, such that the changes to your environment variables take effect.
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
