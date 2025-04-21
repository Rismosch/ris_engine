# assets

Before an asset can be used by ris_engine, it must go through the asset pipeline, which consists of these steps:

1. Source file
2. Imported
3. In use
4. Compiled

## 1 Source file

Most assets start as a source file. ris_engine cannot load source files. But they are kept to allow easy modifications in the future.

## 2 Imported

To use a source file, it must be imported. To kick off the import process, run:

    cargo run -p cli asset import

All imported assets are written to the `./assets/imported/` directory. If a target directory doesn't exist, it will be generated.

## 3 In use

To use an imported file, it must be copied to `./assets/in_use/` or in any of its subdirectories. This step exists to allow the user to pick and choose which assets ris_engine should or should not be aware of. Chances are that the import process generates undesirable names, and chances are, that the user may not want to use all imported assets. The latter may be the case when assets are removed, but still reside in `./assets/imported/` because they weren't cleaned up.

Copying the files to `./assets/in_use/` can be done manually, but it can also be achieved automatically by using meta files:

A meta file has the extension `ris_meta` and is stored inside `./assets/imported/`. The contents of a meta file consist of a single path which is relative to `./assets/in_use/`. After all source files have been imported (or have been attempted to), a copy process reads all the meta files and copies the imported files.

For example the meta file

    ./assets/imported/path/to/my/file.extension.ris_meta

with the contents

    copy_to: new/path

copies the file

    ./assets/imported/path/to/my/file.extension

to

    ./assets/in_use/new/path/file.extension

## 4 Compiled

`./assets/in_use/` contains all assets that ris_engine can load. These assets are kept in a directory structure for the ease of browsing. But this isn't ideal, as this requires interaction with the file system on each access. To allow optimal performance, all used assets can be compiled into a single file.

To compile the assets, run:

    cargo run -p cli asset compile

This command recursively iterates through `./assets/in_use/` and creates a file called `./ris_assets`, which contains all compiled assets.

To decompile a compiled asset file back into a directory structure, you can run:

    cargo run -p cli asset decompile

## Asset discovery

In a debug build, ris_engine attempts to locate the assets in `./assets/in_use/`.  
In a release build, ris_engine attempts to locate the assets in `./ris_assets`.  

This can be overwritten by passing `--assets <path>` as cli args to ris_engine. `<path>` can either be a directory or filepath. This means, even though a given build defaults to either a directory or compiled structure, ris_engine can actually load both, regardless of build. ris_engine will chose the apropriate asset loader during runtime.
