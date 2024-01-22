Write-Host
Write-Host "This script generates docs and moves them to another folder. `
This prevents ``cargo clean`` to delete the docs. In case the workspace doesn't compile, having the docs available is invaluable."
Write-Host

$ErrorActionPreference = "Stop"
Import-Module "$PSScriptRoot/util.ps1" -force
Push-Location $root_dir

try {
    Write-Host "clearing destination directory..."
    $target_directory = GetAndClearCiOutDir

    Write-Host "asking for user input..."
    $cli_cargo_clean_value = $false

    $user_input = Read-Host "should ``cargo clean`` be executed before creating docs? (y/N)"
    if ($user_input.ToLower() -eq "y") {
        $cli_cargo_clean_value = $true
    }

    if ($cli_cargo_clean_value -eq $true) {
        Write-Host "cleaning workspace..."
        cargo clean
    }

    Write-Host "creating documentation..."

    cargo doc

    if ($LASTEXITCODE -eq 0) {
        Write-Host "cargo doc succeeded!"

        $source_directory = "$root_dir/target/doc"

        Write-Host "copying documentation..."
        Copy-Item -Path "$source_directory/*" -Destination $target_directory -Recurse

        Write-Host "done! final documentation can be found under ``$target_directory``"
        Write-Host "i recommend you save a bookmark to ``$target_directory/ris_engine/index.html``"
    } else {
        Write-Host "cargo doc was unsuccessful"
    }
}
finally {
    Pop-Location
}
