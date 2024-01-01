#!/usr/bin/env bash

purpose="This script generates docs and moves them to another folder. \
This prevents \`cargo clean\` to delete the docs. In case the workspace doesn't compile, having the docs available is invaluable."

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)

source "$SCRIPT_DIR/util.sh"
pushd $ROOT_DIR

echo "parsing cli args..."
cli_default="--default"

cli_cargo_clean="--cargo-clean"
cli_no_cargo_clean="--no-cargo-clean"
cli_cargo_clean_value=false

if [ $# -eq 0 ]; then
    echo
    echo $purpose
    echo
    echo "INFO: you may skip user input, by providing cli args."
    echo
    echo "available args:"
    echo "    $cli_default         skips user input and uses default values for everything below"
    echo ""
    echo "    $cli_cargo_clean     executes \`cargo clean\` before creating docs"
    echo "    $cli_no_cargo_clean  does not execute \`cargo clean\` (default)"
    echo ""
    echo ""
    echo ""
    echo ""
    echo ""

    read -p "should \`cargo clean\` be executed before creating docs? (y/N)" user_input
    lower_user_input=$(echo $user_input | tr '[:upper:]' '[:lower:]')
    if [[ $lower_user_input == "y" ]]; then
        cli_cargo_clean_value=true
    fi
else
    while [[ $# -gt 0 ]]; do
        arg=$1
        shift

        if [[ "$arg" == "$cli_default" ]]; then
            break
        elif [[ "$arg" == "$cli_cargo_clean" ]]; then
            cli_cargo_clean_value=true
        elif [[ "$arg" == "$cli_no_cargo_clean" ]]; then
            cli_cargo_clean_value=false
        else
            echo "ERROR: unkown cli arg: $arg"
            popd
            exit
        fi
    done
fi

if [ "$cli_cargo_clean_value" = true ]; then
    echo "cleaning workspace..."
    cargo clean
fi

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
