# This script generates docs and moves them to another folder.
# This prevents `cargo clean` to delete the docs. In case the workspace doesn't compile, having the docs available is invaluable.

$ErrorActionPreference = "Stop"
Import-Module "$PSScriptRoot/util.ps1" -force

Write-Host "creating documentation..."

cargo doc

if ($LASTEXITCODE -eq 0) {
    Write-Host "cargo doc succeeded!"
    Write-Host "clearing destination directory..."

    $target_directory = GetAndClearCiOutDir
    $source_directory = "$root_dir/target/doc"

    Write-Host "copying documentation..."

    Copy-Item -Path "$source_directory/*" -Destination $target_directory -Recurse

    Write-Host "opening..."

    start "$target_directory/ris_engine/index.html"

    Write-Host "done! final documentation can be found under ``$target_directory``"
} else {
    Write-Host "cargo doc was unsuccessful"
}
