# ris_engine

Simple game engine, based on a thread-pool-like job system.

⚠️ **very WIP** ⚠️

## Installation

This engine is using SDL2. Trying to compile it without the required SDL2 libraries will most definitely result in a Linker error. Mine looks like this:

```
error: linking with `link.exe` failed: exit code: 1181
  |
  = note: lots and lots of gibberish
lots and lots of gibberish
lots and lots of gibberish
  = note: LINK : fatal error LNK1181: cannot open input file 'SDL2.lib'


error: could not compile `ris_engine` due to previous error
```


The current target platform is Windows x64. In this repo you will find the `./lib` directory, which contains all the required SDL2 libraries. Simply copy all `.lib` files to:

> C:\\Users\\{Your Username}\\.rustup\\toolchains\\{current toolchain}\\lib\\rustlib\\{current toolchain}\\lib

⚠️ I give no guarantees that this engine will work on other platforms ⚠️

But if you want to try anyway, you will probably require different libraries. Follow the install instructions [here](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries).

All SDL2 libraries in this repo come from the `SDL2-devel-2.24.0-VC.zip` package, which I downloaded [here](https://github.com/libsdl-org/SDL/releases/tag/release-2.24.0).

---

**Note:** `SDL2.dll` must be found in the same directory as the compiled `.exe`, otherwise you will encounter a runtime error. This error may be displayed in the console, or it may be a popup. Mine looks like this:

```
error: process didn't exit successfully: `target\debug\ris_engine.exe` (exit code: 0xc0000135, STATUS_DLL_NOT_FOUND)
```

## Building

🏗️ under construction 👷
