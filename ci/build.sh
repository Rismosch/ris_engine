#!/usr/bin/env bash

purpose="This script generates build info and compiles the workspace as a release ready package."

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)

source "$SCRIPT_DIR/util.sh"
pushd $ROOT_DIR

echo "checking preconditions..."
cargo check

if [ $? -ne 0 ]; then
    echo "cargo check failed"
    exit
fi

echo "clearing destination directory..."
final_dir=''
GetAndClearCiOutDir final_dir "$0"

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
    echo "    $cli_cargo_clean     executes \`cargo clean\` before building"
    echo "    $cli_no_cargo_clean  does not execute \`cargo clean\` (default)"
    echo ""
    echo ""
    echo ""
    echo ""
    echo ""

    read -p "should \`cargo clean\` be executed before building? (y/N)" user_input
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

echo "generating build info..."
build_info_path="$SCRIPT_DIR/../crates/ris_data/src/info/build_info.rs"

function RunCommand() {
    echo "running command: $2"
    
    __temp_path="$CI_OUT_DIR/temp"

    eval "$2" > $__temp_path
    __result=$(cat $__temp_path)

    eval "$1='$__result'"
}

git_repo=''
RunCommand git_repo "git config --get remote.origin.url"
git_commit=''
RunCommand git_commit "git rev-parse HEAD"
git_branch=''
RunCommand git_branch "git rev-parse --abbrev-ref HEAD"

rustc_version=''
RunCommand rustc_version "rustc --version"
rustup_toolchain=''
RunCommand rustup_toolchain "rustup show active-toolchain"

build_date=$(date --rfc-3339=ns)

__temp_path="$CI_OUT_DIR/temp"
if [ -f "$__temp_path" ]; then
    rm "$__temp_path"
fi

auto_generating=false
auto_generating_start="@@AUTO GENERATE START@@"
auto_generating_end="@@AUTO GENERATE END@@"
multi_line=false
multi_line_end_found=false;
multi_line_string=''
while IFS="" read -r p || [ -n "$p" ]
do
    if [[ $p == *"$auto_generating_start"* ]]; then
        auto_generating=true
    elif [[ $p == *"$auto_generating_end"* ]]; then
        auto_generating=false
    fi

    if [ "$auto_generating" = true ]; then
        if [[ $p == *"git_repo"* ]]; then

            echo "git_repo" >> "$__temp_path"
        elif [[ $p == *"git_commit"* ]]; then
            echo "git_commit" >> "$__temp_path"
        elif [[ $p == *"git_branch"* ]]; then
            echo "git_branch" >> "$__temp_path"
        elif [[ $p == *"rustc_version"* ]]; then
            echo "rustc_version" >> "$__temp_path"
        elif [[ $p == *"rustup_toolchain"* ]]; then
            echo "rustup_toolchain" >> "$__temp_path"
        elif [[ $p == *"build_date"* ]]; then
            echo "build_date" >> "$__temp_path"
        else
            echo "$p" >> "$__temp_path"
        fi
    else
        echo "$p" >> "$__temp_path"
    fi
done < "$build_info_path"

#echo "deleting temp file..."
#__temp_path="$CI_OUT_DIR/temp"
#if [ -f "$__temp_path" ]; then
#    rm "$__temp_path"
#fi

echo "done! final build can be found under \`$final_dir\`"

popd
