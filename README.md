# ris_engine

Simple game engine, based on a thread-pool-like job system.

⚠️ **very WIP** ⚠️

## 🔧 Installation

This engine is using SDL2. Trying to compile it without the required SDL2 libraries will most definitely result in a Linker error.

The current target platform of this engine is Windows. I don't guarantee that it works on another platform, but if you want to try anyway, I recommend to follow the install instructions [here](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries).

But assuming you are on windows, and are using `rustup`, follow these instructions:

1. In this repo you will find the `./SDL2-2.0.12` directory. It contains all the required SDL2 libraries. If your OS and toolchain are 64 bit, choose the libraries in the `x64` directory. If they are 32 bit, choose the libraries in the x86 directory.
2. Copy `SDL2.dll` to the root of this repository
3. Copy **ALL** `.lib` files to:

> C:\\Users\\{Your Username}\\.rustup\\toolchains\\{current toolchain}\\lib\\rustlib\\{current toolchain}\\lib

_All SDL2 libraries in this repo come from the `SDL2-devel-2.0.12-VC.zip` package, which I downloaded [here](https://github.com/libsdl-org/SDL/releases/tag/release-2.0.12)._

## 🔨 Building

Assuming SDL2 is installed correctly (see section above), you can now simply compile and run the engine with:

```
cargo run
```

Alternatively, you can build a release-ready package, by running the build script found under:

```
./CI/build_release.ps1
```

**Note:** The build script compiles with _ALL_ optimizations enabled, and thus may take much longer than running `cargo run`.

The build script will generate building information, compile the entire workspace and move all required files into a single folder. Once executed, you will find the following files in `./CI/build/`:

1. **app.exe**  
This is a simple wrapper program to call `ris_engine.exe`. It forwards command args and allows the engine to restart itself. Prefer to run this instead of `ris_engine.exe`.

2. **ris_engine.exe**  
This is the compiled engine. It contains all logic to run the game.

3. **SDL2.dll**  
This contains all functionality of SDL2. It must always be found in the same directory as `ris_engine.exe`, otherwise you will get a runtime error.
