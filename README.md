# ris_engine

very WIP

## Installation

This engine is using SDL2. Trying to compile it without the required SDL2 libraries will most definitely result in a Linker error.

The current target platform is Windows x64. The `./lib` directory contains all required SDL2 libraries. Simply copy all `*.lib` files to

> C:\\Users\\{Your Username}\\.rustup\\toolchains\\{current toolchain}\\lib\\rustlib\\{current toolchain}\\lib

I give no guarantees that this engine will work on other platforms. But if you want to try, check the install instructions [here](https://github.com/Rust-SDL2/rust-sdl2).

The libraries in this repo come from `SDL2-devel-2.24.0-VC.zip` package: [Source](https://github.com/libsdl-org/SDL/releases/tag/release-2.24.0)

`SDL2.dll` must be found in the same directory as the compiled `.exe`, otherwise you will encounter a runtime error that looks something like this:

> error: process didn't exit successfully: `target\debug\ris_engine.exe` (exit code: 0xc0000135, STATUS_DLL_NOT_FOUND)

If you execute this engine with `cargo run`, then `SDL2.dll` must be found in the root of this repo. However, the build script `build.rs` _should_ do this automatically. If for whatever reason `build.rs` isn't run, simply copy `SDL2.dll` manually to the root of this repo