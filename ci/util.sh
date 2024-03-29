#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)

ROOT_DIR="$SCRIPT_DIR/.."
CI_OUT_DIR="$ROOT_DIR/ci_out"

GetAndClearCiOutDir() {
    __caller_path="$2"
    __caller_filename=$(basename -- "$__caller_path")
    __target_name="${__caller_filename%.*}"
    __target_dir="$CI_OUT_DIR/$__target_name"

    if [ ! -d "$CI_OUT_DIR" ]; then
        mkdir "$CI_OUT_DIR"
    fi

    __destination_directory_was_logged=0

    if [ -d "$__target_dir" ]; then
        __target_dir=$(realpath $__target_dir)
        echo "destination directory is: \`$__target_dir\`"
        __destination_directory_was_logged=1

        echo
        echo "WARNING: destination directory exists already"
        read -p "are you sure you want to delete \`$__target_dir\`? (y/N)" user_input
        lower_user_input=$(echo $user_input | tr '[:upper:]' '[:lower:]')
        if [[ $lower_user_input == "y" ]]; then
            echo "deleting..."
            rm -r "$__target_dir"
            echo "deleted \`$__target_dir\`"
        fi

        echo

    fi

    if [ ! -d "$__target_dir" ]; then
        mkdir "$__target_dir"
    fi

    if [ "$__destination_directory_was_logged" -eq 0 ]; then
        __target_dir=$(realpath $__target_dir)
        echo "destination directory is: \`$__target_dir\`"
        __destination_directory_was_logged=1
    fi

    echo

    __result=$(realpath $__target_dir)
    eval "$1='$__result'"
}
