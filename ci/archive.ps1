$purpose = "This script is used to archive the entire workspace."

$ErrorActionPreference = "Stop"
Import-Module "$PSScriptRoot/util.ps1" -force
Push-Location $root_dir

try {
    Write-Host "clearing destination directory..."

    $final_directory = GetAndClearCiOutDir

    Write-Host "parsing cli args..."

    $cli_default = "--default"

    $enum_clean_all = 2
    $enum_clean_except_vendor = 1
    $enum_clean_none = 0
    $cli_clean = "--clean"
    $cli_clean_except_vendor = "--clean-except-vendor"
    $cli_no_clean = "--no-clean"
    $cli_clean_value = $enum_clean_none

    $cli_vendor = "--vendor"
    $cli_no_vendor = "--no-vendor"
    $cli_vendor_value = $false

    $cli_include_git = "--include-git"
    $cli_no_include_git = "--no-include-git"
    $cli_include_git_value = $false

    if ($args.length -eq 0) {
        Write-Host ""
        Write-Host $purpose
        Write-Host ""
        Write-Host "INFO: you may skip user input, by providing cli args."
        Write-Host ""
        Write-Host "available args:"
        Write-Host "    $cli_default              skips user input and uses default values for everything below"
        Write-Host ""
        Write-Host "    $cli_clean                cleans the workspace by running a combination of `git` commands"
        Write-Host "    $cli_clean_except_vendor  cleans the workspace, but ignores `"./vendor`"` and `"./.cargo`""
        Write-Host "    $cli_no_clean             does not clean the workspace (default)"
        Write-Host ""
        Write-Host "    $cli_vendor               downloads dependencies using ``cargo vendor`` and prepares the workspace accordingly"
        Write-Host "    $cli_no_vendor            does not download dependencies (default)"
        Write-Host ""
        Write-Host "    $cli_include_git          include the ``./.git`` directory in the resulting archive"
        Write-Host "    $cli_no_include_git       does not include the ``./.git`` directory in the resulting archive (default)"
        Write-Host ""
        Write-Host ""
        Write-Host ""
        Write-Host ""
        Write-Host ""

        $user_input = Read-Host "should the workspace be cleaned? (y/N)"
        if ($user_input.ToLower() -eq "y") {
            $user_input = Read-Host "exclude vendor from clean? (Y/n)"

            if ($user_input.ToLower() -eq "n") {
                $cli_clean_value = $enum_clean_all
            } else {
                $cli_clean_value = $enum_clean_except_vendor
            }
        }

        $user_input = Read-Host "should dependencies be downloaded? (y/N)"
        if ($user_input.ToLower() -eq "y") {
            $cli_vendor_value = $true
        }

        $user_input = Read-Host "should the ``./.git`` directory be included in the resulting archive? (y/N)"
        if ($user_input.ToLower() -eq "y") {
            $cli_include_git_value = $true
        }
    } else {
        for($i = 0; $i -lt $args.length; ++$i) {
            $arg = $args[$i]
            switch ($arg) {
                $cli_default { break }
                $cli_clean { $cli_clean_value = $enum_clean_all }
                $cli_clean_except_vendor { $cli_clean_value = $enum_clean_except_vendor }
                $cli_no_clean { $cli_clean_value = $enum_clean_none }
                $cli_include_git { $cli_include_git_value = $true }
                $cli_no_include_git { $cli_include_git_value = $false }
                $cli_vendor { $cli_vendor_value = $true }
                $cli_no_vendor { $cli_vendor_value = $false }
                default { throw "unkown cli arg: $arg" }
            }
        }
    }

    if ($cli_clean_value -ne $enum_clean_none) {
        Write-Host "cleaning workspace..."

        Write-Host "git reset ."
        git reset .

        Write-Host "git checkout -- ."
        git checkout -- .

        if ($cli_clean_value -eq $enum_clean_except_vendor) {
            Write-Host "git clean -dxf -e `"vendor/`" -e `".cargo/`""
            git clean -dxf -e "vendor/" -e ".cargo/"
        } else {
            Write-Host "git clean -dxf"
            git clean -dxf
        }


        Write-Host "creating destination directory..."
        $final_directory = GetAndClearCiOutDir
    }

    if ($cli_vendor_value -eq $true) {
        Write-Host "clearing cargo config directory..."
        $cargo_config_path = ".cargo/config.toml";
        $cargo_config_directory = Split-Path -parent $cargo_config_path

        if (Test-Path $cargo_config_directory) {
            Remove-Item -Recurse -Force $cargo_config_directory
        }

        New-Item -Path $cargo_config_directory -ItemType Directory | out-null

        Write-Host "downloading dependencies..."
        $vendor_output = cargo vendor | Out-String

        Write-Host $vendor_output

        New-Item -Path $cargo_config_path -ItemType File | out-null
        Set-Content -Path $cargo_config_path -Value $vendor_output
    }

    Write-Host "find items to compress..."

    $all_items = Get-ChildItem -Path $root_dir -Name -Force
    $items_to_compress = @()

    foreach($item in $all_items) {
        if ($item -eq "ci_out") {
            continue
        }

        if (($cli_include_git_value -eq $false) -and ($item -eq ".git")) {
            continue
        }

        $items_to_compress += $item
    }

    Write-Host "prepare compression..."

    $archive_date = Get-Date -Format "yyyy_MM_dd"
    $target_path = "$final_directory/ris_engine_$archive_date.zip"

    $compress = @{
    LiteralPath= $items_to_compress
    CompressionLevel = "Optimal"
    DestinationPath = $target_path
    }

    Write-Host "compressing..."

    Compress-Archive @compress

    $destination = Resolve-Path $target_path

    Write-Host "done! final archive can be found under ``$destination``"
}
finally {
    Pop-Location
}
