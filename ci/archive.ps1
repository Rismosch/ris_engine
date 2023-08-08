# This script downloads all dependencies from the internet and zips all necessary files into one package.

$ErrorActionPreference = "Stop"
Import-Module "$PSScriptRoot/util.ps1" -force

Write-Host "clearing destination directory..."

$final_directory = GetAndClearCiOutDir

Write-Host "parsing cli args..."

$cli_default = "--default"

$cli_clean = "--clean"
$cli_no_clean = "--no-clean"
$cli_clean_value = $false

$cli_vendor = "--vendor"
$cli_no_vendor = "--no-vendor"
$cli_vendor_value = $false

$cli_include_git = "--include-git"
$cli_no_include_git = "--no-include-git"
$cli_include_git_value = $false

if ($args.length -eq 0) {
    Write-Host ""
    Write-Host "INFO: you may skip user input, by providing cli args."
    Write-Host ""
    Write-Host "available args:"
    Write-Host "    $cli_default         skips user input and uses default values for everything below"
    Write-Host ""
    Write-Host "    $cli_clean           cleans the workspace by running a combination of `git` commands"
    Write-Host "    $cli_no_clean        does not clean the workspace (default)"
    Write-Host ""
    Write-Host "    $cli_vendor          downloads dependencies using ``cargo vendor`` and prepares the workspace accordingly"
    Write-Host "    $cli_no_vendor       does not download dependencies (default)"
    Write-Host ""
    Write-Host "    $cli_include_git     include the ``./.git`` directory in the resulting archive"
    Write-Host "    $cli_no_include_git  does not include the ``./.git`` directory in the resulting archive (default)"
    Write-Host ""
    Write-Host "Default values are chosen to make minimal to no changes to the workspace. I recommend calling this script with these settings:"
    Write-Host "    $cli_clean"
    Write-Host "    $cli_no_include_git"
    Write-Host "    $cli_vendor"
    Write-Host ""
    Write-Host ""
    Write-Host ""
    Write-Host ""
    Write-Host ""

    $user_input = Read-Host "should the workspace be cleaned? (y/N)"
    if ($user_input.ToLower() -eq "y") {
        $cli_clean_value = $true
    }

    $user_input = Read-Host "should dependencies be downloaded? (y/N)"
    if ($user_input.ToLower() -eq "y") {
        $cli_vendor_value = $true
    }

    $user_input = Read-Host "should the ``./.git`` directory should be included in the resulting archive? (y/N)"
    if ($user_input.ToLower() -eq "y") {
        $cli_include_git_value = $true
    }
} else {
    for($i = 0; $i -lt $args.length; ++$i) {
        $arg = $args[$i]
        switch ($arg) {
            $cli_default { break }
            $cli_clean { $cli_clean_value = $true }
            $cli_no_clean { $cli_clean_value = $false }
            $cli_include_git { $cli_include_git_value = $true }
            $cli_no_include_git { $cli_include_git_value = $false }
            $cli_vendor { $cli_vendor_value = $true }
            $cli_no_vendor { $cli_vendor_value = $false }
            default { throw "unkown cli arg: $arg" }
        }
    }
}

if ($cli_clean_value -eq $true) {
    Write-Host "cleaning workspace..."

    Write-Host "git reset ."
    git reset .

    Write-Host "git checkout -- ."
    git checkout -- .

    Write-Host "git clean -dxf"
    git clean -dxf
}

if ($cli_vendor_value -eq $true) {
    Write-Host "clearing cargo config directory..."
    if (Test-Path $cargo_config_directory) {
        Remove-Item -Recurse -Force $cargo_config_directory
    }

    Write-Host "downloading dependencies..."
    $vendor_output = cargo vendor | Out-String

    Write-Host $vendor_output

    Write-Host "preparing workspace for offline use..."
    $cargo_config_path = ".cargo/config.toml";
    $cargo_config_directory = Split-Path -parent $cargo_config_path

    New-Item -Path $cargo_config_directory -ItemType Directory | out-null
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

$target_path = "$final_directory/ris_engine.zip"

$compress = @{
LiteralPath= $items_to_compress
CompressionLevel = "Optimal"
DestinationPath = $target_path
}

Write-Host "compressing..."

Compress-Archive @compress

$destination = Resolve-Path $target_path

Write-Host "done! final archive can be found under ``$destination``"
