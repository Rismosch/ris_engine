$purpose = "This script generates build info and compiles the workspace as a release ready package."

$ErrorActionPreference = "Stop"
Import-Module "$PSScriptRoot/util.ps1" -force
Push-Location $root_dir

try {
    Write-Host "checking preconditions..."
    $sdl2_dll_path = "$root_dir/SDL2.dll"
    $sdl2_dll_exists = Test-Path $sdl2_dll_path

    if (!$sdl2_dll_exists) {
        throw "could not find ``SDL2.dll`` in the root directory"
    }

    Write-Host "clearing destination directory..."
    $final_directory = GetAndClearCiOutDir

    Write-Host "parsing cli args..."
    $cli_default = "--default"

    $cli_cargo_clean = "--cargo-clean"
    $cli_no_cargo_clean = "--no-cargo-clean"
    $cli_cargo_clean_value = $false

    if ($args.length -eq 0) {
        Write-Host ""
        Write-Host $purpose
        Write-Host ""
        Write-Host "INFO: you may skip user input, by providing cli args."
        Write-Host ""
        Write-Host "available args:"
        Write-Host "    $cli_default         skips user input and uses default values for everything below"
        Write-Host ""
        Write-Host "    $cli_cargo_clean     executes ``cargo clean`` before building"
        Write-Host "    $cli_no_cargo_clean  does not execute ``cargo clean`` (default)"
        Write-Host ""
        Write-Host ""
        Write-Host ""
        Write-Host ""
        Write-Host ""

        $user_input = Read-Host "should ``cargo clean`` be executed before building? (y/N)"
        if ($user_input.ToLower() -eq "y") {
            $cli_cargo_clean_value = $true
        }
    } else {
        for($i = 0; $i -lt $args.length; ++$i) {
            $arg = $args[$i]
            switch ($arg) {
                $cli_default { break }
                $cli_cargo_clean { $cli_cargo_clean_value = $true }
                $cli_no_cargo_clean { $cli_cargo_clean_value = $false }
                default { throw "unkown cli arg: $arg" }
            }
        }
    }

    Write-Host "generating build info..."
    $build_info_path = "$PSScriptRoot/../crates/ris_data/src/info/build_info.rs"

    $git_repo = RunCommand "git config --get remote.origin.url"
    $git_commit = RunCommand "git rev-parse HEAD"
    $git_branch = RunCommand "git rev-parse --abbrev-ref HEAD"

    $rustc_version = RunCommand "rustc --version"
    $rustup_toolchain = RunCommand "rustup show active-toolchain"

    $build_date = Get-Date -Format "o"

    $auto_generating = $false
    $auto_generating_start = "@@AUTO GENERATE START@@"
    $auto_generating_end = "@@AUTO GENERATE END@@"
    $to_replace = ""
    $is_multi_line = $false
    $multi_line = ""
    $total_quotation_marks = 0
    $total_open_paranthesis = 0
    $total_close_paranthesis = 0

    $build_info_content = ""

    function ParseMultiLine {
        $quotation_marks = ([regex]::Matches($line, "`"")).count
        $script:total_quotation_marks += $quotation_marks

        $open_paranthesis = ([regex]::Matches($line, "\(")).count
        $script:total_open_paranthesis += $open_paranthesis
        $close_paranthesis = ([regex]::Matches($line, "\)")).count
        $script:total_close_paranthesis += $close_paranthesis

        if (($total_quotation_marks -gt 0) -and (($n % 2) -eq 0) -and ($total_open_paranthesis -gt 0) -and ($total_close_paranthesis -gt 0) -and ($total_open_paranthesis -eq $total_close_paranthesis)) {
            # end found! we can parse!
            $script:multi_line += "$line`n"

            $splits = $multi_line -Split "`""
            $string1 = $splits[0]
            $string2 = $splits[-1]

            $script:build_info_content += "$string1`"$to_replace`"$string2"

            $script:multi_line = ""
            $script:total_quotation_marks = 0
            $script:total_open_paranthesis = 0
            $script:total_close_paranthesis = 0
            $script:is_multi_line = $false
        } else {
            # end not found
            $script:multi_line += "$line`n"
            $script:is_multi_line=$true
        }
    }

    foreach($line in Get-Content $build_info_path) {

        if ($line -match $auto_generating_start) {
            $auto_generating = $true
        }
        elseif ($line -match $auto_generating_end) {
            $auto_generating = $false
        }

        if ($auto_generating -eq $true) {
            if ($line -match "git_repo") {
                $to_replace = $git_repo
                ParseMultiLine
            } elseif ($line -match "git_commit") {
                $to_replace = $git_commit
                ParseMultiLine
            } elseif ($line -match "git_commit") {
                $to_replace = $git_commit
                ParseMultiLine
            } elseif ($line -match "git_branch") {
                $to_replace = $git_branch
                ParseMultiLine
            } elseif ($line -match "rustc_version") {
                $to_replace = $rustc_version
                ParseMultiLine
            } elseif ($line -match "rustup_toolchain") {
                $to_replace = $rustup_toolchain
                ParseMultiLine
            } elseif ($line -match "build_date") {
                $to_replace = $build_date
                ParseMultiLine
            } elseif ($is_multi_line -eq $true) {
                ParseMultiLine
            } else {
                $build_info_content += "$line`n"
            }
        } else {
            $build_info_content += "$line`n"
        }
    }

    New-Item -Path $build_info_path -ItemType File -Value $build_info_content -Force | out-null

    if ($cli_cargo_clean_value -eq $true) {
        Write-Host "cleaning workspace..."
        cargo clean
    }
    
    Write-Host "importing assets..."
    cargo run -p ris_asset_compiler importall
    Write-Host "compiling assets..."
    cargo run -p ris_asset_compiler compile
    
    Write-Host "compiling workspace..."
    cargo build -r
    
    Write-Host "moving files..."
    
    $target_directory = Resolve-Path "$root_dir/target/release"
    $source_exe_path = Resolve-Path "$target_directory/ris_engine.exe"
    $asset_filename = "ris_assets"
    $asset_path = Resolve-Path "$root_dir/$asset_filename"
    
    Copy-Item $source_exe_path -Destination "$final_directory/ris_engine.exe"
    Copy-Item $sdl2_dll_path -Destination "$final_directory/SDL2.dll"
    Copy-Item $asset_path -Destination "$final_directory/$asset_filename"

    Write-Host "done! final build can be found under ``$final_directory``"

}
finally {
    Pop-Location
}
