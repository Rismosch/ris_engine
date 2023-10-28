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

    if [ -d "$__target_dir" ]; then
        rm -r "$__target_dir"
    fi

    mkdir "$__target_dir"

    __result=$(realpath $__target_dir)
    eval "$1='$__result'"
}
