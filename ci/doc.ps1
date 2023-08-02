$ErrorActionPreference = "Stop"
Import-Module "$PSScriptRoot/util.ps1" -force

Write-Host "creating documentation..."

cargo doc

if ($LASTEXITCODE) {
    Write-Host "cargo doc succeeded!"
    Write-Host "clearing destination directory..."

    $final_directory = GetAndClearCiOutDir


} else {
    Write-Host "cargo doc was unsuccessful"
}
