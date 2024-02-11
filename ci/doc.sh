#!/usr/bin/env bash

echo
echo "This script generates docs and moves them to another folder, thus \`cargo clean\` wont be able to delete them. This is very helpful in the situation that the workspace doesn't compile, which means the workspace is in a state where \`cargo doc\` will fail."
echo
echo
echo

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)

source "$SCRIPT_DIR/util.sh"
pushd $ROOT_DIR

echo "clearing destination directory..."
target_dir=''
GetAndClearCiOutDir target_dir "$0"

echo "asking for user input..."
cli_cargo_clean_value=false

read -p "should \`cargo clean\` be executed before creating docs? (y/N)" user_input
lower_user_input=$(echo $user_input | tr '[:upper:]' '[:lower:]')
if [[ $lower_user_input == "y" ]]; then
    cli_cargo_clean_value=true
fi

if [ "$cli_cargo_clean_value" = true ]; then
    echo "cleaning workspace..."
    cargo clean
fi

echo "creating documentation..."

cargo doc

if [ $? -eq 0 ]; then
    echo "cargo doc succeeded!"

    source_dir="$ROOT_DIR/target/doc"

    echo "copying documentation..."
    cp -r "$source_dir" "$target_dir/.."

    echo "done! final documentation can be found under \`$target_dir\`"
    echo "i recommend you save a bookmark to \`$target_dir/ris_engine/index.html\`"
else
    echo "cargo doc was unsuccessful"
fi
 

popd
