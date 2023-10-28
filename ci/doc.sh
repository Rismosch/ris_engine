#!/usr/bin/env bash

purpose="This script generates docs and moves them to another folder. \
This prevents \`cargo clean\` to delete the docs. In case the workspace doesn't compile, having the docs available is invaluable."

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)

source "$SCRIPT_DIR/util.sh"
pushd $ROOT_DIR

echo "creating documentation..."

cargo doc

if [ $? -eq 0 ]; then
    echo "cargo doc succeeded!"
    echo "clearing destination directory..."

    target_dir=''
    GetAndClearCiOutDir target_dir "$0"
    source_dir="$ROOT_DIR/target/doc"

    echo "copying documentation..."
    cp -r "$source_dir" "$target_dir/.."

    echo "done! final documentation can be found under \`$target_dir\`"
    echo "i recommend you save a bookmark to \`$target_dir/ris_engine/index.html\`"
else
    echo "cargo doc was unsuccessful"
fi
 

popd
