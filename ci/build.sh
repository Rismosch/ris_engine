#!/usr/bin/env bash

echo
echo "This script generates build info and compiles the workspace as a release ready package."
echo
echo
echo

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)

source "$SCRIPT_DIR/util.sh"
pushd $ROOT_DIR

echo "checking preconditions..."

echo "clearing destination directory..."
final_dir=''
GetAndClearCiOutDir final_dir "$0"

echo "asking for user input..."
cli_cargo_clean_value=false

read -p "should \`cargo clean\` be executed before building? (y/N)" user_input
lower_user_input=$(echo $user_input | tr '[:upper:]' '[:lower:]')
if [[ $lower_user_input == "y" ]]; then
    cli_cargo_clean_value=true
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
to_replace=''
is_multi_line=false
multi_line=''
total_quotation_marks=0
total_open_paranthesis=0
total_close_paranthesis=0

function ParseMultiLine() {
    quotation_marks="${p//[^\"]}"
    total_quotation_marks=$(($total_quotation_marks + "${#quotation_marks}"))

    open_paranthesis="${p//[^\(]}"
    total_open_paranthesis=$(($total_open_paranthesis + "${#open_paranthesis}"))
    close_paranthesis="${p//[^\)]}"
    total_close_paranthesis=$(($total_close_paranthesis + "${#close_paranthesis}"))

    if [[ $total_quotation_marks -gt 0 ]] && [[ $((total_quotation_marks % 2)) -eq 0 ]] && [[ $total_open_paranthesis -gt 0 ]] && [[ $total_close_paranthesis -gt 0 ]] && [[ $total_open_paranthesis -eq $total_close_paranthesis ]]; then
        # end found! we can parse!
        multi_line+=$p
        
        declare -a quotation_mark_positions=()
        for (( i=0; i<${#multi_line}; i++ )); do
            character="${multi_line:$i:1}"
            if [[ $character == "\"" ]]; then
                quotation_mark_positions+=($i)
            fi
        done

        first_quotation_mark="${quotation_mark_positions[0]}"
        last_quotation_mark="${quotation_mark_positions[-1]}"

        string1_index=0
        string1_length=$(( first_quotation_mark + 1 ))
        string2_index=$last_quotation_mark
        string2_length=$(( "${#multi_line}" - last_quotation_mark + 1 ))

        string1=${multi_line:string1_index:string1_length}
        string2=${multi_line:string2_index:string2_length}
        echo "$string1$to_replace$string2" >> "$__temp_path"

        multi_line=''
        total_quotation_marks=0
        total_open_paranthesis=0
        total_close_paranthesis=0
        is_multi_line=false
    else
        # end not found.
        multi_line+=$p
        is_multi_line=true
    fi
}

while IFS="" read -r p || [ -n "$p" ]
do
    if [[ $p == *"$auto_generating_start"* ]]; then
        auto_generating=true
    elif [[ $p == *"$auto_generating_end"* ]]; then
        auto_generating=false
    fi

    if [ "$auto_generating" = true ]; then
        if [[ $p == *"git_repo"* ]]; then
            to_replace="$git_repo"
            ParseMultiLine
        elif [[ $p == *"git_commit"* ]]; then
            to_replace="$git_commit"
            ParseMultiLine
        elif [[ $p == *"git_branch"* ]]; then
            to_replace="$git_branch"
            ParseMultiLine
        elif [[ $p == *"rustc_version"* ]]; then
            to_replace="$rustc_version"
            ParseMultiLine
        elif [[ $p == *"rustup_toolchain"* ]]; then
            to_replace="$rustup_toolchain"
            ParseMultiLine
        elif [[ $p == *"build_date"* ]]; then
            to_replace="$build_date"
            ParseMultiLine
        elif [ "$is_multi_line" = true ]; then
            ParseMultiLine
        else
            echo "$p" >> "$__temp_path"
        fi
    else
        echo "$p" >> "$__temp_path"
    fi
done < "$build_info_path"

echo "copy \"$__temp_path\" to \"$build_info_path\""

cp -fr $__temp_path $build_info_path

echo "deleting temp file..."
if [ -f "$__temp_path" ]; then
    rm "$__temp_path"
fi

if [ "$cli_cargo_clean_value" = true ]; then
    echo "cleaning workspace..."
    cargo clean
fi

echo "setting rustflags..."
RUSTFLAGS="-C target-cpu=native"

echo "importing assets..."
cargo run -p ris_asset_compiler importall
echo "compiling assets..."
cargo run -p ris_asset_compiler compile

echo "compiling workspace..."
cargo build --release 

echo "moving files..."
target_dir="$ROOT_DIR/target/release"
source_exe_path="$target_dir/ris_engine"
asset_filename="ris_assets"
asset_path="$ROOT_DIR/$asset_filename"

cp "$source_exe_path" "$final_dir"
cp "$asset_path" "$final_dir"

echo "done! final build can be found under \`$final_dir\`"
popd

