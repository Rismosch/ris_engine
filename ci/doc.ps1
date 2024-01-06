$purpose = "This script generates docs and moves them to another folder. `
This prevents ``cargo clean`` to delete the docs. In case the workspace doesn't compile, having the docs available is invaluable."

$ErrorActionPreference = "Stop"
Import-Module "$PSScriptRoot/util.ps1" -force
Push-Location $root_dir

try {
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
        Write-Host "    $cli_cargo_clean     executes ``cargo clean`` before creating docs"
        Write-Host "    $cli_no_cargo_clean  does not execute ``cargo clean`` (default)"
        Write-Host ""
        Write-Host ""
        Write-Host ""
        Write-Host ""
        Write-Host ""

        $user_input = Read-Host "should ``cargo clean`` be executed before creating docs? (y/N)"
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

    if ($cli_cargo_clean_value -eq $true) {
        Write-Host "cleaning workspace..."
        cargo clean
    }

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
}
finally {
    Pop-Location
}
