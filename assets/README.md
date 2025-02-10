# assets

Before an asset can be used by `ris_engine`, it must go through the asset pipeline, which consists of these steps:

1. Source file
2. Imported
3. In use
4. Compiled

## 1 Source file

Most assets start as a source file. These are kept, such that assets can be modified in the future. While this is handy, such source files aren't optimized: Most often they aren't compressed and are slow to load. Because of this, `ris_engine` cannot directly load source files.

## 2 Imported

To use an asset from a source file, it must be imported first. This can be achieved by running `ris_engine` as a debug build with the appropriate settings, or by running:

    cargo run -p cli asset import

All imported assets are written in the `./assets/imported/` directory. If the directory doesn't exist, the import process will generate it.

## 3 In use

To use an imported file, it must be copied to `./assets/in_use/` or in any of its subdirectories. This can be done manually, but it can also be achieved automatically by using meta files.

A meta file inside `./assets/imported/` stores a path relative to `./assets/in_use/`. After all source files have been imported (or have been attempted to), a copy process reads all the meta files and copies the imported files.

For example the meta file

    ./assets/imported/path/to/my/file.extension.meta

with the contents

    new/path

copies the file

    ./assets/imported/path/to/my/file.extension

to

    ./assets/in_use/new/path/file.extension

## 4 Compiled

`./assets/in_use/` are all the assets that can actually be loaded into `ris_engine`. These assets are kept in a directory structure because of the ease of browsing. But this isn't ideal, as `ris_engine` must interact with the file system to access these assets. Because of that, assets can be compiled for optimal file system access. To compile the assets, run:

    cargo run -p cli asset compile

This command recursively iterates through `./assets/in_use/` and creates `./ris_assets`, which stores all assets as a single file.

To decompile a compiled asset file, you can run:

    cargo run -p cli asset decompile

## Asset discovery

In a debug build, `ris_engine` attempts to locate the assets in `./assets/in_use/`.  
In a release build, `ris_engine` attempts to locate the assets in `./ris_assets`.  
This can be overwritten by passing `--assets <filepath>` to `ris_engine`. `<filepath>` can either be a directory, as described in [3. In use](#3-In-use), or a compiled asset, as described in [4. Compiled](#4-Compiled).