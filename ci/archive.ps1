Write-Host
Write-Host "This script is used to archive the entire workspace. The script cleans the repo, vendors dependencies, and compresses the result.`n`nTo compress the repo, 7-Zip and a WSL (Windows Subsystem for Linux) are required. If they are missing, the compression step at the very end will most likely fail."
Write-Host
Write-Host
Write-Host

$ErrorActionPreference = "Stop"
Import-Module "$PSScriptRoot/util.ps1" -force
Push-Location $root_dir

try {
    Write-Host "clearing destination directory..."
    $final_directory = GetAndClearCiOutDir

    Write-Host "asking for user input..."
    $enum_clean_none = 0
    $enum_clean_except_vendor = 1
    $enum_clean_all = 2
    $cli_clean_value = $enum_clean_none

    $cli_vendor_value = $false

    $cli_compress_value = $false

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

    $user_input = Read-Host "should be compressed? (y/N)"
    if ($user_input.ToLower() -eq "y") {
        $cli_compress_value = $true
    }

    Write-Host
    Write-Host "confirm settings:"

    if ($cli_clean_value -eq $enum_clean_none) {
        $cli_clean_display = "no"
    } elseif ($cli_clean_value -eq $enum_clean_except_vendor) {
        $cli_clean_display = "yes, except vendor"
    } elseif ($cli_clean_value -eq $enum_clean_all) {
        $cli_clean_display = "yes, all"
    }

    if ($cli_vendor_value -eq $true) {
        $cli_vendor_display = "yes"
    } else {
        $cli_vendor_display = "no"
    }

    if ($cli_compress_value -eq $true) {
        $cli_compress_display = "yes"
    } else {
        $cli_compress_display = "no"
    }

    Write-Host "  clean:    $cli_clean_display"
    Write-Host "  vendor:   $cli_vendor_display"
    Write-Host "  compress: $cli_compress_display"

    
    $user_input = Read-Host "continue with these settings? (Y/n)"
    if ($user_input.ToLower() -eq "n") {
        exit
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

    if ($cli_compress_value -eq $true) {
        Write-Host "prepare compression for zip..."
        $archive_date = Get-Date -Format "yyyy_MM_dd"
        $target_path = "$final_directory/ris_engine_$archive_date"

        Write-Host "compressing..."
        $7z = "C:\Program Files\7-Zip\7z.exe"
        RunCommand ".`"$7z`" a -t7z -m0=lzma -mx=9 -mfb=64 -md=32m -ms=on -x'!ci_out' -x'!.git' $target_path.7z *"
        RunCommand ".`"$7z`" a -tzip -mx9 -mfb=258 -mpass=15 -r -x'!ci_out' -x'!.git' $target_path.zip *"
        
        Write-Host "prepare compression for tgz..."
        
        $target_path = $target_path.Replace('\','/').Replace('C:','/mnt/c')
        $source_dir = Resolve-Path "."
        $source_dir = "$source_dir".Replace('\','/').Replace('C:','/mnt/c')

        Write-Host "compressing..."
        RunCommand "wsl tar --exclude='ci_out' --exclude='.git' -czf $target_path.tgz -C $source_dir ."

        $destination = Resolve-Path $final_directory
        Write-Host "done! compressed archives can be found under ``$destination``"
    } else {
        Write-Host "done!"
    }

}
finally {
    Pop-Location
}
